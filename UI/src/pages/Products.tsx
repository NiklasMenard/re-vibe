import { useState } from 'react';
import { ProductsResponse } from '../types';
import useFetch from '../hooks/useFetch';
import Header from '../components/Header';
import Carousel from '@/components/Carousel';
import Footer from '../components/Footer';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/Card';

const Products = () => {
  const { data, error, loading } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = data?.products.slice(0, 5) || [];
  const [loadedImages, setLoadedImages] = useState(new Set());

  const handleImageLoad = (productId: number) => {
    setLoadedImages((prev) => new Set(prev).add(productId));
  };

  const allImagesLoaded = products.length > 0 && loadedImages.size === products.length;

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <div className="flex-grow py-20 pb-2">
        <h1 className="text-center pt-8">Products</h1>

        {!loading && error && <p>Error: {error}</p>}

        {!loading && !error && products.length === 0 && data && <p>No products found</p>}

        {!error && (
          <Carousel loading={loading} renderOverlays={!loading && allImagesLoaded}>
            {products.map((product) => (
              <Card
                key={product.product_id}
                className={`rounded-[1rem] overflow-hidden border border-jet 
                  transition-opacity duration-300  ${
                    allImagesLoaded ? 'opacity-100' : 'opacity-0'
                  }`}
                style={{ pointerEvents: allImagesLoaded ? 'auto' : 'none' }} // Prevent interaction until loaded
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
                    onLoad={() => handleImageLoad(product.product_id)}
                    className="max-w-full h-auto object-contain border-2 border-jet"
                  />
                </CardContent>
              </Card>
            ))}
          </Carousel>
        )}
      </div>
      <Footer />
    </div>
  );
};

export default Products;
