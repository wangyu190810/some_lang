CREATE database spider;

CREATE  TABLE chouti(
  chouti_id serial PRIMARY KEY,
  content json NOT NULL,
  comments json NOT NULL,
)

