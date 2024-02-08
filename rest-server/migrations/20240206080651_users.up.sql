CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    username CHAR(16) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar VARCHAR(256),
    password TEXT,
    points INT NOT NULL DEFAULT 0,
    coins INT NOT NULL DEFAULT 0,
    plan_type SMALLINT NOT NULL DEFAULT 0
);

CREATE INDEX user_email ON "users" (email);
CREATE INDEX user_username ON "users" (username);

CREATE TABLE IF NOT EXISTS "payments" (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES "users"(id) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    method SMALLINT NOT NULL,
    status SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS "subscriptions" (
    subscription_id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES "users"(id) NOT NULL,
    plan_type SMALLINT NOT NULL,
    payment_id INT REFERENCES "payments"(id),
    end_date TIMESTAMP NOT NULL,
    status SMALLINT NOT NULL DEFAULT 0
);

CREATE INDEX user_subscription ON "subscriptions" (user_id);