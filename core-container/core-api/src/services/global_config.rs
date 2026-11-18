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

#[derive(Clone)]
pub struct Fees {
    pub admin_fee: i32,
    pub bot_fee: i32,
    pub userbot_fee: i32,
    pub userbot_user_token_fee: i32,
    pub userbot_owner_fee: i32,
    pub scale: i32,
}
#[derive(Clone, Serialize)]
struct DecimalFees {
    pub admin_fee: Decimal,
    pub bot_fee: Decimal,
    pub userbot_fee: Decimal,
    pub userbot_user_token_fee: Decimal,
    pub userbot_owner_fee: Decimal,
}

impl Serialize for Fees {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let scale = self.scale.ilog10();
        DecimalFees {
            admin_fee: Decimal::new(self.admin_fee as i64, scale as u32),
            bot_fee: Decimal::new(self.bot_fee as i64, scale as u32),
            userbot_fee: Decimal::new(self.userbot_fee as i64, scale as u32),
            userbot_user_token_fee: Decimal::new(self.userbot_user_token_fee as i64, scale as u32),
            userbot_owner_fee: Decimal::new(self.userbot_owner_fee as i64, scale as u32),
        }
        .serialize(serializer)
    }
}
pub async fn get_fees<'e>(
    executor: impl Executor<'e, Database = Postgres>,
) -> Result<Fees, sqlx::Error> {
    let fees = query!("select admin_fee, bot_fee, userbot_fee, userbot_user_token_fee, userbot_owner_fee, scale from global_config")
        .fetch_one(executor)
        .await?;
    let fees = Fees {
        admin_fee: fees.admin_fee,
        bot_fee: fees.bot_fee,
        userbot_fee: fees.userbot_fee,
        userbot_user_token_fee: fees.userbot_user_token_fee,
        userbot_owner_fee: fees.userbot_owner_fee,
        scale: fees.scale,
    };
    Ok(fees)
}
#[derive(Clone, Serialize)]
pub struct DailyClaim {
    pub daily_claim: i64,
    pub reset_hour: u32,
}

pub async fn get_daily_claim<'e>(
    executor: impl Executor<'e, Database = Postgres>,
) -> Result<DailyClaim, sqlx::Error> {
    let daily_claim = query!("select daily_claim, reset_hour from global_config")
    .fetch_one(executor)
    .await?;
    Ok(DailyClaim {
        daily_claim: daily_claim.daily_claim,
        reset_hour: daily_claim.reset_hour as u32,
    })
}
