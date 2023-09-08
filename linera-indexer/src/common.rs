// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::http::GraphiQLSource;
use axum::{
    http::Uri,
    response::{self, IntoResponse},
};
use linera_base::crypto::CryptoHash;
use linera_views::rocks_db::RocksDbContextError;
use reqwest::header::InvalidHeaderValue;
use std::net::AddrParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    GraphQLError(#[from] graphql_ws_client::Error),
    #[error(transparent)]
    TungsteniteError(#[from] async_tungstenite::tungstenite::Error),
    #[error(transparent)]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error(transparent)]
    ParserError(#[from] AddrParseError),
    #[error(transparent)]
    ServerError(#[from] hyper::Error),
    #[error(transparent)]
    RocksDbError(#[from] RocksDbContextError),
    #[error("Null GraphQL data: {0:?}")]
    NullData(Option<Vec<graphql_client::Error>>),
    #[error("Block not found: {0}")]
    NotFound(linera_base::crypto::CryptoHash),
    #[error("Unknown plugin: {0}")]
    UnknownPlugin(String),
    #[error("Plugin not loaded: {0}")]
    UnloadedPlugin(String),
    #[error("Unknown certificate status: {0:?}")]
    UnknownCertificateStatus(String),
    #[error("Different plugins in command line and memory")]
    WrongPlugins,
    #[error("Plugin is already registered")]
    PluginAlreadyRegistered,
    #[error("Invalid block: {0:?}")]
    InvalidBlock(CryptoHash),
}

pub async fn graphiql(uri: Uri) -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint(uri.path()).finish())
}
