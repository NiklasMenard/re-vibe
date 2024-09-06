import styled from 'styled-components';

import { ProductsResponse } from '../types'; // Adjust the path as necessary
import { useAuth } from '../hooks/useAuth';
import { FlexFill } from '../styles/layouts';

import useFetch from '../hooks/useFetch';

const Products = () => {
  const { logout } = useAuth();
  const { data, loading, error } = useFetch<ProductsResponse>(`/api/products`);

  const products = data?.products || [];

  return (
    <FlexFill>
      <ProductsContainer>
        <h1>Products</h1>
        <button onClick={() => logout()}>Logout</button>
        {loading && <p>Loading...</p>}
        {error && <p>Error: {error}</p>}
        {products.length === 0 ? (
          <p>No products found</p>
        ) : (
          <ul>
            {products.map((product) => (
              <li key={product.product_id}>
                <h2>{product.name}</h2>
                <p>{product.description}</p>
                <p>Price: ${product.price}</p>
                <p>Quantity: {product.quantity}</p>
                <p>Category ID: {product.category_id}</p>
                <p>Creation Date: {new Date(product.creation_date).toLocaleDateString()}</p>
              </li>
            ))}
          </ul>
        )}
      </ProductsContainer>
    </FlexFill>
  );
};

const ProductsContainer = styled.div`
  display: flex;
  justify-content: center;
  flex-direction: column;
  margin: auto;

  button {
    margin-bottom: 1rem;
  }
`;

export default Products;
