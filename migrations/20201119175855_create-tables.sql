CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid default uuid_generate_v4() PRIMARY KEY,
    username VARCHAR NOT NULL unique,
    email VARCHAR NOT NULL unique,
    password_hash VARCHAR NOT NULL,
    full_name VARCHAR NULL,
    active BOOLEAN NOT NULL default true,
    code VARCHAR NULL,
    access_token VARCHAR NULL,
    created_at TIMESTAMP NOT NULL default current_timestamp,
    updated_at TIMESTAMP NOT NULL default current_timestamp
);

CREATE TABLE IF NOT EXISTS transactions
(
    id     BIGSERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL default current_timestamp,
    results jsonb NOT NULL
);
