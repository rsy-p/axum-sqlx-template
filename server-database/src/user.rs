use server_model::User;
use sqlx::PgExecutor;
use uuid::Uuid;

pub struct UserMapper;
impl UserMapper {
    pub async fn query_by_id<'c, E>(id: Uuid, executor: E) -> anyhow::Result<Option<User>>
    where
        E: PgExecutor<'c>,
    {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT
                    *
                FROM
                    "user"
                WHERE
                    ID = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;
        Ok(result)
    }
}
