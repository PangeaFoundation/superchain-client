use superchain_client::{
    provider::UniswapV3Provider,
    ClientBuilder,
    WsProvider,
};

/// not implemented yet
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

    let pool_address = "8ad599c3a0ff1de082011efddc58f1908eb6e6d8".parse().unwrap();

    let pool = client.get_pool(pool_address).await.unwrap();
    println!("Pool: {pool:?}");
}
