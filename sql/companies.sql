CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    cnpj VARCHAR(18) UNIQUE,
    email VARCHAR(100),
    phone VARCHAR(20),
    address TEXT,
    plan VARCHAR(20) DEFAULT 'pro',
    subscription_expires_at TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW()
);
