CREATE TABLE habits (
  id SERIAL,
  user_id SERIAL,
  name VARCHAR NOT NULL,
  frequency frequency NOT NULL, --at what frequency is this habit tracked (daily, monthly, weekly)
  days day ARRAY, --on what days is this habit tracked --(if null all days)
  check_ins_per_req SMALLINT NOT NULL DEFAULT 1, --how many time per (day, week, or month) do you intend to check in
  start_date DATE NOT NULL, --when do you want to start tracking this habit?
  PRIMARY KEY(id),
  CONSTRAINT fk_users --this create a relationship between the habits and user, enforcing that the user exists when creating the reference
	      FOREIGN KEY(id) 
		  REFERENCES users(id)
);