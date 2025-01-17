#![cfg_attr(not(debug_assertions), deny(unused_variables))]
#![cfg_attr(not(debug_assertions), deny(unused_imports))]
#![cfg_attr(not(debug_assertions), deny(dead_code))]
#![cfg_attr(not(debug_assertions), deny(unused_extern_crates))]
#![deny(unused_must_use)]
#![deny(unreachable_patterns)]
#![deny(unknown_lints)]
#![recursion_limit = "2048"]
#![feature(drain_filter)]

#[macro_use]
mod macros;
pub mod base_node_service;
pub mod contacts_service;
pub mod error;
pub mod output_manager_service;
pub mod storage;
pub mod test_utils;
pub mod transaction_service;
pub mod types;
pub mod util;
pub mod wallet;

#[cfg(feature = "test_harness")]
pub mod testnet_utils;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

mod config;
pub mod schema;
pub mod utxo_scanner_service;

pub use config::WalletConfig;
pub use wallet::Wallet;

use crate::{
    contacts_service::storage::sqlite_db::ContactsServiceSqliteDatabase,
    output_manager_service::storage::sqlite_db::OutputManagerSqliteDatabase,
    storage::sqlite_db::WalletSqliteDatabase,
    transaction_service::storage::sqlite_db::TransactionServiceSqliteDatabase,
};

pub type WalletSqlite = Wallet<
    WalletSqliteDatabase,
    TransactionServiceSqliteDatabase,
    OutputManagerSqliteDatabase,
    ContactsServiceSqliteDatabase,
>;
