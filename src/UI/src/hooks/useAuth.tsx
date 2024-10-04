import { useState, useCallback, useEffect, ReactNode, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { statusCodeMessages } from '../constants/requests';
import { createContext } from 'react';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '';
const REFRESH_THRESHOLD = 30;

interface AuthProviderProps {
  children: ReactNode;
}

interface AuthContextType {
  token: string | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  loading: boolean;
  error: string | null;
  isAuthenticated: boolean;
  refreshAuthToken: () => Promise<string | null>;
  getUserId: () => string | null;
}

const defaultAuthContext: AuthContextType = {
  token: null,
  login: async () => Promise.resolve(),
  logout: () => {},
  loading: false,
  error: null,
  isAuthenticated: false,
  refreshAuthToken: async () => null,
  getUserId: () => null,
};

const AuthContext = createContext<AuthContextType>(defaultAuthContext);

const decodeToken = (token: string | null) => {
  if (!token) {
    return null;
  }

  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    return payload;
  } catch (error) {
    console.error('Failed to decode token:', error);
    return null;
  }
};

const AuthProvider = ({ children }: AuthProviderProps) => {
  const navigate = useNavigate();

  const [loading, setLoading] = useState(true);
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

  const logout = useCallback(async () => {
    setLoading(true);

    const response = await fetch(`${API_BASE_URL}/auth/logout`, {
      method: 'POST',
      credentials: 'include',
    });

    if (!response.ok) {
      const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
      setLoading(false);
      throw new Error(message);
    }

    setToken(null);
    setLoading(false);
    navigate('/');
  }, [navigate]);

  const refreshAuthToken = useCallback(async () => {
    try {
      setLoading(true);

      const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
        method: 'POST',
        credentials: 'include',
      });

      if (!response.ok) {
        const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
        throw new Error(message);
      }

      const data = await response.json();

      if (data.success && data.access_token) {
        setToken(data.access_token);
        return data.access_token;
      } else {
        return null;
      }
    } catch (error) {
      console.error('Error refreshing token:', error);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const isAuthenticated = !!token;

  const getUserId = useCallback(() => {
    if (!isAuthenticated) {
      return null;
    }

    return decodeToken(token).sub;
  }, [isAuthenticated, token]);

  useEffect(() => {
    if (!token) {
      refreshAuthToken();
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const checkToken = async () => {
      if (loading) {
        return;
      }

      const payload = decodeToken(token);

      if (!payload) {
        return; // Token is not valid, no further checks
      }

      const now = Math.floor(Date.now() / 1000);
      const timeToExpire = payload.exp - now;

      // Check if we are within the refresh threshold
      if (timeToExpire > REFRESH_THRESHOLD) {
        return; // Token is still valid, no need to refresh
      }

      // If the token is under the threshold, either refresh or logout
      if (timeToExpire > 0) {
        // Token is within the threshold, attempt to refresh
        await refreshAuthToken();
      } else {
        // Token is expired, log the user out
        logout();
      }
    };

    if (token) {
      checkToken();
    }
  }, [token, loading, refreshAuthToken, logout]);

  const value: AuthContextType = {
    token,
    login,
    logout,
    loading,
    error,
    isAuthenticated,
    refreshAuthToken,
    getUserId,
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
