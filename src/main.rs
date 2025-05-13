mod db;
use colored::{self, Colorize};

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

        let opc: u8 = opc.trim().parse().unwrap();

        match opc {
            1 => encurtar_url(&pool).await,
            2 => desencurtar_url().await,
            _ => println!("Entrada inválida"),
        };
    }
}

async fn encurtar_url(pool: &sqlx::PgPool) {
    println!("Digite a url: ");

    let mut url = String::new();
    std::io::stdin()
    .read_line(&mut url)
    .unwrap();


    let encurtar = db::shorten(&pool, url).await.unwrap();
    
    println!("{}", "-".repeat(78));
    println!("\t{}: {}","URL encurtada".bold().green(),encurtar.blue());
    println!("{}\n", "-".repeat(78));

}

async fn desencurtar_url() {
    todo!()
}