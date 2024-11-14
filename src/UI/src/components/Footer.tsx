import { Link } from 'react-router-dom';

const Footer = () => {
  return (
    <footer className="w-full z-10 bg-viridian text-white outline outline-jet outline-1 flex flex-col items-center justify-between p-4">
      {/* Top Section */}
      <div className="w-full flex justify-around text-sm md:text-base flex-wrap">
        {/* Company Info */}
        <div className="min-w-fit flex-1 p-4">
          <h3 className="font-semibold text-2xl">About Us</h3>
          <p className="text-sm max-w-[65ch]">
            We are committed to giving pre-loved clothes a second life. Shop sustainably and
            stylishly with us!
          </p>
        </div>

        {/* Navigation Links */}
        <div className="min-w-fit flex-1 p-4">
          <h3 className="font-semibold text-2xl">Quick Links</h3>
          <ul className="space-y-1">
            <li>
              <Link className="hover:underline text-sm" to="/products">
                Products
              </Link>
            </li>
          </ul>
        </div>

        {/* Contact Info */}
        <div className="min-w-fit flex-1 p-4">
          <h3 className="font-semibold text-2xl">Get in Touch</h3>
          <ul className="space-y-1 ">
            <li className="text-sm">
              <p> email@email</p>
            </li>

            <li className="text-sm">Address: 123 Thrift Lane, Sustainable City, Earth</li>
          </ul>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
