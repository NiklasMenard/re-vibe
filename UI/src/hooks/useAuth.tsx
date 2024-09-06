import { useState, useCallback, useEffect, ReactNode, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { statusCodeMessages } from '../constants/requests';
import { createContext } from 'react';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;
const REFRESH_THRESHOLD = 300;

interface AuthProviderProps {
  children: ReactNode;
}

interface AuthContextType {
  token: string | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  loading: boolean;
  error: string | null;
  isAuthenticated: () => boolean;
  refreshAuthToken: () => Promise<string | null>;
}

const defaultAuthContext: AuthContextType = {
  token: null,
  login: async () => Promise.resolve(),
  logout: () => {},
  loading: false,
  error: null,
  isAuthenticated: () => false,
  refreshAuthToken: async () => null,
};

const deleteCookie = (name: string, path: string = '/', domain: string = 'localhost') => {
  document.cookie = `${name}=; Path=${path}; Domain=${domain}; Expires=Thu, 01 Jan 1970 00:00:01 GMT; Secure; SameSite=None;`;
};

const AuthContext = createContext<AuthContextType>(defaultAuthContext);

const AuthProvider = ({ children }: AuthProviderProps) => {
  const navigate = useNavigate();

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [token, setToken] = useState<string | null>(null);

  const login = useCallback(
    async (email: string, password: string) => {
      setLoading(true);
      setError(null);

      try {
        const response = await fetch(`${API_BASE_URL ?? ''}/auth/login`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ email, password }),
          credentials: 'include', // Ensure credentials are sent
        });

        if (!response.ok) {
          const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
          throw new Error(message);
        }

        const data = await response.json();
        if (data.success && data.access_token) {
          setToken(data.access_token);
          navigate('/products');
        } else {
          throw new Error('Login failed');
        }
      } catch (error) {
        setError(error instanceof Error ? error.message : 'An unknown error occurred');
      } finally {
        setLoading(false);
      }
    },
    [navigate]
  );

  const logout = useCallback(() => {
    deleteCookie('refresh_token', '/');
    setToken(null);
    navigate('/');
  }, [navigate]);

  const refreshAuthToken = useCallback(async () => {
    // Token is expired, check refresh token
    const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
      method: 'POST',
      credentials: 'include',
    });

    if (!token) {
      logout();
    }

    if (!response.ok) {
      const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
      logout();
      throw new Error(message);
    }

    const data = await response.json();

    if (data.success && data.access_token) {
      setToken(data.access_token);
      return data.access_token;
    } else {
      // Refresh token is expired or invalid
      logout();
      return null;
    }
  }, [logout, token]);

  const isAuthenticated = useCallback(() => {
    return !!token;
  }, [token]);

  useEffect(() => {
    const decodeToken = (token: string) => {
      const payload = JSON.parse(atob(token.split('.')[1]));
      return payload.exp;
    };

    const checkToken = () => {
      const exp = decodeToken(token ?? '');
      const now = Math.floor(Date.now() / 1000);
      const timeToRefresh = exp - now - REFRESH_THRESHOLD;

      // Token is still valid
      if (timeToRefresh > 0) {
        return;
      }

      refreshAuthToken();
    };

    if (token) {
      checkToken();
    }
  }, [token, refreshAuthToken, logout]);

  const value: AuthContextType = {
    token,
    login,
    logout,
    loading,
    error,
    isAuthenticated,
    refreshAuthToken,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }

  return context;
};

// eslint-disable-next-line react-refresh/only-export-components
export { useAuth, AuthProvider };
