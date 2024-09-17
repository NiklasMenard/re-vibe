
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
    name VARCHAR(50) NOT NULL,
    description VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    quantity INT NOT NULL,
    seller_id UUID NOT NULL,
    category_id INT NOT NULL,
    creation_date TIMESTAMP NOT NULL,
    bucket_key VARCHAR(255) NOT NULL
);


-- Create the ProductTags table (Many-to-Many Relationship)
CREATE TABLE product_tags (
    product_id INT,
    tag_id INT,
    PRIMARY KEY (product_id, tag_id)
);

-- Create the Tags table
CREATE TABLE IF NOT EXISTS tags (
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
    ('58346d65-40e1-4d88-b938-13588c0caa15', 'Albert Einstein', 'ADMIN', '$2y$12$g43/Hck5wHj0VdtgXtlc5eTkcmmIdoZJG1ar/c9.nagbEJJuI6s2G', 'i65BJem7yJl2XfviZvne7g==', 'Theoretical physicist known for the theory of relativity', 'url1'),
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
INSERT INTO products (name, description, price, quantity, seller_id, category_id, creation_date, bucket_key)
VALUES 
('Blouse', 'Chiffon blouse', 29.99, 11, '7af689f1-3f74-4586-a56c-29b913815f0b', 2, NOW(), 'fc979ab5-c417-4d2a-b5ac-3afabe54df76'),
('Blouse', 'Lace-trimmed blouse', 36.99, 7, '7af689f1-3f74-4586-a56c-29b913815f0b', 2, NOW(), '56b1af19-d5d8-474a-9e60-448bb240aa0c'),
('Blouse', 'Elegant silk blouse', 39.99, 5, '58346d65-40e1-4d88-b938-13588c0caa15', 2, NOW(), '3bd550b7-4f96-475a-9732-68733681c8db'),
('Blouse', 'Bohemian style blouse', 32.99, 8, '8d71442b-99f0-4557-af99-7d3e78aa1ea5', 2, NOW(), '189c4be5-1c48-49e3-a7d1-fff159125df8'),
('Blouse', 'Casual white blouse', 24.99, 12, '58346d65-40e1-4d88-b938-13588c0caa15', 2, NOW(), '544b4c12-1468-455c-9955-7f22ca757158'),
('Blouse', 'Striped cotton blouse', 29.99, 10, '58346d65-40e1-4d88-b938-13588c0caa15', 2, NOW(), '9e11b3ac-4055-43f1-a6c7-b64cc0b055af'),
('Blouse', 'Floral print blouse', 34.99, 7, '58346d65-40e1-4d88-b938-13588c0caa15', 2, NOW(), '3e664783-0abe-4e4a-a932-c4f9ca8537ee'),
('Blouse', 'Polka dot blouse', 27.99, 9, '58346d65-40e1-4d88-b938-13588c0caa15', 2, NOW(), '80544c92-f2c8-40f8-b43c-5cc05120475f'),
('Dress', 'Casual maxi dress', 54.99, 6, '58346d65-40e1-4d88-b938-13588c0caa15', 3, NOW(), 'cc6c24c8-76fa-454c-89ba-98de65e66c86'),
('Dress', 'Elegant evening dress', 59.99, 5, '58346d65-40e1-4d88-b938-13588c0caa15', 3, NOW(), '22e4adf7-041e-4f34-aa72-9acd3096a047'),
('Dress', 'Floral summer dress', 49.99, 8, '58346d65-40e1-4d88-b938-13588c0caa15', 3, NOW(), '595807b6-3a6c-4eca-b759-c7a7edf0a983'),
('Dress', 'Cocktail party dress', 69.99, 4, '8d71442b-99f0-4557-af99-7d3e78aa1ea5', 3, NOW(), '64d39dab-7c77-4dbf-a3f0-043n231defa9'),
('Dress', 'Formal evening gown', 89.99, 3, '8d71442b-99f0-4557-af99-7d3e78aa1ea5', 3, NOW(), 'df26554a-17a0-4512-8f04-2f799e36cc30'),
('Dress', 'Casual day dress', 44.99, 10, '8d71442b-99f0-4557-af99-7d3e78aa1ea5', 3, NOW(), '320cd6a7-e976-4b46-a82c-aeb05e69a173'),
('Dress', 'Printed summer dress', 39.99, 12, '8d71442b-99f0-4557-af99-7d3e78aa1ea5', 3, NOW(), '5d8ad5c1-abaf-4559-aa02-93c4326f9d5a'),
('Dress', 'Little black dress', 79.99, 6, '7af689f1-3f74-4586-a56c-29b913815f0b', 3, NOW(), 'd1b01c1d-444e-4b6e-83a9-09aad05a91c4');

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
