CREATE TABLE habits (
  id BIGSERIAL,
  user_id BIGSERIAL,
  name VARCHAR NOT NULL,
  frequency VARCHAR CHECK (frequency in ('Daily','Weekly','Monthly')), --at what frequency is this habit tracked (daily, monthly, weekly)
  days BIGINT ARRAY, --on what days is this habit tracked --(if null all days) (1 = sunday, 2 = monday, etc)
  check_ins_per_freq BIGINT NOT NULL DEFAULT 1, --how many time per (day, week, or month) do you intend to check in
  start_date DATE NOT NULL, --when do you want to start tracking this habit?
  PRIMARY KEY(id),
  CONSTRAINT fk_users --this create a relationship between the habits and user, enforcing that the user exists when creating the reference
        FOREIGN KEY(id) 
      REFERENCES users(id),
  UNIQUE (user_id, name)
);