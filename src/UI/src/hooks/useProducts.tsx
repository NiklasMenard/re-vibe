import { useEffect, useState } from 'react';
import useRequest from './useRequest';
import { Pagination, PaginatedProductsResponse } from '@/types/types';

const useProducts = (pagination: Pagination) => {
  const { data, loading, error } = useRequest<PaginatedProductsResponse>({
    url: `/api/products?page=${pagination.page}&page_size=${pagination.pageSize}`,
    refresh: true,
    auth: false,
  });

  const [fetchedProducts, setFetchedProducts] = useState<
    PaginatedProductsResponse['paginatedProducts']['products']
  >([]);

  const totalCount = data?.paginatedProducts.total_count ?? 0;

  useEffect(() => {
    if (data) {
      // Append new products to the existing fetched products state
      setFetchedProducts((prevProducts) => [...prevProducts, ...data.paginatedProducts.products]);
    }
  }, [data]);

  return { fetchedProducts, loading, error, totalCount };
};

export default useProducts;
