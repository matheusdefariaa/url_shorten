mod db;
mod error;

use colored::{self, Colorize};
use error::AppError;

#[tokio::main]
async fn main() {
    menu().await;
}

async fn menu() {

    let pool: sqlx::Pool<sqlx::Postgres> = db::connect_db().await.expect("Error ao chamar db");

    loop {
        println!("1 - Encurtar URL");
        println!("2 - Desencurtar URL\n");

        let mut opc = String::new();

        std::io::stdin()
        .read_line(&mut opc)
        .unwrap();

        let opc: u8 = opc.trim().parse().expect("Error em opc");

        let _: Result<(), AppError> = match opc {
            1 => encurtar_url(&pool).await,
            2 => desencurtar_url().await,
            _ => continue,
        };
    }
}

async fn encurtar_url(pool: &sqlx::PgPool) -> Result<(), error::AppError> {
    println!("Digite a url: ");

    let mut url = String::new();
    std::io::stdin()
    .read_line(&mut url)
    .expect("Falha ao criar string");

    let url = url.trim().to_string();

    let encurtar = db::shorten(&pool, url).await.unwrap();
    
    println!("{}", "-".repeat(78));
    println!("\t{}: {}","URL encurtada".bold().green(),encurtar.blue());
    println!("{}\n", "-".repeat(78));

    Ok(())

}

async fn desencurtar_url() -> Result<(),error::AppError> {
    Err(error::AppError::DatabaseError((sqlx::Error::BeginFailed)))
}