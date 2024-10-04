import { Link } from 'react-router-dom';
import Logo from './Logo';
import { Button } from './Buttons';
import { useAuth } from '@/hooks/useAuth';

const Header = () => {
  const { isAuthenticated, logout, loading } = useAuth();

  return (
    <div
      className="fixed flex justify-between
      top-0 w-full z-50 bg-white border-b-gray-300 
      h-[4rem] px-4 outline outline-jet outline-1"
    >
      <Logo />
      {loading ? null : isAuthenticated ? (
        <Button variant="default" className="my-auto" onClick={logout}>
          Logout
        </Button>
      ) : (
        <Button asChild variant="default" className="my-auto">
          <Link to="/login">Login</Link>
        </Button>
      )}
    </div>
  );
};

export default Header;
