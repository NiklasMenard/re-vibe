import React, { useState } from 'react';

const HamburgerMenu: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => {
    setIsOpen((prev) => !prev);
  };

  return (
    <div className="block md:hidden">
      <button
        className="fixed top-4 right-4 z-50 flex flex-col gap-1.5 p-2"
        onClick={toggleMenu}
        aria-label="Toggle Menu"
      >
        <span
          className={`h-1 w-6 bg-black transition-transform ${
            isOpen ? 'rotate-45 translate-y-[0.48rem]' : ''
          }`}
        />
        <span className={`h-1 w-6 bg-black ${isOpen ? 'opacity-0' : ''}`} />
        <span
          className={`h-1 w-6 bg-black transition-transform ${
            isOpen ? '-rotate-45 -translate-y-[0.75rem]' : ''
          }`}
        />
      </button>

      <div
        className={`fixed inset-0 z-40 flex flex-col items-center justify-center bg-[#EF8453] text-[#F9F3E0] transition-transform ${
          isOpen ? 'translate-x-0' : 'translate-x-full'
        }`}
      >
        <ul className="space-y-6 text-center text-2xl">
          <li className="hover:text-black cursor-pointer">
            <a href="/Login">Login</a>
          </li>
          <li className="hover:text-black cursor-pointer">
            <a href="/products">Products</a>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default HamburgerMenu;
