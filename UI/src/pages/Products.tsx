import { ProductsResponse } from '../types';

import useFetch from '../hooks/useFetch';
import Header from '../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '@/components/Footer';

const Products = () => {
  const { data, loading, error } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = data?.products.slice(0, 5) || [];

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <div className="flex-grow pt-40 pb-28">
        <h1 className="text-center pb-6">Products</h1>

        {loading && <p>Loading...</p>}
        {error && <p>Error: {error}</p>}
        {!loading && products.length === 0 && !error ? (
          <p>No products found</p>
        ) : (
          <div className="h-[50svh] lg:h-[60svh]">
            <Carousel>
              {products.map((product) => (
                <img
                  key={product.product_id}
                  src={product.bucket_key}
                  alt={product.name}
                  loading="lazy"
                />
              ))}
            </Carousel>
          </div>
        )}
      </div>

      <Footer />
    </div>
  );
};

export default Products;
