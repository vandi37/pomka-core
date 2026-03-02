create table global_config (
    id int primary key check (id = 1),
    control_pool bigint default 0,
    updated_at timestamptz not null default current_timestamp
);


create trigger set_update_global_config before update on global_config
    for each row execute function update_modified_column();

insert into global_config (id)
values (1);