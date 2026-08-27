

use sqlx::{Executor, Postgres, query_as};

use crate::{models::users::User};

pub async fn get_user<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where id = $1"
    )
            .bind(id)
            .fetch_optional(executor)
            .await
}

pub async fn get_user_for_update<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where id = $1 for update "
    )
            .bind(id)
            .fetch_optional(executor)
            .await
}

pub async fn get_user_by_handle<'e>(executor: impl Executor<'e, Database = Postgres>, userhandle: &str) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where userhandle = $1"
    )
            .bind(userhandle)
            .fetch_optional(executor)
            .await
}