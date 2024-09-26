import Logo from './Logo';

const Header = () => {
  return (
    <div
      className="fixed flex
      top-0 w-full z-50 bg-white border- b-gray-300 
      h-[4rem] px-4 outline outline-jet outline-1"
    >
      <Logo />
    </div>
  );
};

export default Header;
