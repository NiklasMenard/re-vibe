import { Button } from '@/components/Buttons';
import FavoriteIcon from '@/components/FavoriteIcon';
import ProductImage from '@/components/ProductImage';
import { useAuth } from '@/hooks/useAuth';
import useFavoriteProducts from '@/hooks/useFavoriteProducts';
import useRequest from '@/hooks/useRequest';
import { ProductResponse } from '@/types/types';
import { useParams } from 'react-router-dom';

const ProductDetailsPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { isAuthenticated } = useAuth();
  const { likeProduct, unlikeProduct, isProductLiked } = useFavoriteProducts();
  const { data } = useRequest<ProductResponse>({
    url: `/api/products/${id}/`,
    auth: false,
    refresh: true,
  });

  const product = data?.product;

  if (!product) {
    return (
      <div className="flex flex-col md:p-10 mt-16 bg-coral md:bg-vanilla justify-center align-center text-center w-full h-[80vh]">
        <p>LOADING</p>
      </div>
    );
  }

  return (
    <div className="flex flex-1 flex-col md:p-10 mt-16 bg-coral md:bg-vanilla mx-auto">
      <div className="bg-white m-5 md:p-8 relative rounded-lg overflow-hidden border-black border-solid">
        {isAuthenticated && (
          <FavoriteIcon
            className="absolute top-0 right-0"
            isLiked={isProductLiked(product)}
            item={product}
            likeItem={likeProduct}
            unlikeItem={unlikeProduct}
          />
        )}
        <ProductImage product={product} className="rounded-lg  mx-auto" />

        <div className="p-5">
          <div className="flex justify-between items-center my-8">
            <p className="text-2xl font-bold text-jet">{product.name}</p>
            <p className="text-2xl font-bold text-jet">{product.price}€</p>
          </div>

          <div className="flex gap-6 my-8 flex-wrap">
            <div className="flex-[2] min-w-fit">
              <h2 className="font-semibold text-jet underline mb-1">Product Description</h2>
              <p className="text-jet mb-2">{product.description}</p>
            </div>

            <div className="flex-1 min-w-fit">
              <h2 className="font-semibold text-jet underline mb-1">Specifications</h2>
              <ul className="list-disc list-inside text-jet">
                <li>Material: 100% Cotton</li>
                <li>Dimensions: 10 x 20 x 15 cm</li>
                <li>Weight: 1.2 kg</li>
                <li>Color: Vibrant Blue</li>
              </ul>
            </div>
          </div>

          <div className="flex gap-14 my-8 flex-wrap">
            <div className="flex-[2] min-w-fit">
              <h2 className="font-semibold text-jet underline mb-1">Customer Reviews</h2>
              <p className="text-jet">"Amazing product! Exceeded my expectations." – Jane D.</p>
              <p className="text-jet">
                "Would definitely buy again. Great value for the price." – Mark T.
              </p>
            </div>

            <div className="flex-1 min-w-fit">
              <h2 className="font-semibold text-jet underline mb-1">Seller Information</h2>
              <p className="text-jet">Sold by: John's Store</p>
              <p className="text-jet">Location: San Francisco, CA, USA</p>
              <p className="text-jet">Member since: 2021</p>
            </div>
          </div>
        </div>

        <div className="flex flex-col p-2">
          <Button className="self-end">Contact Seller</Button>
        </div>
      </div>
    </div>
  );
};

export default ProductDetailsPage;
