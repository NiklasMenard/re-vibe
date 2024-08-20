import { BrowserRouter } from 'react-router-dom';
import { Routes, Route } from 'react-router-dom';

import LoginPage from './pages/Login';
import { GlobalStyle } from './styles/global';
import Products from './pages/Products';

const App = () => {
  return (
    <BrowserRouter>
      <GlobalStyle />
      <Routes>
        <Route path="/" element={<LoginPage />} />
        <Route path="/products" element={<Products />} />
      </Routes>
    </BrowserRouter>
  );
};

export default App;
