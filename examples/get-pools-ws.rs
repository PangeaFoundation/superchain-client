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

    let query = QueryOptions::default()
        .start(Bound::Exact(height - 100000))
        .end(Bound::Latest);

    let stream = client.get_pools(query).await.unwrap();
    futures::pin_mut!(stream);

    while let Some(pair) = stream.next().await {
        println!("Pair: {pair:?}");
    }
}
