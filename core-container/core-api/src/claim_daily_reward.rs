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

use chrono::prelude::*;
use redis::{AsyncCommands, RedisError, RedisResult};

/// KEYS[1] = the redis key for this user's last claim
/// ARGV[1] = current unix timestamp (seconds)
/// ARGV[2] = the most recent reset boundary (unix timestamp, seconds)
const CLAIM_SCRIPT: &str = r#"
    local last = redis.call('GET', KEYS[1])
    if last and tonumber(last) >= tonumber(ARGV[2]) then
        return 0
    end
    redis.call('SET', KEYS[1], ARGV[1], 'EX', 86500) -- a bit more than a day
    return 1
"#;
const DAILY_KEY: &str = "users-daily";

/// returns unix timestamp (seconds)
fn most_recent_reset_boundary(reset_hour_utc: u32) -> i64 {
    let now = Utc::now();
    let mut boundary = Utc
        .with_ymd_and_hms(now.year(), now.month(), now.day(), reset_hour_utc, 0, 0)
        .unwrap();
    if boundary > now {
        boundary -= chrono::Duration::days(1);
    }

    boundary.timestamp()
}
pub enum ClaimDailyRewardError {
    Redis(RedisError),
    AlreadyClaimedToday(i64), 
}

pub async fn claim_daily_reward(
    client: &redis::Client,
    id: i64,
    reset_hour_utc: u32,
) -> Result<(), ClaimDailyRewardError> {
    let mut conn = client
        .get_connection_manager()
        .await
        .map_err(ClaimDailyRewardError::Redis)?;

    let key = format!("{DAILY_KEY}:{id}");
    let now = Utc::now().timestamp();
    let boundary = most_recent_reset_boundary(reset_hour_utc);

    let script = redis::Script::new(CLAIM_SCRIPT);
    let result: i32 = script
        .key(&key)
        .arg(now)
        .arg(boundary)
        .invoke_async(&mut conn)
        .await
        .map_err(ClaimDailyRewardError::Redis)?;

    if result == 0 {
        Err(ClaimDailyRewardError::AlreadyClaimedToday(boundary + chrono::Duration::days(1).num_seconds()))
    } else {
        Ok(())
    }
}

pub async fn remove_daily_reward_record(
    client: &redis::Client,
    id: i64,
) -> RedisResult<()> {
    let mut conn = client
        .get_connection_manager()
        .await?;
    let key = format!("{DAILY_KEY}:{id}");
    conn.del(key).await?
}