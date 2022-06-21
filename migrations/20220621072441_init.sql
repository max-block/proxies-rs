create table source (
    id serial primary key,
    name text not null unique,
    link text not null unique,
    username text not null,
    password text not null,
    port integer not null,
    protocol text not null check (protocol in ('http', 'socks5')),
    created_at timestamptz not null default now(),
    checked_at timestamptz
);
create index on source(created_at);
create index on source(checked_at);


create table proxy (
    id serial primary key,
    status text not null check (status in ('unknown', 'ok', 'down')) default 'unknown',
    protocol text not null check (protocol in ('http', 'socks5')),
    host text not null unique,
    username text not null,
    password text not null,
    port integer not null,
    created_at timestamptz not null default now(),
    checked_at timestamptz,
    last_ok_at timestamptz
);
create index on proxy(status);
create index on proxy(created_at);
create index on proxy(checked_at);
create index on proxy(last_ok_at);

