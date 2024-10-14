import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/Card';
import { Product } from '@/types/types';

interface ProductCardProps {
  product: Product;
  icon: React.ReactNode;
  className?: string;
  children: React.ReactNode;
}

const ProductCard: React.FC<ProductCardProps> = ({ product, icon, className, children }) => {
  return (
    <Card
      className={`rounded-[1rem] overflow-hidden border border-jet
      ${className}`}
    >
      <CardHeader className="p-4">
        <CardTitle className="flex justify-between items-center">
          {product.name} {icon}
        </CardTitle>

        <div className="flex justify-between">
          <CardDescription className=" overflow-hidden whitespace-nowrap text-ellipsis max-w-prose">
            {product.description}
          </CardDescription>
          <CardDescription className="font-bold text-sm text-jet">{`${product.price}€`}</CardDescription>
        </div>
      </CardHeader>
      <CardContent>{children}</CardContent>
    </Card>
  );
};

export default ProductCard;
