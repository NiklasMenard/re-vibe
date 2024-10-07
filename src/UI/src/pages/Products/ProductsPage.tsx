import { useState } from 'react';
import { Pagination } from '../../types/types';

import Header from '../../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '../../components/Footer';
import ProductCard from './ProductCard';
import useFavoriteProducts from '@/hooks/useFavoriteProducts';
import useProducts from '@/hooks/useProducts';
import FavoriteIcon from '@/components/FavoriteIcon';

import { useAuth } from '@/hooks/useAuth';

import FavoriteProductsContainer from './FavoriteProducts';
import ProductImage from '@/components/ProductImage';

const defaultPagination: Pagination = { page: 1, pageSize: 5 };

const ProductsPage = () => {
  const [pagination, setPagination] = useState(defaultPagination);

  const { isAuthenticated } = useAuth();

  const { fetchedProducts, loading, error, totalCount } = useProducts(pagination);

  // const products = import.meta.env.DEV ? dummmyProducts : fetchedProducts;

  const { favoriteProducts, likeProduct, unlikeProduct, isProductLiked } = useFavoriteProducts();

  const loadedAndNoErrorState = !loading && !error;

  const initialIndex = Math.floor(pagination.pageSize / 2);

  const handleNextClick = (index: number) => {
    if (fetchedProducts.length < totalCount && index === fetchedProducts.length - 2) {
      setPagination((prev) => ({ ...prev, page: prev.page + 1 }));
    }
  };

  return (
    <div className="flex flex-col min-h-[100vh] overflow-y-auto overflow-x-hidden">
      <Header />

      <Carousel
        initialIndex={initialIndex}
        onNextClick={handleNextClick}
        renderCards={loadedAndNoErrorState}
      >
        {fetchedProducts.map((product) => (
          <ProductCard
            key={product.product_id}
            product={product}
            className={`max-w-[70vw]`}
            icon={
              isAuthenticated ? (
                <FavoriteIcon
                  isLiked={isProductLiked(product)}
                  item={product}
                  likeItem={likeProduct}
                  unlikeItem={unlikeProduct}
                />
              ) : null
            }
          >
            <ProductImage product={product} />
          </ProductCard>
        ))}
      </Carousel>

      <FavoriteProductsContainer
        favoriteProducts={favoriteProducts}
        icon={(product) => (
          <FavoriteIcon
            isLiked={isProductLiked(product)}
            item={product}
            likeItem={likeProduct}
            unlikeItem={unlikeProduct}
          />
        )}
        isAuthenticated={isAuthenticated}
      />
      <Footer />
    </div>
  );
};

export default ProductsPage;
