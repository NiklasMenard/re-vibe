-- Drop foreign key constraints
ALTER TABLE product_tags DROP CONSTRAINT IF EXISTS fk_product_tags_products;
ALTER TABLE product_tags DROP CONSTRAINT IF EXISTS fk_product_tags_tags;
ALTER TABLE products DROP CONSTRAINT IF EXISTS fk_products_users;
ALTER TABLE products DROP CONSTRAINT IF EXISTS fk_products_categories;

-- Drop tables
DROP TABLE IF EXISTS product_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS product_categories;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS users;
