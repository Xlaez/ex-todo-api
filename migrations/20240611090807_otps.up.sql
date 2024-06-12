-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS  otps (
        email VARCHAR(50) NOT NULL UNIQUE PRIMARY KEY,
        otp VARCHAR(10) NOT NULL UNIQUE,
        created_at TIMESTAMP
         WITH
            TIME ZONE DEFAULT NOW()
    );