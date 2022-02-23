CREATE TABLE todos (
  id            SERIAL      NOT NULL    PRIMARY KEY,
  content       TEXT        NOT NULL,
  is_done       BOOLEAN     NOT NULL,
  created_at    TIMESTAMP   NOT NULL
);
