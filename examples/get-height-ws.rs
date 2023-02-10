use superchain_client::{
    provider::EthereumProvider,
    ClientBuilder,
    WsProvider,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let username = std::env::var("SUPERCHAIN_USERNAME").unwrap();
    let password = std::env::var("SUPERCHAIN_PASSWORD").unwrap();

    let client = ClientBuilder::default()
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    let height = client.get_height().await.unwrap();
    println!("Height: {height}");
}
