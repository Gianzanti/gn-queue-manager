-- migrations.sql
CREATE TABLE IF NOT EXISTS visitors (
    id              INTEGER     PRIMARY KEY NOT NULL,
    name            TEXT                    NOT NULL,
    phone           TEXT                    NOT NULL,
    email           TEXT                    NOT NULL,
    created_on      DATETIME                            DEFAULT (datetime('now','localtime')),
    updated_on      DATETIME                            DEFAULT (datetime('now','localtime')),
    lgpd            BOOLEAN                 NOT NULL    DEFAULT 0,
    image_rights    BOOLEAN                 NOT NULL    DEFAULT 0
);

