-- users table
CREATE TABLE IF NOT EXISTS users(
    id blob(16) not null primary key,
    username text not null,
    email text not null,
    password text not null
);

-- questions table
CREATE TABLE IF NOT EXISTS questions(
    id blob(16) not null primary key,
    question text not null,
    a text not null,
    b text not null,
    c text not null,
    d text not null,
    answer int(1) not null,
    category blob(16) not null,
    foreign key(category) references categories(id)
);

-- categories table
CREATE TABLE IF NOT EXISTS categories(
    id blob(16) not null primary key,
    name text not null
);
