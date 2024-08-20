import { useEffect, useState } from 'react';
import { Product, ProductsResponse } from '../types'; // Adjust the path as necessary
import { useAuth } from '../hooks/useAuth';
import { FlexFill } from '../styles/layouts';
import styled from 'styled-components';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

const Products = () => {
  // State to store products and any errors
  const [products, setProducts] = useState<Product[]>([]);
  const [error, setError] = useState<string | null>(null);
  const { token, refreshAuthToken } = useAuth();

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        const response = await fetch(`${API_BASE_URL ?? ''}/api/products`, {
          method: 'GET',
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        });

        if (!response.ok) {
          throw new Error('Network response was not ok');
        }

        const data: ProductsResponse = await response.json();
        setProducts(data.body.Products);
      } catch (error) {
        if (error instanceof Error) {
          setError(error.message);
        } else {
          setError('An unknown error occurred');
        }
      }
    };

    fetchProducts();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <FlexFill>
      <ProductsContainer>
        <h1>Products</h1>
        <button onClick={() => refreshAuthToken()}>Refresh</button>
        <button onClick={() => refreshAuthToken()}>Logout</button>
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
