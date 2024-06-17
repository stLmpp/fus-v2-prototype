create table if not exists main.migrations
(
    filename text(500) not null primary key,
    created_at integer NOT NULL default (unixepoch())
);

create table if not exists main.database_connection
(
    id                integer    not null
        primary key autoincrement,
    name              text(50)   not null,
    connection_string text(1000) not null,
    description       text(500),
    created_at integer not null default (unixepoch()),
    updated_at integer not null default (unixepoch())
);

