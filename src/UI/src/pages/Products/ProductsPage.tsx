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
import ProductImage from '@/components/ProductImage';

const DEFAULT_PAGINATION: Pagination = { page: 1, pageSize: 20 };
const MAX_DISPLAYED_PRODUCTS = 40;

enum SliceState {
  Start = 'START',
  Middle = 'MIDDLE',
  End = 'END',
}

const calculateInitialIndex = (length: number) => (length === 0 ? 2 : Math.floor(length / 2));

const ProductsPage = () => {
  const [pagination, setPagination] = useState(DEFAULT_PAGINATION);
  const [windowSlice, setWindowSlice] = useState(0);
  const [sliceState, setSliceState] = useState(SliceState.Middle);

  const { isAuthenticated } = useAuth();
  const { fetchedProducts, loading, error, totalCount } = useProducts(pagination);
  const { likeProduct, unlikeProduct, isProductLiked } = useFavoriteProducts();

  //Conditions
  const loadedAndNoErrorState = !loading && !error && fetchedProducts.length > 0;
  const sliceStart = windowSlice * MAX_DISPLAYED_PRODUCTS;
  const displayedProducts = fetchedProducts.slice(sliceStart, sliceStart + MAX_DISPLAYED_PRODUCTS);
  const initialIndex = calculateInitialIndex(displayedProducts.length);

  const handleNextClick = (index: number) => {
    if (!loadedAndNoErrorState) {
      return;
    }

    const nearEnd = index === displayedProducts.length - initialIndex;

    if (nearEnd && fetchedProducts.length < totalCount) {
      setPagination((prev) => ({ ...prev, page: prev.page + 1 }));
    }

    setSliceState(
      index + 1 === displayedProducts.length - 1 &&
        displayedProducts.length === MAX_DISPLAYED_PRODUCTS
        ? SliceState.End
        : SliceState.Middle
    );

    if (index === displayedProducts.length - 1) {
      setWindowSlice((prev) => prev + 1);
    }
  };

  const handlePrevClick = (index: number) => {
    if (!loadedAndNoErrorState) {
      return;
    }

    setSliceState(index === 1 ? SliceState.Start : SliceState.Middle);

    if (index <= 0 && windowSlice > 0) {
      setWindowSlice((prev) => prev - 1);
    }
  };

  return (
    <div className="flex flex-col min-h-[100vh] overflow-y-auto overflow-x-hidden">
      <Header />
      <Carousel
        key={sliceStart}
        initialIndex={initialIndex}
        onNextClick={handleNextClick}
        onPrevClick={handlePrevClick}
        renderCards={loadedAndNoErrorState}
        prevButtonText={
          sliceState === SliceState.Start &&
          sliceStart !== 0 && <p className="text-white">Previous</p>
        }
        nextButtonText={sliceState === SliceState.End && <p className="text-white">Load more</p>}
      >
        {displayedProducts.map((product) => (
          <ProductCard
            key={product.product_id}
            product={product}
            className="max-w-[70vw]"
            icon={
              isAuthenticated && (
                <FavoriteIcon
                  isLiked={isProductLiked(product)}
                  item={product}
                  likeItem={likeProduct}
                  unlikeItem={unlikeProduct}
                />
              )
            }
          >
            <ProductImage product={product} />
          </ProductCard>
        ))}
      </Carousel>
      <Footer />
    </div>
  );
};

export default ProductsPage;
