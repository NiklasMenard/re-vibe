

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

export interface PaginatedProducts {
    products: Product[];
    total_count: number;
    total_pages: number;
    current_page: number;
}  

export interface PaginatedProductsResponse {
  paginatedProducts: PaginatedProducts;
  total_count: number,
  total_pages: number,
  current_page: number,
}

export type ProductsResponse = {
  products: Product[];
};


export type ProductResponse = {
  product: Product;
};

export interface Pagination {
    page: number,
    pageSize: number
}