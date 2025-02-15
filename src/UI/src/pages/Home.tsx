import ImageScroller from '@/components/ImageScroller';

import { Button } from '@/components/Buttons';
import { useNavigate } from 'react-router-dom';

const images_first_column = [
  'dress_102_0.jpg',
  'dress_72_0.jpg',
  'dress_92_0.jpg',
  'jacket_1_0.jpg',
  'jacket_31_0.jpg',
  'jacket_51_0.jpg',
  'jacket_71_0.jpg',
];

const images_second_column = [
  'dress_62_0.jpg',
  'dress_82_0.jpg',
  'jacket_1_0_1.jpg',
  'jacket_21_0.jpg',
  'jacket_41_0.jpg',
  'jacket_61_0.jpg',
  'jacket_81_0.jpg',
];

const Home = () => {
  const navigate = useNavigate();

  return (
    <div className="flex-grow grid grid-cols-1 grid-rows-4 pt-[4rem] md:grid-cols-2 md:grid-rows-2">
      <div className="flex flex-col justify-center  bg-coral md:bg-vanilla p-10 border-b border-b-jet md:border-none">
        <div className="m-auto max-w-[65ch] ">
          <h2 className="font-bold mb-4 ">Revive Your Wardrobe with Timeless Classics</h2>
          <p className="mb-4">
            Discover curated, pre-loved clothing that not only brings classic style to your wardrobe
            but also helps reduce waste. Join the movement for sustainable fashion and make a
            positive impact on the planet — one thrifted piece at a time.
          </p>

          <Button onClick={() => navigate('/products')} className="w-full lg:w-[25rem]">
            Explore
          </Button>
        </div>
      </div>

      <div className="flex bg-vanilla justify-center">
        <ImageScroller
          className=" absolute mt-[-8rem] right-0"
          imagePaths={images_first_column.map(
            (path) => `${import.meta.env.BASE_URL}image_gallery/${path}`
          )}
        />

        <ImageScroller
          className="absolute"
          imagePaths={images_second_column.map(
            (path) => `${import.meta.env.BASE_URL}image_gallery/${path}`
          )}
        />
      </div>

      <div className="flex flex-col justify-center bg-coral p-10 border-t border-t-jet">
        <div className="m-auto max-w-[65ch]">
          <h2 className="font-bold mb-4">
            Discover the Easiest Way to Buy and Sell Products Online
          </h2>
          <p>
            Our platform offers a seamless and secure experience for buying and selling a wide
            variety of products. With intuitive navigation and secure transactions, you can trust us
            to provide a user-friendly marketplace for all your needs.
          </p>
        </div>
      </div>
      <div className="flex flex-col items-center  justify-center bg-vanilla md:bg-coral p-4 border-t lg:border-b border-t-jet">
        <img
          src={`${import.meta.env.BASE_URL}hero-image.jpg`}
          alt="Hero image"
          className=" fullhd:max-h-[37vh]"
        />
      </div>
    </div>
  );
};

export default Home;
