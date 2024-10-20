import { useState } from 'react';
import { Product } from '@/types/types';

interface ProductImageProps {
  product: Product;
  width?: string;
  height?: string;
}

const ProductImage: React.FC<ProductImageProps> = ({ product, width = '512', height = '512' }) => {
  const [loaded, setLoaded] = useState(false);

  return (
    <img
      src={product.bucket_key}
      alt={product.name}
      width={width}
      height={height}
      className={`max-w-full h-auto object-contain border-2 border-jet transition-opacity duration-50 ease-in ${
        loaded ? 'opacity-100' : 'opacity-0'
      }`}
      onLoad={() => setLoaded(true)}
    />
  );
};

export default ProductImage;