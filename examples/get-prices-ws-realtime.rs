use ethers::types::H160;
use futures::StreamExt;
use superchain_client::{
    provider::UniswapV2Provider,
    query::{
        Bound,
        QueryOptions,
    },
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

    let pair_address = "b4e16d0168e52d35cacd2c6185b44281ec28c9dc".parse().unwrap();
    let filter: Vec<H160> = vec![pair_address];
    let query = QueryOptions::default()
        .start(Bound::Latest)
        .end(Bound::None);

    let stream = UniswapV2Provider::get_prices(&client, filter, query)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(trade) = stream.next().await {
        println!("Price: {trade:?}");
    }
}
