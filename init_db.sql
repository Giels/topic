CREATE TABLE boards (
	uid SERIAL UNIQUE,
	link VARCHAR(15) UNIQUE,
	name VARCHAR(255),
	notice TEXT,
	rules TEXT[]
);

CREATE TABLE threads (
	uid SERIAL UNIQUE,
	bid SERIAL REFERENCES boards (uid),
	title VARCHAR(255),
	bdate TIMESTAMP NOT NULL DEFAULT current_timestamp,
	sticky BOOLEAN NOT NULL DEFAULT(FALSE)
);

CREATE TABLE posts (
	uid SERIAL UNIQUE,
	tid SERIAL REFERENCES threads (uid),
	number INTEGER NOT NULL,
	name VARCHAR(255) NOT NULL DEFAULT 'Anonymous',
	content TEXT,
	password VARCHAR(255),
	bump TIMESTAMP NOT NULL DEFAULT current_timestamp,
	ip VARCHAR(15) NOT NULL,
	cdate TIMESTAMP NOT NULL DEFAULT current_timestamp,
	deleted BOOLEAN NOT NULL DEFAULT(FALSE)
);

CREATE TABLE categories (
	uid SERIAL UNIQUE,
	name VARCHAR(255),
	bid SERIAL REFERENCES boards (uid)
);

CREATE TABLE mods (
	uname VARCHAR(255) NOT NULL,
	pass VARCHAR(255) NOT NULL,
	can_delete BOOLEAN NOT NULL DEFAULT(FALSE),
	can_edit BOOLEAN NOT NULL DEFAULT(FALSE),
	can_ban BOOLEAN NOT NULL DEFAULT(FALSE),
	can_sticky BOOLEAN NOT NULL DEFAULT(FALSE)
);

CREATE TABLE reports (
	ip VARCHAR(15) NOT NULL,
	post SERIAL REFERENCES posts (uid),
	reason VARCHAR(255)
);

INSERT INTO mods (uname, pass, can_delete, can_edit, can_ban, can_sticky) VALUES
('god', '5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03', TRUE, TRUE, TRUE, TRUE);

INSERT INTO boards (link, name, notice, rules) VALUES
	('tv', 'TV and Films', 'Happy birthday TV and Films!', ARRAY['No loitering', 'Films are for shmucks, video games are where it"s at']),
	('v', 'Video games', NULL, ARRAY['Video games are serious business', 'Video games are for kids, films are where it"s at']),
	('mat', 'Mathematics', NULL, ARRAY['No rules here']);

INSERT INTO categories (name, bid)
	(SELECT 'Hobbies', uid from boards WHERE name='Video games');
INSERT INTO categories (name, bid)
	(SELECT 'Hobbies', uid from boards WHERE name='TV and Films');
INSERT INTO categories (name, bid)
	(SELECT 'Science', uid from boards WHERE name='Mathematics');
