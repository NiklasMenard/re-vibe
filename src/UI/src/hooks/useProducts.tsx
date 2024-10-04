import useRequest from './useRequest';
import { ProductsResponse } from '@/types/types';

const useProducts = () => {
  const { data, loading, error } = useRequest<ProductsResponse>({
    url: `/api/products`,
    refresh: true,
    auth: false,
  });

  return { fetchedProducts: data, loading, error };
};

export default useProducts;
