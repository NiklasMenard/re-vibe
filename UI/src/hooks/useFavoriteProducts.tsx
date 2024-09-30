import { useEffect } from 'react';
import useRequest from './useRequest';
import { useAuth } from './useAuth';
import { Product, ProductsResponse } from '@/types/types';

const useFavoriteProducts = () => {
  const { getUserId } = useAuth();
  const { data, sendRequest: fetchFavoriteProducts } = useRequest<ProductsResponse>({
    url: `/user/${getUserId()}/favorites`,
    auth: true,
  });

  const { sendRequest: favoriteProduct } = useRequest<ProductsResponse>({
    auth: true,
  });

  const likeProduct = async (item: Product) => {
    await favoriteProduct(`/user/${getUserId()}/favorites/${item.product_id}`, 'POST');
    fetchFavoriteProducts();
  };

  const unlikeProduct = async (item: Product) => {
    await favoriteProduct(`/user/${getUserId()}/favorites/${item.product_id}`, 'DELETE');
    fetchFavoriteProducts();
  };

  useEffect(() => {
    if (getUserId() !== null) {
      fetchFavoriteProducts();
    }
  }, [fetchFavoriteProducts, getUserId]);

  return { favoriteProducts: data, likeProduct, unlikeProduct };
};

export default useFavoriteProducts;
