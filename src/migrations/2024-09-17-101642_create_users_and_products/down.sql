-- Drop foreign key constraints
ALTER TABLE product_tags DROP CONSTRAINT IF EXISTS fk_product_tags_products;
ALTER TABLE product_tags DROP CONSTRAINT IF EXISTS fk_product_tags_tags;
ALTER TABLE products DROP CONSTRAINT IF EXISTS fk_products_users;
ALTER TABLE products DROP CONSTRAINT IF EXISTS fk_products_categories;

-- Drop tables
DROP TABLE IF EXISTS product_tags CASCADE;
DROP TABLE IF EXISTS products CASCADE;
DROP TABLE IF EXISTS product_categories CASCADE;
DROP TABLE IF EXISTS user_roles CASCADE;
DROP TABLE IF EXISTS roles CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS tags CASCADE;
