CREATE TABLE misc (
  id uuid DEFAULT uuidv7() PRIMARY KEY,
  mood smallint NOT NULL CHECK (mood BETWEEN 1 AND 5),
  behaviours text[] NOT NULL,
  notes text NOT NULL,
  time timestamptz NOT NULL,
  tz text NOT NULL,
  date date NOT NULL GENERATED ALWAYS AS (CAST(time AT TIME ZONE tz AS date)) STORED
);

CREATE TABLE sleep (
  id UUID DEFAULT uuidv7() PRIMARY KEY,
  sleep_hours real CHECK (sleep_hours BETWEEN 0 AND 24),
  sleep_quality smallint CHECK (sleep_quality BETWEEN 1 AND 5),
  source TEXT NOT NULL CHECK (source = 'manual') DEFAULT 'manual',
  time timestamptz NOT NULL,
  tz text NOT NULL,
  date date NOT NULL GENERATED ALWAYS AS (CAST(time AT TIME ZONE tz AS date)) STORED
);
