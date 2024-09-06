import { BrowserRouter } from 'react-router-dom';
import { Routes, Route } from 'react-router-dom';

import LoginPage from './pages/Login';
import { GlobalStyle } from './styles/global';
import Products from './pages/Products';
import { AuthProvider } from './hooks/useAuth';

const App = () => {
  return (
    <BrowserRouter>
      <AuthProvider>
        <GlobalStyle />
        <Routes>
          <Route path="/" element={<LoginPage />} />
          <Route path="/products" element={<Products />} />
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
};

export default App;
