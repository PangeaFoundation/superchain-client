use std::vec;

use futures::StreamExt;
use superchain_client::{
    provider::{
        EthereumProvider,
        UniswapV3Provider,
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
    let pool_address = "8ad599c3a0ff1de082011efddc58f1908eb6e6d8".parse().unwrap();

    let stream = UniswapV3Provider::get_prices(&client, vec![pool_address], query)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(trade) = stream.next().await {
        println!("Price: {trade:?}");
    }
}
