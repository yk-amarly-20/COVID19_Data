-- Your SQL goes here
CREATE TABLE num_infected (
  id SERIAL PRIMARY KEY,
  date timestamptz not null,
  prefecture_id integer not null,
  num_of_infected integer not null
);
