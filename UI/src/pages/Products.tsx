import { ProductsResponse } from '../types';

import useFetch from '../hooks/useFetch';
import Header from '../components/Header';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from '@/components/Carousel';
import { Card, CardContent } from '@/components/Card';

const Products = () => {
  const { data, loading, error } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = data?.products.slice(0, 10) || [];

  return (
    <>
      <Header />
      <div className="flex align-middle flex-col pt-[10rem]">
        <h1 className="text-center mb-[2rem]">Products</h1>

        {loading && <p>Loading...</p>}
        {error && <p>Error: {error}</p>}
        {!loading && products.length === 0 ? (
          <p>No products found</p>
        ) : (
          <Carousel className="w-full p-[1rem] md:w-[50rem] mx-auto ">
            <CarouselPrevious />
            <CarouselContent>
              {products.map((product, i) => (
                <CarouselItem key={i}>
                  <Card>
                    <CardContent className="flex items-center justify-center">
                      <img src={product.bucket_key} alt={product.name} loading="lazy" />
                    </CardContent>
                  </Card>
                </CarouselItem>
              ))}
            </CarouselContent>

            <CarouselNext />
          </Carousel>
        )}
      </div>
    </>
  );
};

export default Products;
