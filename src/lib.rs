//! # superchain-client
//!
//! Superchain is a start up with the mission provide easy and fast access to on
//! chain data. This crate gives you easy access to a hand full of our
//! endpoints. Namely `get_pair` and `get_prices`. These endpoints allow you to
//! get a stream of all pair created events and all price quotes of uniswap v2.
//!
//! ### Introduction
//! This crates allows you to easily use the Superchain API. Both the WebSocket
//! endpoints and the HTTP endpoints.
//!
//! ### API overview
//! There are two ways to interface with Superchain: HTTP and WebSocket
//!
//! The WebSocket interface is a lot more flexible and powerful, while also
//! being simpler, so use this one whenever you can.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(rust_2018_idioms, rustdoc::broken_intra_doc_links)]

mod core;
mod providers;

pub use ::{
    ethers,
    futures,
    reqwest,
    tokio,
    tokio_tungstenite,
    tungstenite,
    url,
};

#[doc(inline)]
pub use crate::core::{
    builder::ClientBuilder,
    client::Client,
    error::{
        Error,
        Result,
    },
    provider,
    types::{
        ethereum,
        query,
        uniswap_v2,
        uniswap_v3,
    },
};
#[doc(inline)]
pub use crate::providers::{
    http::HttpProvider,
    ws::WsProvider,
};
