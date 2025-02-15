import { BrowserRouter } from 'react-router-dom';
import { Routes, Route } from 'react-router-dom';

import { AuthProvider } from './hooks/useAuth';

import Home from './pages/Home';
import ProductsPage from './pages/Products/ProductsPage';
import LoginPage from './pages/Login';
import ProductDetailsPage from './pages/ProductDetails/ProductDetailsPage';
import FavoriteProductsPage from './pages/FavoriteProducts/FavoriteProductsPage';
import Header from './components/Header';
import Footer from './components/Footer';

const App = () => {
  return (
    <BrowserRouter>
      <div className="flex flex-col min-h-[100dvh] overflow-auto overflow-x-hidden">
        <AuthProvider>
          <Header />
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/products" element={<ProductsPage />} />
            <Route path="/favorites" element={<FavoriteProductsPage />} />
            <Route path="/products/:id" element={<ProductDetailsPage />} />
            <Route path="/login" element={<LoginPage />} />
          </Routes>
        </AuthProvider>
        <Footer />
      </div>
    </BrowserRouter>
  );
};

export default App;
