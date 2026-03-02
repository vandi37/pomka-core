
create table stocks (
    id bigserial primary key,
    owner_id bigint references users(id) on delete cascade not null ,
    capacity bigint not null,
    left_amount bigint not null,
    base bigint not null,
    power float not null,
    executor bigint references executors(id) on delete no action not null,
    updated_at timestamptz not null default current_timestamp,
    created_at timestamptz not null default current_timestamp
);

create trigger set_update_stocks before update on stocks
    for each row execute function update_modified_column();

create index idx_active_stocks on stocks (left_amount) where left_amount > 0;
create index idx_user_stocks on stocks (owner_id);