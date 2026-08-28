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

use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use redis::RedisError;

use crate::{claim_daily_reward::{self, ClaimDailyRewardError, remove_daily_reward_record}, error::AppError, models::executors::ExecutorType, routes::Executor, services::{executors::get_executor, users::{get_pool_id, get_user}}, state::AppState, transactions::{Pay, TxAllowance, notify_about_transaction, pay_tx}};
pub const TYPE: &str = "claim-daily-reward";

pub async fn claim_daily_reward(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id, 
            "gotten error while creating transaction");
        AppError::Internal
    })?;

    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id,
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }

    get_user(&state.db, id).await.map_err(|e| {
        tracing::error!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id,
            "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(id))?;

    match claim_daily_reward::claim_daily_reward(&state.redis, id, state.reset_hour_utc).await {
        Err(ClaimDailyRewardError::Redis(r)) => {
            tracing::error!(target: "claim-daily-reward", 
                error=?r, 
                executor=executor.id, 
                id,
            "gotten error while getting and updating daily reward claims in redis");
            Err(AppError::Internal)
        },
        Err(ClaimDailyRewardError::AlreadyClaimedToday(next)) => Err(AppError::DailyRewardAlreadyClaimed(next)),
        Ok(()) => Ok(()),
    }?;

    let pool = get_pool_id(&state.db).await.map_err(|e| {
            tracing::error!(target: "claim-daily-reward", 
                error=?e, 
                id,
                by=executor.id,
            "gotten error while getting pool id");
            AppError::Internal
        })?;

    let pay = Pay {
        sender: pool ,
        receiver: id, 
        data: None,
        amount: state.daily_claim,
        executor: executor.id,
        r#type: TYPE.into(),
        allowed: TxAllowance::NotAllowed, // doesn't matter
        idempotency_key: None,
    };
    let delete_redis = |e: RedisError| {
        tracing::error!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id,
        "gotten error while deleting daily reward record from redis");
    };
    let (transaction, nl_sender, nl_receiver) = match pay_tx(pay, &mut tx).await {
        Err(e) => {
            tracing::error!(target: "claim-daily-reward", 
                error=?e, 
                executor=executor.id, 
                id,
                "gotten error while giving daily reward");
            remove_daily_reward_record(&state.redis, id).await.unwrap_or_else(delete_redis);
            return Err(AppError::Internal)
        }
        Ok(ok)=> ok
       
    };
    if let Err(e) = tx.commit().await {
        tracing::error!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id,
        "gotten error while committing transaction");
        remove_daily_reward_record(&state.redis, id).await.unwrap_or_else(delete_redis);
        return Err(AppError::Internal)?
    };

    notify_about_transaction(&state.redis, pool, nl_sender, id, nl_receiver, &transaction)
    .await.unwrap_or_else(|e| {
        tracing::warn!(target: "claim-daily-reward", 
            error=?e, 
            executor=executor.id, 
            id,
        "gotten error while publishing transaction");
    });
    
    Ok(Json(transaction))
}