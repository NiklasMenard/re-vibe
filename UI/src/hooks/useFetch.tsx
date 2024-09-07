import { useState, useEffect, useCallback } from 'react';
import { statusCodeMessages } from '../constants/requests';
import { useAuth } from './useAuth';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '';

interface FetchOptions {
  refresh?: boolean;
  auth?: boolean;
}

interface ApiResponse<T> {
  body: T;
}

interface UseFetchResponse<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
  fetchData: () => Promise<void>;
}

const useFetch = <T,>(url: string, options: FetchOptions = {}): UseFetchResponse<T> => {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const { token, refreshAuthToken, logout } = useAuth();

  // Default options
  const defaultOptions: FetchOptions = {
    refresh: true,
    auth: true,
  };

  // Merge default options with user-provided options
  const mergedOptions = { ...defaultOptions, ...options };

  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const headers: HeadersInit = {
        'Content-Type': 'application/json',
      };

      if (mergedOptions.auth) {
        if (!token) {
          const newToken = await refreshAuthToken();
          if (!newToken) {
            logout();
            return;
          }
          headers['Authorization'] = `Bearer ${newToken}`;
        } else {
          headers['Authorization'] = `Bearer ${token}`;
        }
      }

      const response = await fetch(`${API_BASE_URL}${url}`, {
        method: 'GET',
        headers: headers,
      });

      if (!response.ok) {
        const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
        throw new Error(message);
      }

      const result: ApiResponse<T> = await response.json();
      setData(result.body);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unknown error occurred.');
    } finally {
      setLoading(false);
    }
  }, [url, token, refreshAuthToken, logout, mergedOptions.auth]);

  useEffect(() => {
    if (mergedOptions.refresh) {
      fetchData();
    }
  }, [fetchData, mergedOptions.refresh]);

  return { data, loading, error, fetchData };
};

export default useFetch;
