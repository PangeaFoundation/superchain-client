//! # superchain-client
//!
//! ### Introduction
//! This crate allows you to access [Superchain API](https://docs.superchain.network/).

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(rust_2018_idioms, rustdoc::broken_intra_doc_links)]

pub mod core;
mod providers;

pub use ::{futures, reqwest, tokio, tokio_tungstenite, tungstenite, url};

pub use  ethers_core::types::Address;

#[doc(inline)]
pub use crate::core::{
    builder::ClientBuilder,
    client::Client,
    error::{Error, Result},
    provider, requests,
    types::{format::Format, query, ChainId},
    utils,
};
#[doc(inline)]
pub use crate::providers::{http::HttpProvider, ws::WsProvider};
