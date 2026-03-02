create table userbots(
    id bigserial primary key,
    owner_id bigint references users(id) on delete cascade not null,
    relevancy bigint not null default random(),
    updated_at timestamptz not null default current_timestamp,
    created_at timestamptz not null default current_timestamp
);
create index on userbots(owner_id);
create trigger set_update_userbotsbots before update on userbots
    for each row execute function update_modified_column();