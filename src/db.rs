use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;


pub async fn connect_db() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();

    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST não está configurado");
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT não está configurado");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB não está configurado");
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER não está configurado");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD não está configurado");

    // Construir a URL de conexão com o PostgreSQL
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?;

    Ok(pool)
}


pub async fn shorten(url_real: String) -> String {
    let pool: sqlx::Pool<sqlx::Postgres> = connect_db().await.expect("Error ao chamar db");

    let url_shorten = format!("encurtador.com/{}",Uuid::new_v4());

    sqlx::query("INSERT INTO url (url_shorten, unshorten) VALUES ($1,$2)")
        .bind(url_shorten)
        .bind(url_real)
        .execute(&pool)
        .await
        .unwrap();

    "a".to_string()
}