
CREATE TABLE IF NOT EXISTS user_favorite_products (
    user_id UUID NOT NULL,
    product_id INT NOT NULL,
    added_date TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, product_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (product_id) REFERENCES products(product_id)
);