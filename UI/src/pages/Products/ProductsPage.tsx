import { useState } from 'react';
import { Product, ProductsResponse } from '../../types/types';

import Header from '../../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '../../components/Footer';
import ProductCard from './ProductCard';
import useFavoriteProducts from '@/hooks/useFavoriteProducts';
import useProducts from '@/hooks/useProducts';
import FavoriteIcon from '@/components/FavoriteIcon';

import { useAuth } from '@/hooks/useAuth';

import FavoriteProductsContainer from './FavoriteProducts';
import { dummmyProducts } from '@/constants/dummy';

const ProductsPage = () => {
  const [loadedImages, setLoadedImages] = useState(new Set());

  const { isAuthenticated } = useAuth();

  const { fetchedProducts, loading, error } = useProducts();
  const { favoriteProducts, likeProduct, unlikeProduct } = useFavoriteProducts();

  const products = import.meta.env.DEV
    ? dummmyProducts
    : fetchedProducts?.products.slice(0, 5) || [];

  const handleImageLoad = (productId: number) => {
    setLoadedImages((prev) => new Set(prev).add(productId));
  };

  const allImagesLoaded = products.length > 0 && loadedImages.size === products.length;
  const loadedAndNoErrorState = !loading && !error && fetchedProducts?.products?.length === 0;
  const isProductLiked = (product: Product): boolean =>
    favoriteProducts?.products?.some(({ product_id }) => product_id === product.product_id) ||
    false;

  const favoriteProductsResponse: ProductsResponse = {
    products:
      products.filter((product: Product) =>
        favoriteProducts?.products
          .map((favorite) => favorite.product_id)
          .includes(product.product_id)
      ) || [],
  };

  return (
    <div className="flex flex-col min-h-[100dvh] overflow-y-auto overflow-x-hidden">
      <Header />
      <Carousel renderCards={loadedAndNoErrorState} renderOverlays={!loading && allImagesLoaded}>
        {products.map((product) => (
          <ProductCard
            key={product.product_id}
            product={product}
            className={`transition-opacity duration-300 ${
              allImagesLoaded ? 'opacity-100' : 'opacity-0'
            }  max-w-[70vw]`}
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
            <img
              src={product.bucket_key}
              alt={product.name}
              width="512"
              height="512"
              onLoad={() => handleImageLoad(product.product_id)}
              className="max-w-full h-auto object-contain border-2 border-jet"
            />
          </ProductCard>
        ))}
      </Carousel>
      <FavoriteProductsContainer
        icon={(product) => (
          <FavoriteIcon
            isLiked={isProductLiked(product)}
            item={product}
            likeItem={likeProduct}
            unlikeItem={unlikeProduct}
          />
        )}
        favoriteProducts={favoriteProductsResponse}
        isAuthenticated={isAuthenticated}
      />
      <Footer />
    </div>
  );
};

export default ProductsPage;
