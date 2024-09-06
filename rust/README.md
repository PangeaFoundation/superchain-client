# Rust superchain-client

## Credentials

You will be given a username and password to use to access the Superchain API. The easiest way to use these credentials is to create a .env file in the same folder as this README.md file like so:

    SUPER_USERNAME=xxxxx 
    SUPER_PASSWORD=xxxxx
    SUPER_URL=app.superchain.network


## Example

```rust
use std::collections::HashSet;

use futures::StreamExt;
use superchain_client::{
    core::types::{format::Format, ChainId},
    provider::UniswapV3Provider,
    query::Bound,
    requests::uniswap_v3::GetPricesRequest,
    ClientBuilder,
    WsProvider,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("SUPER_URL")
        .unwrap_or("app.superchain.network".to_string());
    let username = std::env::var("SUPER_USERNAME").unwrap();
    let password = std::env::var("SUPER_PASSWORD").unwrap();

    // setup websocket client
    let client = ClientBuilder::default()
        .endpoint(&url)
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    // subscribe to prices 
    let request = GetPricesRequest {
        chains: HashSet::from([ChainId::ETH]),
        from_block: Bound::Latest, 
        to_block: Bound::Subscribe, // real-time
        // for historical data without subscription uncomment below
        // from_block: Bound::FromLatest(100), 
        // to_block: Bound::Latest,
        ..Default::default()
    };

    let stream = client.get_prices_by_format(request, Format::JsonStream, false).await.unwrap();

    futures::pin_mut!(stream);
    
    // async iterator over stream of data
    while let Some(data) = stream.next().await {
        let price = String::from_utf8(data.unwrap()).unwrap(); // or use serde json
        println!("Price: {price:?}");
    }
}
```
