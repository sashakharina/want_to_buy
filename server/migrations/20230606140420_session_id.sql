CREATE TABLE IF NOT EXISTS session (
    session_id TEXT PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES public.users(id) UNIQUE,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);