-- migrations.sql
CREATE TABLE IF NOT EXISTS visitors (
    id              INTEGER     PRIMARY KEY NOT NULL,
    customer        TEXT                 NOT NULL,
    name            TEXT                    NOT NULL,
    phone           TEXT                    NOT NULL,
    email           TEXT                    NOT NULL,
    created_at      DATETIME                            DEFAULT (datetime('now','localtime')),
    updated_at      DATETIME                            DEFAULT (datetime('now','localtime')),
    lgpd            BOOLEAN                 NOT NULL    DEFAULT 0,
    image_rights    BOOLEAN                 NOT NULL    DEFAULT 0
);

