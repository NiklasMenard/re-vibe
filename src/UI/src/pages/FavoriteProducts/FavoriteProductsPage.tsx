import React from 'react';
import useFavoriteProducts from '@/hooks/useFavoriteProducts';
import { useAuth } from '@/hooks/useAuth';
import ProductCard from '../Products/ProductCard';
import ProductImage from '@/components/ProductImage';
import FavoriteIcon from '@/components/FavoriteIcon';

const FavoriteProductsPage: React.FC = () => {
  const { isAuthenticated, loading } = useAuth();
  const { favoriteProducts, likeProduct, unlikeProduct, isProductLiked, error } =
    useFavoriteProducts();

  if (error) {
    return <div>Error loading favorite products: {error}</div>;
  }

  return (
    <>
      <h1 className="text-3xl font-bold text-jet mb-8 pt-28 text-center">
        {loading
          ? null
          : isAuthenticated
          ? `Favorite Products`
          : `Login to see your favourite products`}
      </h1>
      <div className="flex gap-10 flex-wrap flex-grow justify-center p-10">
        {isAuthenticated
          ? favoriteProducts?.products.map((product) => (
              <ProductCard
                className="min-w-[17rem] max-w-[17.5rem] max-h-[23.5rem] flex-1"
                key={product.product_id}
                product={product}
                icon={
                  <FavoriteIcon
                    isLiked={isProductLiked(product)}
                    item={product}
                    likeItem={likeProduct}
                    unlikeItem={unlikeProduct}
                  />
                }
              >
                <ProductImage product={product} width="320" height="320" />
              </ProductCard>
            ))
          : null}
      </div>
    </>
  );
};

export default FavoriteProductsPage;
