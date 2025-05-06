CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE token_status AS ENUM ('pending', 'verified', 'expired');

CREATE UNLOGGED TABLE tokens (
    token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    code VARCHAR(6) NOT NULL,
    expiry TIMESTAMP NOT NULL,
    status token_status NOT NULL DEFAULT 'pending'
);
