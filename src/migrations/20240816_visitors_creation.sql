-- migrations.sql
CREATE TABLE IF NOT EXISTS visitors (
    id              serial      PRIMARY KEY NOT NULL,
    customer        INT4                    NOT NULL,
    name            TEXT                    NOT NULL,
    phone           TEXT                    NOT NULL,
    email           TEXT                    NOT NULL,
    created_at      timestamp               NOT NULL DEFAULT NOW(),
    updated_at      timestamp               NOT NULL DEFAULT NOW(),
    lgpd            BOOLEAN                 NOT NULL    DEFAULT FALSE,
    image_rights    BOOLEAN                 NOT NULL    DEFAULT FALSE
);

