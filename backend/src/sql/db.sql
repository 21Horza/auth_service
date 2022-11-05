CREATE TABLE IF NOT EXISTS users (
    id uuid NOT NULL,
    email character varying(100) NOT NULL,
    name character varying(150) NOT NULL,
    password character varying(150) NOT NULL,
    created_at timestamp without time zone DEFAULT timezone('UTC'::text, now()) NOT NULL,
    updated_at timestamp without time zone,
    role character varying(20) NOT NULL
);