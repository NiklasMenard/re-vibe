-- Drop foreign key constraints
ALTER TABLE user_favorite_products DROP CONSTRAINT IF EXISTS fk_products_users;
ALTER TABLE user_favorite_products DROP CONSTRAINT IF EXISTS fk_products_products;

-- Drop table
DROP TABLE IF EXISTS user_favorite_products CASCADE;
