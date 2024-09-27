import { useState } from 'react';
import { ProductsResponse } from '../types';
import useFetch from '../hooks/useFetch';
import Header from '../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '../components/Footer';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/Card';
import { dummmyProducts } from '@/constants/dummy';

const Products = () => {
  const { data, loading, error } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = import.meta.env.DEV ? dummmyProducts : data?.products.slice(0, 5) || [];
  const [loadedImages, setLoadedImages] = useState(new Set());

  const handleImageLoad = (productId: number) => {
    setLoadedImages((prev) => new Set(prev).add(productId));
  };

  const allImagesLoaded = products.length > 0 && loadedImages.size === products.length;
  const loadedAndNoErrorState = !loading && !error && data?.products?.length === 0;

  //TODO handle error
  return (
    <div className="flex flex-col min-h-[100dvh] overflow-y-auto overflow-x-hidden">
      <Header />
      <Carousel renderCards={loadedAndNoErrorState} renderOverlays={!loading && allImagesLoaded}>
        {products.map((product) => (
          <Card
            key={product.product_id}
            className={`rounded-[1rem] overflow-hidden border border-jet max-w-[70vw]
            transition-opacity duration-300  ${allImagesLoaded ? 'opacity-100' : 'opacity-0'}`}
          >
            <CardHeader className="p-4">
              <CardTitle>{product.name}</CardTitle>
              <CardDescription className="overflow-hidden whitespace-nowrap text-ellipsis max-w-prose">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex items-center justify-center pt-0">
              <img
                src={product.bucket_key}
                alt={product.name}
                width="512"
                height="512"
                onLoad={() => handleImageLoad(product.product_id)}
                className="max-w-full h-auto object-contain border-2 border-jet"
              />
            </CardContent>
          </Card>
        ))}
      </Carousel>
      <div className="flex flex-col justify-center bg-coral p-10 border-t border-t-jet min-h-80">
        <div className="m-auto max-w-[65ch]">
          <h2 className="font-bold mb-4">Favorited products here</h2>
        </div>
      </div>
      <Footer />
    </div>
  );
};

export default Products;
