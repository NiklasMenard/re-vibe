import { Link } from 'react-router-dom';
import Logo from './Logo';
import { Button } from './Buttons';
import { useAuth } from '@/hooks/useAuth';
import HamburgerMenu from './HamburgerMenu';

const Header = () => {
  const { isAuthenticated, logout, loading } = useAuth();

  return (
    <div
      className="fixed flex justify-between
      top-0 w-full z-[9999] bg-white border-b-gray-300 
      h-[4rem] px-4 outline outline-jet outline-1"
    >
      <Logo />
      <div className="flex w-full px-8 items-center">
        <Link className="font-bold hover:underline ml-16 hidden md:block" to="/products">
          Products
        </Link>
        <Link className="font-bold hover:underline ml-16 hidden md:block" to="/favorites">
          Favorites
        </Link>
      </div>
      <HamburgerMenu />
      {loading ? null : isAuthenticated ? (
        <Button variant="default" className="my-auto hidden md:block" onClick={logout}>
          Logout
        </Button>
      ) : (
        <Button asChild variant="default" className="my-auto hidden md:block">
          <Link to="/login">Login</Link>
        </Button>
      )}
    </div>
  );
};

export default Header;
