import { Product, ProductsResponse } from '@/types/types';
import ProductCard from './ProductCard';

interface FavoriteProducts {
  favoriteProducts: ProductsResponse | null;
  isAuthenticated: boolean;
  icon: (product: Product) => React.ReactNode;
}

const FavoriteProductsContainer: React.FC<FavoriteProducts> = ({
  favoriteProducts,
  isAuthenticated,
  icon,
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
                <img
                  src={product.bucket_key}
                  alt={product.name}
                  width="320"
                  height="320"
                  className="max-w-full h-auto object-contain border-2 border-jet"
                />
              </ProductCard>
            ))
          : null}
      </div>
    </div>
  );
};

export default FavoriteProductsContainer;
