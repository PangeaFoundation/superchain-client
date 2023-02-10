use superchain_client::{
    provider::UniswapV2Provider,
    ClientBuilder,
    HttpProvider,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let username = std::env::var("SUPERCHAIN_USERNAME").unwrap();
    let password = std::env::var("SUPERCHAIN_PASSWORD").unwrap();

    let client = ClientBuilder::default()
        .credential(&username, &password)
        .build::<HttpProvider>()
        .await
        .unwrap();

    let pair_address = "b4e16d0168e52d35cacd2c6185b44281ec28c9dc".parse().unwrap();

    let pair = client.get_pair(pair_address).await.unwrap();
    println!("Pair: {pair:?}");
}
