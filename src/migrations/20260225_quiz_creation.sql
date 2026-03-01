CREATE TABLE IF NOT EXISTS quiz (
    "id"                serial      PRIMARY KEY NOT NULL,
    "title"             TEXT                    NOT NULL,
    "created_at"        timestamp               NOT NULL DEFAULT NOW(),
    "updated_at"        timestamp               NOT NULL DEFAULT NOW(),
    "is_active"         BOOLEAN                 NOT NULL    DEFAULT FALSE,
    "is_deleted"        BOOLEAN                 NOT NULL    DEFAULT FALSE,
    "back_img"          TEXT                    NOT NULL,
    "has_matching"      BOOLEAN                 NOT NULL    DEFAULT FALSE,
    "matches"           jsonb                   NOT NULL    DEFAULT '{}'::jsonb,
    "match_threshold"   INT4                    NOT NULL    DEFAULT 0
);

CREATE TABLE IF NOT EXISTS quiz_question (
    "id"                serial      PRIMARY KEY NOT NULL,
    "quiz_id"         INT4                    NOT NULL,
    "question"        TEXT                    NOT NULL,
    "created_at"      timestamp               NOT NULL DEFAULT NOW(),
    "updated_at"      timestamp               NOT NULL DEFAULT NOW(),
    "is_deleted"      BOOLEAN                 NOT NULL    DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS quiz_alternative (
    "id"                serial      PRIMARY KEY NOT NULL,
    "question_id"       INT4                    NOT NULL,
    "alternative"       TEXT                    NOT NULL,
    "weights"           jsonb                   NOT NULL    DEFAULT '{}'::jsonb,
    "created_at"        timestamp               NOT NULL DEFAULT NOW(),
    "updated_at"        timestamp               NOT NULL DEFAULT NOW(),
    "is_deleted"        BOOLEAN                 NOT NULL    DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS visitor (
    "id"                serial      PRIMARY KEY NOT NULL,
    "fullname"          TEXT                    NOT NULL,
    "phone"             TEXT                    NOT NULL,
    "email"             TEXT                    NOT NULL,
    "city"              TEXT                    NOT NULL,
    "state"             TEXT                    NOT NULL,
    "created_at"        timestamp               NOT NULL DEFAULT NOW(),
    "updated_at"        timestamp               NOT NULL DEFAULT NOW(),
    "lgpd"              BOOLEAN                 NOT NULL    DEFAULT FALSE,
    "image_rights"      BOOLEAN                 NOT NULL    DEFAULT FALSE,
    "is_deleted"        BOOLEAN                 NOT NULL    DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS quiz_response (
    "id"                serial      PRIMARY KEY NOT NULL,
    "visitor_id"        INT4                    NOT NULL,
    "quiz_id"           INT4                    NOT NULL,
    "responses"         jsonb                   NOT NULL    DEFAULT '{}'::jsonb,
    "result"            jsonb                   NOT NULL    DEFAULT '{}'::jsonb,
    "created_at"        timestamp               NOT NULL DEFAULT NOW(),
    "updated_at"        timestamp               NOT NULL DEFAULT NOW(),
    "is_deleted"        BOOLEAN                 NOT NULL    DEFAULT FALSE
);