-- users table
CREATE TABLE IF NOT EXISTS users(
    id blob(16) not null primary key,
    username text not null,
    email text not null,
    password text not null
);
