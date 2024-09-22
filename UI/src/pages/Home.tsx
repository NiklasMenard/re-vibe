import { useNavigate } from 'react-router-dom';
import Header from '../components/Header';
import Footer from '../components/Footer';
import { Button } from '@/components/Buttons';

const Home = () => {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <div className="flex-grow grid grid-cols-1 grid-rows-4  py-28 md:grid-cols-2 md:grid-rows-2">
        <div className="flex flex-col justify-center  bg-vanilla h-full px-10 outline outline-jet outline-1">
          <h1 className="text-3xl font-bold mb-4">Medium length hero heading goes here</h1>
          <p className="text-lg mb-4">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse varius enim in eros
            elementum tristique. Duis cursus, mi quis viverra ornare, eros dolor interdum nulla, ut
            commodo diam libero vitae erat.
          </p>
          <Button onClick={() => navigate('/products')} className="w-full lg:w-[25rem]">
            Explore
          </Button>
        </div>
        <div className="bg-gray-200 min-h-[15rem] h-full outline outline-jet outline-1">
          {/* Will add image here */}
          <p>pictures</p>
        </div>

        <div className="flex flex-col justify-center  bg-coral border-b h-full px-10 outline outline-jet outline-1">
          <h2 className="text-2xl font-semibold mb-4">
            Discover the Easiest Way to Buy and Sell Products Online
          </h2>
          <p className="text-lg">
            Our platform offers a seamless and secure experience for buying and selling a wide
            variety of products. With intuitive navigation and secure transactions, you can trust us
            to provide a user-friendly marketplace for all your needs.
          </p>
        </div>
        <div className="bg-gray-200 min-h-[15rem] h-full  outline outline-jet outline-1">
          {/* Will add image here */}
          <p>pictures</p>
        </div>
      </div>
      <Footer />
    </div>
  );
};

export default Home;
