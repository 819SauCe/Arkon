CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    company_id INT REFERENCES companies(id),
    username VARCHAR(20) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    full_name VARCHAR(100),
    img_url TEXT,
    cpf VARCHAR(14) UNIQUE,
    cnpj VARCHAR(18) UNIQUE,
    phone VARCHAR(20),
    birth_date DATE,
    address TEXT,
    last_login TIMESTAMP,
    created_by VARCHAR(20),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
