import ProductCard from './ProductCard';
import ProductImage from '@/components/ProductImage';
import { Product, ProductsResponse } from '@/types/types';
interface FavoriteProducts {
  favoriteProducts: ProductsResponse | null;
  icon: (product: Product) => React.ReactNode;
  isAuthenticated: boolean;
}

const FavoriteProductsContainer: React.FC<FavoriteProducts> = ({
  favoriteProducts,
  icon,
  isAuthenticated,
}) => {
  return (
    <div className="flex flex-col items-center justify-start bg-coral p-10 border-t border-t-jet min-h-[80vh] md:min-h-[40vh]">
      <h2 className="font-bold mb-4">
        {isAuthenticated
          ? `Here are your favorite products`
          : `Login to see your favourite products`}
      </h2>
      <div className="flex gap-10 flex-wrap ">
        {isAuthenticated
          ? favoriteProducts?.products.map((product) => (
              <ProductCard
                className="min-w-[15rem] max-w-[20rem] flex-1"
                key={product.product_id}
                product={product}
                icon={isAuthenticated ? icon(product) : null}
              >
                <ProductImage product={product} width="320" height="320" />
              </ProductCard>
            ))
          : null}
      </div>
    </div>
  );
};

export default FavoriteProductsContainer;
