use sqlx::sqlite::SqlitePoolOptions;

pub async fn init(database_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
