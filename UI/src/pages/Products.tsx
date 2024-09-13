import styled from 'styled-components';

import { ProductsResponse } from '../types'; // Adjust the path as necessary

import { FlexColumn } from '../styles/layouts';

import useFetch from '../hooks/useFetch';
import Header from '../components/Header';

const Products = () => {
  const { data, loading, error } = useFetch<ProductsResponse>(`/api/products`, {
    refresh: true,
    auth: false,
  });

  const products = data?.products.slice(0, 5) || [];

  return (
    <FlexColumn>
      <Header />
      <ProductsContainer>
        <h1>Products</h1>

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
                <img src={product.bucket_key} alt={product.name} loading="lazy" />
                <img></img>
              </li>
            ))}
          </ul>
        )}
      </ProductsContainer>
    </FlexColumn>
  );
};

const ProductsContainer = styled.div`
  display: flex;
  flex-direction: column;
  margin: auto;
  button {
    margin-bottom: 1rem;
  }
`;

export default Products;
