mod db;

#[tokio::main]
async fn main() {
    menu().await;
}

async fn menu() {
    println!("1 - Encurtar URL");
    println!("2 - Desencurtar URL");

    let mut opc = String::new();

    std::io::stdin()
    .read_line(&mut opc)
    .unwrap();

    let opc: u8 = opc.trim().parse().unwrap();

    match opc {
        1 => encurtar_url().await,
        2 => desencurtar_url().await,
        _ => println!("Entrada inválida"),
    };
}

async fn encurtar_url() {
    println!("Digite a url: ");

    let mut url = String::new();
    std::io::stdin()
    .read_line(&mut url)
    .unwrap();

    let _encurtar = db::shorten(url).await.unwrap();
    
    println!("URL encurtada");
}

async fn desencurtar_url() {
    todo!()
}