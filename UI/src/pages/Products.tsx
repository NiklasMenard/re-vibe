import { ProductsResponse } from '../types';
import useFetch from '../hooks/useFetch';
import Header from '../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '../components/Footer';
import { Skeleton } from '@/components/Skeleton';

const Products = () => {
  const { data, error, loading } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = data?.products.slice(0, 5) || [];

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <div className="flex-grow py-20">
        <h1 className="text-center pt-6">Products</h1>

        {!loading && error && <p>Error: {error}</p>}

        {!loading && !error && products.length === 0 && data && <p>No products found</p>}

        <div className="h-[50svh]">
          <Carousel overlay={!loading}>
            {loading && !error && products.length === 0
              ? [...Array(4)].map((_, index) => (
                  <Skeleton key={index} className="h-96 w-96 rounded-[1rem]" />
                ))
              : products.map((product) => (
                  <img
                    key={product.product_id}
                    src={product.bucket_key}
                    alt={product.name}
                    loading="lazy"
                  />
                ))}
          </Carousel>
        </div>
      </div>
      <Footer />
    </div>
  );
};

export default Products;
