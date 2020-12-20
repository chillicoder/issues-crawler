-- Your SQL goes here
create table issues (
    id bigserial primary key,
    author varchar(256) not null,
    title varchar(256) not null,
    url varchar(4096) not null
)
