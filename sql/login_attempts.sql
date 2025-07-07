CREATE TABLE login_attempts (
    id SERIAL PRIMARY KEY,
    ip_address INET NOT NULL,
    username TEXT,
    success BOOLEAN NOT NULL,
    attempted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
