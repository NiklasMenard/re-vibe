import { useState, useEffect, useCallback } from 'react';
import { statusCodeMessages } from '../constants/requests';
import { useAuth } from './useAuth';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '';

interface RequestOptions {
  url?: string | null;
  refresh?: boolean;
  auth?: boolean;
  method?: string;
}

interface ApiResponse<T> {
  body: T;
}

interface UseRequestResponse<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
  sendRequest: (requestUrl?: string, requestMethod?: string) => Promise<void>;
}

const useRequest = <T,>(options: RequestOptions = {}): UseRequestResponse<T> => {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const { token } = useAuth();

  // Default options
  const defaultOptions: RequestOptions = {
    refresh: false,
    auth: true,
    method: 'GET',
    url: null,
  };

  // Merge default options with user-provided options
  const mergedOptions = { ...defaultOptions, ...options };

  const sendRequest = useCallback(
    async (requestUrl?: string, requestMethod?: string) => {
      setLoading(true);
      setError(null);

      try {
        const headers: HeadersInit = {
          'Content-Type': 'application/json',
        };

        if (mergedOptions.auth) {
          headers['Authorization'] = `Bearer ${token}`;
        }

        const fetchUrl = requestUrl ? requestUrl : mergedOptions.url;

        if (!fetchUrl) {
          throw new Error('No URL provided');
        }

        const response = await fetch(`${API_BASE_URL}${fetchUrl}`, {
          method: requestMethod ? requestMethod : mergedOptions.method,
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
    },
    [mergedOptions.auth, mergedOptions.method, mergedOptions.url, token]
  );

  useEffect(() => {
    if (mergedOptions.refresh) {
      sendRequest();
    }
  }, [sendRequest, mergedOptions.refresh]);

  return { data, loading, error, sendRequest };
};

export default useRequest;
