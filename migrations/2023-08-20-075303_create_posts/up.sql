-- Create the Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    password VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    salt VARCHAR(100) NOT NULL,
    bio TEXT,
    name VARCHAR(100),
    profile_picture_url TEXT
);

-- Create the Categories table
CREATE TABLE categories (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description TEXT
);

-- Create the Comments table
CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    post_id INT NOT NULL,
    author_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    comment_date TIMESTAMP NOT NULL
);

-- Create the Posts table
CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    publication_date TIMESTAMP NOT NULL,
    author_id UUID NOT NULL,
    category_id INT NOT NULL
);

-- Create the PostTags table (Many-to-Many Relationship)
CREATE TABLE posttags (
    post_id INT,
    tag_id INT,
    PRIMARY KEY (post_id, tag_id)
);

-- Create the Tags table
CREATE TABLE tags (
    tag_id SERIAL PRIMARY KEY,
    name VARCHAR(50)
);

-- Create foreign key constraints
ALTER TABLE comments ADD CONSTRAINT fk_comments_posts FOREIGN KEY (post_id) REFERENCES posts(post_id);
ALTER TABLE posts ADD CONSTRAINT fk_posts_users FOREIGN KEY (author_id) REFERENCES users(id);
ALTER TABLE posts ADD CONSTRAINT fk_posts_categories FOREIGN KEY (category_id) REFERENCES categories(category_id);
ALTER TABLE posttags ADD CONSTRAINT fk_posttags_posts FOREIGN KEY (post_id) REFERENCES posts(post_id);
ALTER TABLE posttags ADD CONSTRAINT fk_posttags_tags FOREIGN KEY (tag_id) REFERENCES tags(tag_id);

-- Seed Users
INSERT INTO users (id, name, email, password, salt, bio, profile_picture_url)
VALUES
    ('5fbb36b4-04f3-4a25-bd95-545f8de8b430', 'Albert Einstein', 'einstein@example.com', 'hashed_password_1', 'salt_hash', 'Theoretical physicist known for the theory of relativity', 'url1'),
    ('b4b28e67-3b94-4e14-b126-d4d0c69d59e7', 'Marie Curie', 'mariecurie@example.com', 'hashed_password_2', 'salt_hash', 'Physicist and chemist, pioneer in radioactivity research', 'url2'),
    ('1e7ce3ca-2c0b-4344-8b33-2a2e6cbe0b59', 'Charles Darwin', 'cdarwin@example.com', 'hashed_password_3', 'salt_hash', 'Naturalist, known for the theory of evolution', 'url3'),
    ('e6ab1f4d-6653-4a97-82ab-3ad6ee8c7be5', 'Isaac Newton', 'newton@example.com', 'hashed_password_4', 'salt_hash', 'Mathematician and physicist, formulated the laws of motion and universal gravitation', 'url4'),
    ('edc35096-bc18-4192-8b0e-e8fb6c2d4a3b', 'Jane Goodall', 'janegoodall@example.com', 'hashed_password_5', 'salt_hash', 'Primatologist and ethologist, studied chimpanzees in the wild', 'url5');


-- Seed Scientific Categories
INSERT INTO categories (name, description)
VALUES
    ('Physics', 'Study of matter, energy, and the fundamental forces of nature'),
    ('Chemistry', 'Study of the composition, structure, and properties of matter'),
    ('Biology', 'Study of living organisms and their interactions'),
    ('Geology', 'Study of the Earth and its processes, including rocks, minerals, and landscapes'),
    ('Astronomy', 'Study of celestial objects and the universe');

-- Seed Posts
INSERT INTO posts (title, content, publication_date, author_id, category_id)
VALUES
    ('Theory of Relativity', 'Imagination is more important than knowledge.', NOW(), '5fbb36b4-04f3-4a25-bd95-545f8de8b430', 1),
    ('Radioactivity Research', 'Nothing in life is to be feared, it is only to be understood.', NOW(), 'b4b28e67-3b94-4e14-b126-d4d0c69d59e7', 2),
    ('Theory of Evolution', 'It is not the strongest of the species that survives, nor the most intelligent that survives. It is the one that is the most adaptable to change.', NOW(), '1e7ce3ca-2c0b-4344-8b33-2a2e6cbe0b59', 3),
    ('Laws of Motion', 'If I have seen further, it is by standing on the shoulders of giants.', NOW(), 'e6ab1f4d-6653-4a97-82ab-3ad6ee8c7be5', 4),
    ('Chimpanzee Research', 'The least I can do is speak out for those who cannot speak for themselves.', NOW(), 'edc35096-bc18-4192-8b0e-e8fb6c2d4a3b', 5);
