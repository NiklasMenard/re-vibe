

export type Product = {
    product_id: number;
    name: string;
    description: string;
    price: string; 
    quantity: number;
    seller_id: string;
    category_id: number;
    creation_date: string; 
    bucket_key: string
  };
  

  export type ProductsResponse = {
    products: Product[];
  };