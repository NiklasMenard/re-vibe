
-- Create the Users table (if not already created)
CREATE TABLE users (
    id UUID PRIMARY KEY,
    password VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    salt VARCHAR(100) NOT NULL,
    bio TEXT,
    name VARCHAR(100),
    profile_picture_url TEXT
);

-- Create the Roles table (if not already created)
CREATE TABLE roles (
    role_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Create the User_Roles table (if not already created)
CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id INT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(role_id)
);

-- Create the Categories table for products
CREATE TABLE product_categories (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description TEXT
);

-- Create the Products table
CREATE TABLE products (
    product_id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    quantity INT NOT NULL,
    seller_id UUID NOT NULL,
    category_id INT NOT NULL,
    creation_date TIMESTAMP NOT NULL
);

-- Create the ProductTags table (Many-to-Many Relationship)
CREATE TABLE product_tags (
    product_id INT,
    tag_id INT,
    PRIMARY KEY (product_id, tag_id)
);

-- Create the Tags table
CREATE TABLE tags (
    tag_id SERIAL PRIMARY KEY,
    name VARCHAR(50)
);

-- Create foreign key constraints
ALTER TABLE products ADD CONSTRAINT fk_products_users FOREIGN KEY (seller_id) REFERENCES users(id);
ALTER TABLE products ADD CONSTRAINT fk_products_categories FOREIGN KEY (category_id) REFERENCES product_categories(category_id);
ALTER TABLE product_tags ADD CONSTRAINT fk_product_tags_products FOREIGN KEY (product_id) REFERENCES products(product_id);
ALTER TABLE product_tags ADD CONSTRAINT fk_product_tags_tags FOREIGN KEY (tag_id) REFERENCES tags(tag_id);

-- Seed Users
INSERT INTO users (id, name, email, password, salt, bio, profile_picture_url)
VALUES
    ('58346d65-40e1-4d88-b938-13588c0caa15', 'Albert Einstein', 'ADMIN', '$2y$12$xSbDr.K5c96ZcLMpENf5mO9j7bqua5YqtXYj/pBFuwao45KGByelu', 'zUdFtAM7e/8beNOrGPh7oQ==', 'Theoretical physicist known for the theory of relativity', 'url1'),
    ('5cf522b0-ac95-4526-a0c3-3163d38115f0', 'Marie Curie', 'mariecurie@example.com', 'hashed_password_2', 'salt_hash', 'Physicist and chemist, pioneer in radioactivity research', 'url2'),
    ('ef7903d1-ec4b-4264-bcbd-f46524d601d6', 'Charles Darwin', 'cdarwin@example.com', 'hashed_password_3', 'salt_hash', 'Naturalist, known for the theory of evolution', 'url3'),
    ('8d71442b-99f0-4557-af99-7d3e78aa1ea5', 'Isaac Newton', 'newton@example.com', 'hashed_password_4', 'salt_hash', 'Mathematician and physicist, formulated the laws of motion and universal gravitation', 'url4'),
    ('7af689f1-3f74-4586-a56c-29b913815f0b', 'Jane Goodall', 'janegoodall@example.com', 'hashed_password_5', 'salt_hash', 'Primatologist and ethologist, studied chimpanzees in the wild', 'url5');

 
-- Seed Roles (if not already seeded)
INSERT INTO roles (name) VALUES
    ('user'),
    ('seller'),
    ('admin');

-- Assign 'admin' role to user with ID '58346d65-40e1-4d88-b938-13588c0caa15' (if not already assigned)
INSERT INTO user_roles (user_id, role_id) VALUES
    ('58346d65-40e1-4d88-b938-13588c0caa15', 3);

-- Seed Product Categories
INSERT INTO product_categories (name, description)
VALUES
    ('Electronics', 'Electronic devices and accessories'),
    ('Clothing', 'Apparel and fashion items'),
    ('Books', 'Literature and printed material'),
    ('Home Goods', 'Household items and furnishings');

-- Seed Products
INSERT INTO products (name, description, price, quantity, seller_id, category_id, creation_date)
VALUES
    ('Smartphone', 'High-performance smartphone with advanced features', 599.99, 10, '58346d65-40e1-4d88-b938-13588c0caa15', 1, NOW()),
    ('Designer Shirt', 'Fashionable designer shirt for any occasion', 49.99, 20, '7af689f1-3f74-4586-a56c-29b913815f0b', 2, NOW()),
    ('Programming Book', 'Comprehensive guide to programming', 29.99, 15, 'ef7903d1-ec4b-4264-bcbd-f46524d601d6', 3, NOW()),
    -- Add more product data as needed
    ('Coffee Maker', 'Automatic coffee maker for home use', 79.99, 5, 'ef7903d1-ec4b-4264-bcbd-f46524d601d6', 4, NOW());

-- Seed Tags (if not already seeded)
INSERT INTO tags (name) VALUES
    ('Electronics'),
    ('Fashion'),
    ('Programming'),
    ('Home Appliances');

-- Seed Product Tags (if not already seeded)
INSERT INTO product_tags (product_id, tag_id) VALUES
    (1, 1), -- Smartphone tagged as Electronics
    (2, 2), -- Designer Shirt tagged as Fashion
    (3, 3), -- Programming Book tagged as Programming
    (4, 4); -- Coffee Maker tagged as Home Appliances
