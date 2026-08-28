-- The Pomka Ecosystem Core Source Code
-- Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
-- 
-- This program is free software: you can redistribute it and/or modify
-- it under the terms of the GNU General Public License as published by
-- the Free Software Foundation, either version 3 of the License.
-- 
-- This program is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
-- GNU General Public License for more details.
-- 
-- You should have received a copy of the GNU General Public License
-- along with this program.  If not, see <https://www.gnu.org/licenses/>.

create table global_config (
    id int primary key check (id = 1),
    admin_fee real not null,
    bot_fee real not null,
    userbot_fee real not null,
    userbot_user_token_fee real not null,
    userbot_owner_fee real not null,
    updated_at timestamptz not null default current_timestamp,
    check (admin_fee >= 0 and admin_fee <= 1),
    check (bot_fee >= 0 and bot_fee <= 1),
    check (userbot_fee >= 0 and userbot_fee <= 1),
    check (userbot_user_token_fee >= 0 and userbot_user_token_fee <= 1),
    check (userbot_owner_fee >= 0 and userbot_owner_fee <= 1)
);


create trigger set_update_global_config before update on global_config
    for each row execute function update_modified_column();

insert into global_config (id, admin_fee, bot_fee, userbot_fee, userbot_user_token_fee, userbot_owner_fee)
values (1, 0.0, 0.01, 0.02, 0.05, 0.05);