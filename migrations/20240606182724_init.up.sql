-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS  users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        username VARCHAR(255) NOT NULL,
        email VARCHAR(50) NOT NULL UNIQUE,
        img VARCHAR(250),
        password TEXT NOT NULL,
        email_verified BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
         WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );