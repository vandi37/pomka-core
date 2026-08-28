// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::{Executor, Postgres, query};


#[derive( Clone)]
pub struct Fees {
    pub admin: i32,
    pub bot: i32,
    pub userbot: i32,
    pub userbot_user_token: i32,
    pub userbot_owner: i32,
    pub scale: i32
}
#[derive( Clone,Serialize)]
struct DecimalFees {
    pub admin: Decimal,
    pub bot: Decimal,
    pub userbot: Decimal,
    pub userbot_user_token: Decimal,
    pub userbot_owner: Decimal,
}

impl Serialize for Fees {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let scale = self.scale.ilog10();
        DecimalFees {
            admin: Decimal::new(self.admin as i64, scale as u32),
            bot: Decimal::new(self.bot as i64, scale as u32),
            userbot: Decimal::new(self.userbot as i64, scale as u32),
            userbot_user_token: Decimal::new(self.userbot_user_token as i64, scale as u32),
            userbot_owner: Decimal::new(self.userbot_owner as i64, scale as u32),
        }.serialize(serializer)
    }
}
pub async fn get_fees<'e>(executor: impl Executor<'e, Database = Postgres>) -> Result<Fees, sqlx::Error> {
    let fees = query!("select admin_fee, bot_fee, userbot_fee, userbot_user_token_fee, userbot_owner_fee, scale from global_config")
        .fetch_one(executor)
        .await?;
    let fees = Fees {
        admin: fees.admin_fee,
        bot: fees.bot_fee,
        userbot: fees.userbot_fee,
        userbot_user_token: fees.userbot_user_token_fee,
        userbot_owner: fees.userbot_owner_fee,
        scale: fees.scale,
    };
    Ok(fees)
}