CREATE TABLE categories
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE tags
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE pets
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR      NOT NULL,
    category_id INTEGER      NOT NULL,
    status      VARCHAR(100) NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories (id)
);

CREATE TABLE pet_tag
(
    pet_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (pet_id, tag_id),
    FOREIGN KEY (pet_id) REFERENCES pets (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);