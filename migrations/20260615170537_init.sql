CREATE TABLE diseases (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  diseases TEXT[] NOT NULL,
  symptoms text[] NOT NULL,
  creation_time timestamptz NOT NULL
);
