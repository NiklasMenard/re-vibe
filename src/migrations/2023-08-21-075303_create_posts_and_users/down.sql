-- Drop foreign key constraints
ALTER TABLE posttags DROP CONSTRAINT IF EXISTS fk_posttags_tags;
ALTER TABLE posttags DROP CONSTRAINT IF EXISTS fk_posttags_posts;
ALTER TABLE posts DROP CONSTRAINT IF EXISTS fk_posts_categories;
ALTER TABLE posts DROP CONSTRAINT IF EXISTS fk_posts_users;
ALTER TABLE comments DROP CONSTRAINT IF EXISTS fk_comments_posts;

-- Delete data from tables
DELETE FROM posttags;
DELETE FROM tags;
DELETE FROM posts;
DELETE FROM comments;
DELETE FROM user_roles;
DELETE FROM roles;
DELETE FROM categories;
DELETE FROM users;

-- Drop tables
DROP TABLE IF EXISTS posttags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS users;
