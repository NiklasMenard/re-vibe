-- Drop foreign key constraints
ALTER TABLE posttags DROP CONSTRAINT fk_posttags_tags;
ALTER TABLE posttags DROP CONSTRAINT fk_posttags_posts;
ALTER TABLE posts DROP CONSTRAINT fk_posts_categories;
ALTER TABLE posts DROP CONSTRAINT fk_posts_users;
ALTER TABLE comments DROP CONSTRAINT fk_comments_posts;

-- Drop tables in reverse order
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS posttags;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS users;