
create table bots (
    id bigserial primary key ,
    username text unique not null,
    password text not null,
    creator bigint references admins(id) on delete set null,
    allow_produce_stocks boolean not null,
    updated_at timestamptz not null default current_timestamp,
    created_at timestamptz not null default current_timestamp
);

create index on bots(creator);
create trigger set_update_bots before update on bots
    for each row execute function  update_modified_column();