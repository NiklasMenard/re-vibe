import { BrowserRouter } from 'react-router-dom';
import { Routes, Route } from 'react-router-dom';

import { GlobalStyle, Theme } from './styles/global';
import Products from './pages/Products';
import { AuthProvider } from './hooks/useAuth';
import Home from './pages/Home';

const App = () => {
  return (
    <BrowserRouter>
      <AuthProvider>
        <Theme>
          <GlobalStyle />
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/products" element={<Products />} />
          </Routes>
        </Theme>
      </AuthProvider>
    </BrowserRouter>
  );
};

export default App;
