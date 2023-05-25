CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    nickname VARCHAR(50) NOT NULL CHECK (LENGTH(nickname) >= 1),
    email VARCHAR(256) NOT NULL UNIQUE,
    phone_number VARCHAR(20) UNIQUE,
    password TEXT NOT NULL,
    photo TEXT,
    date_of_birth DATE,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);

CREATE TYPE list_access_level as ENUM('private', 'public', 'by_link');
CREATE  TABLE IF NOT EXISTS lists (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES public.users(id),
    name VARCHAR(40) NOT NULL CHECK (LENGTH(name) >= 1),
    access_level list_access_level NOT NULL DEFAULT 'private',
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);

CREATE TABLE IF NOT EXISTS currencies (
    code CHAR(3) PRIMARY KEY,
    description VARCHAR(60)
);

CREATE TABLE IF NOT EXISTS purchases (
    id BIGSERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL REFERENCES public.lists(id),
    name VARCHAR(40) NOT NULL CHECK (LENGTH(name) >= 1),
    price_amount BIGINT,
    price_currency CHAR(3) REFERENCES public.currencies(code),
    priority SMALLINT NOT NULL DEFAULT 0 CHECK(priority >= 0 AND priority <=7),
    description VARCHAR(100),
    link TEXT,
    photo TEXT,
    is_done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);

CREATE TABLE IF NOT EXISTS contacts (
    subscriber_id BIGINT NOT NULL,
    subscribing_id BIGINT NOT NULL CHECK(subscribing_id != subscriber_id),
    is_accept BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(subscriber_id,subscribing_id),
    FOREIGN KEY (subscriber_id) REFERENCES public.users(id),
    FOREIGN KEY (subscribing_id) REFERENCES public.users(id),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);