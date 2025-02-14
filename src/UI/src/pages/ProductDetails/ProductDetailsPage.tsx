import { Button } from '@/components/Buttons';
import FavoriteIcon from '@/components/FavoriteIcon';
import Footer from '@/components/Footer';
import Header from '@/components/Header';
import { useAuth } from '@/hooks/useAuth';
import useFavoriteProducts from '@/hooks/useFavoriteProducts';
import useRequest from '@/hooks/useRequest';
import { ProductResponse } from '@/types/types';
import React from 'react';
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

  if (!data) {
    return <div>Loading...</div>;
  }

  const { product } = data;
  return (
    <div className="flex flex-col ">
      <Header />
      <div className="flex flex-col items-stretch">
        <div className="flex flex-1 flex-col md:p-10 justify-between mt-16 bg-coral md:bg-vanilla mx-auto ">
          <div className="p-10 bg-coral md:bg-white border md:rounded-lg">
            <div className="bg-white p-8 relative rounded-lg">
              {isAuthenticated && (
                <FavoriteIcon
                  className="absolute top-0 right-0"
                  isLiked={isProductLiked(product)}
                  item={product}
                  likeItem={likeProduct}
                  unlikeItem={unlikeProduct}
                />
              )}

              <img
                src={product?.bucket_key}
                alt={product?.name}
                width={768}
                className="w-full max-w-[768px] h-auto mx-auto object-contain rounded-lg border border-black border-solid"
              />

              <div className="flex justify-between items-center my-8">
                <p className="text-2xl font-bold text-jet">{product?.name}</p>
                <p className="text-2xl font-bold text-jet">{product?.price}€</p>
              </div>

              <div className="flex gap-6 my-8 flex-wrap">
                <div className="flex-[2] min-w-fit">
                  <h2 className="text-xl font-semibold text-jet underline">Product Description</h2>
                  <p className="text-jet mb-2">{product?.description}</p>
                </div>

                <div className="flex-1 min-w-fit">
                  <h2 className="text-xl font-semibold text-jet underline">Specifications</h2>
                  <ul className="list-disc list-inside text-gray-600">
                    <li>Material: 100% Cotton</li>
                    <li>Dimensions: 10 x 20 x 15 cm</li>
                    <li>Weight: 1.2 kg</li>
                    <li>Color: Vibrant Blue</li>
                  </ul>
                </div>
              </div>

              <div className="flex gap-14 my-8 flex-wrap">
                <div className="flex-[2] min-w-fit">
                  <h2 className="text-xl font-semibold text-jet underline">Customer Reviews</h2>
                  <p className="text-gray-600">"Amazing product! Exceeded my expectations." – Jane D.</p>
                  <p className="text-gray-600">
                    "Would definitely buy again. Great value for the price." – Mark T.
                  </p>
                </div>

                <div className="flex-1 min-w-fit">
                  <h2 className="text-xl font-semibold text-jet underline">Seller Information</h2>
                  <p className="text-gray-600">
                    Sold by: <span className="font-medium">John's Store</span>
                  </p>
                  <p className="text-gray-600">Location: San Francisco, CA, USA</p>
                  <p className="text-gray-600">Member since: 2021</p>
                </div>
              </div>

              <div className="flex flex-col gap-6">
                <Button className="self-end">Contact Seller</Button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <Footer />
    </div>
  );
};

export default ProductDetailsPage;
