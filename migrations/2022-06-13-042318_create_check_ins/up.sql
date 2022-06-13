CREATE TABLE check_ins (
  id SERIAL,
  habit_id SERIAL,
  date date NOT NULL,
  count SMALLINT CHECK (count > 0), -- count can't be set to negative

  PRIMARY KEY(id),
  CONSTRAINT fk_habits --this create a relationship between the habits and user, enforcing that the user exists when creating the reference
	      FOREIGN KEY(id) 
		  REFERENCES habits(id)
);