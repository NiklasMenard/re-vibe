-- Add the new column to the products table
ALTER TABLE products
ADD COLUMN bucket_key VARCHAR(255) NOT NULL DEFAULT 'default_bucket_key';


-- Insert data into the products table with bucket key
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


ALTER TABLE product_tags DROP CONSTRAINT fk_product_tags_products;

ALTER TABLE product_tags ADD CONSTRAINT fk_product_tags_products
FOREIGN KEY (product_id) REFERENCES products(product_id)
ON DELETE CASCADE;

-- Delete previously seeded products
DELETE FROM products
WHERE name IN ('Smartphone', 'Designer Shirt', 'Programming Book', 'Coffee Maker');
