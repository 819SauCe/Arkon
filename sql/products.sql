-- Tabela principal de produtos
CREATE TABLE create_product (
    id SERIAL PRIMARY KEY,
    webstore_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    old_price INTEGER NOT NULL,
    price INTEGER NOT NULL,
    currency VARCHAR(10) NOT NULL,
    show_old_price BOOLEAN NOT NULL,
    shipping BOOLEAN NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Tabela de imagens do produto
CREATE TABLE product_img (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    image TEXT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES create_product(id) ON DELETE CASCADE
);

-- Tabela de criação de produto por usuário
CREATE TABLE product_create_for (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    user_name TEXT NOT NULL,
    creat_at TIMESTAMP NOT NULL,
    FOREIGN KEY (product_id) REFERENCES create_product(id) ON DELETE CASCADE
);
