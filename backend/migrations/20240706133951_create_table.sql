CREATE TABLE passwords (
	id uuid NOT NULL,
	PRIMARY KEY (id),
	name TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL
);