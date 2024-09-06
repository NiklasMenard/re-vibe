import { useState, useEffect, useCallback } from 'react';
import { statusCodeMessages } from '../constants/requests';
import { useAuth } from './useAuth';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

interface FetchOptions {
  refresh?: boolean;
}

interface ApiResponse<T> {
  body: T;
}

interface UseFetchResponse<T> {
  data: T | null;
  loading: boolean;
  error: string | undefined;
  fetchData: () => Promise<void>;
}

const useFetch = <T,>(
  url: string,
  options: FetchOptions = { refresh: true }
): UseFetchResponse<T> => {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const { token, refreshAuthToken } = useAuth();

  const getData = useCallback(async () => {
    let accessToken = token;
    // If token is not set, refresh it
    if (!accessToken) {
      accessToken = await refreshAuthToken();
    }

    // Proceed to fetch data with the token
    const response = await fetch(`${API_BASE_URL ?? ''}${url}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${accessToken}`, // Use the refreshed token
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      const message = statusCodeMessages[response.status] || 'An unknown error occurred.';
      throw new Error(message);
    }

    const result: ApiResponse<T> = await response.json();
    setData(result.body);
  }, [refreshAuthToken, token, url]);

  const fetchData = async () => {
    setLoading(true);
    setError(null);

    try {
      await getData();
    } catch (error) {
      if (error instanceof Error) {
        setError(error);
      } else {
        setError(new Error('An unknown error occurred'));
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (options.refresh) {
      fetchData();
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { data, loading, error: error?.message, fetchData };
};

export default useFetch;
