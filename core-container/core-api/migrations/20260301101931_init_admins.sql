create table admins (
    id bigserial primary key,
    username text not null unique ,
    password text not null ,
    creator bigint references admins(id) on delete cascade, -- may be null
    updated_at timestamptz not null default current_timestamp,
    created_at timestamptz not null default current_timestamp
);

create index on admins (creator);
create trigger set_update_admins before update on admins
    for each row execute function  update_modified_column();