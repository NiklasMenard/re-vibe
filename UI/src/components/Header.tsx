import Logo from './Logo';

const Header = () => {
  return (
    <div className="fixed flex top-0 w-full z-1 bg-white border-b-gray-300 h-[7rem] px-8">
      <Logo />
    </div>
  );
};

export default Header;
