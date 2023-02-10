use futures::StreamExt;
use superchain_client::{
    provider::{
        EthereumProvider,
        UniswapV2Provider,
    },
    query::{
        Bound,
        QueryOptions,
    },
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

    let height = client.get_height().await.unwrap();

    let query = QueryOptions::default()
        .start(Bound::Exact(height - 100))
        .end(Bound::Latest);
    let pair_address = "b4e16d0168e52d35cacd2c6185b44281ec28c9dc".parse().unwrap();

    let stream = client.get_prices(vec![pair_address], query).await.unwrap();
    futures::pin_mut!(stream);

    while let Some(trade) = stream.next().await {
        println!("Price: {trade:?}");
    }
}
