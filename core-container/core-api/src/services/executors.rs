use sqlx::{Executor, Postgres, query_as};

use crate::models::executors::ExecutorRow;

pub async fn get_executor<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<ExecutorRow>, sqlx::Error> {
     query_as::<_, ExecutorRow>(
        "select id, executor_type, admin, bot, userbot, updated_at, created_at from executors where id = $1"
    )
        .bind(id)
        .fetch_optional(executor)
        .await
}