CREATE TABLE webstores (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    url VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    image VARCHAR(255),
    category VARCHAR(100),
    creator_name VARCHAR(100),
    theme VARCHAR(100),
    creator_id INTEGER REFERENCES users(id),
    companies INTEGER REFERENCES companies(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    user_accept_terms BOOLEAN
);
