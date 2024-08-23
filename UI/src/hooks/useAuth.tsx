import { jwtDecode } from 'jwt-decode';
import { useState, useCallback, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { statusCodeMessages } from '../constants/requests';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;
const REFRESH_THRESHOLD = 60;

const TOKEN_KEY = 'jwt_token';
type LoginFunction = (username: string, password: string) => Promise<void>;

export function useAuth() {
  const [token, setToken] = useState<string | null>(() => {
    return localStorage.getItem(TOKEN_KEY);
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const navigate = useNavigate();

  const login: LoginFunction = useCallback(
    async (email, password) => {
      setLoading(true);
      setError(null);

      try {
        const response = await fetch(`${API_BASE_URL ?? ''}/auth/login`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ email, password }),
        });

        if (!response.ok) {
          const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
          throw new Error(message);
        }

        const data = await response.json();

        setToken(data.token);
        localStorage.setItem(TOKEN_KEY, data.token);

        navigate('/products');
      } catch (error) {
        setError(error instanceof Error ? error.message : 'An unknown error occurred');
      } finally {
        setLoading(false);
      }
    },
    [navigate]
  );

  const logout = useCallback(() => {
    setToken(null);
    localStorage.removeItem(TOKEN_KEY);

    navigate('/');
  }, [navigate]);

  // Function to handle refreshing the token using the refresh token from localstorage
  const refreshAuthToken = useCallback(async () => {
    const token = localStorage.getItem(TOKEN_KEY);
    if (!token) {
      logout();
      return null;
    }

    try {
      const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error('Failed to refresh token');
      }

      const data = await response.json();
      setToken(data.token);

      return data.token;
    } catch (error) {
      console.error('Error refreshing token:', error);
      logout(); // If the refresh fails, log out the user
      return null;
    }
  }, [logout]);

  const isAuthenticated = useCallback(() => {
    return !!token;
  }, [token]);

  useEffect(() => {
    if (token) {
      const decoded = jwtDecode<{ exp: number }>(token);
      const currentTime = Date.now() / 1000;

      // Check if the token is about to expire within the threshold
      if (decoded.exp < currentTime + REFRESH_THRESHOLD) {
        refreshAuthToken();
      }
    } else {
      logout();
    }
  }, [token, refreshAuthToken, navigate, logout]);

  return {
    token,
    login,
    logout,
    isAuthenticated,
    loading,
    refreshAuthToken,
    error,
  };
}
