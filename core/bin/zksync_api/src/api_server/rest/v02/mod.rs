// External uses
use actix_web::{
    web::{self},
    Scope,
};
// Workspace uses
use crate::api_server::rest::network_status::SharedNetworkStatus;
use zksync_api_types::v02::ApiVersion;
use zksync_config::ZkSyncConfig;
use zksync_types::network::Network;

// Local uses
use crate::api_server::tx_sender::TxSender;

mod account;
mod block;
mod config;
pub mod error;
mod fee;
mod paginate_impl;
mod paginate_trait;
mod response;
mod status;
#[cfg(test)]
pub mod test_utils;
mod token;
mod transaction;

#[derive(Debug, Clone, Copy)]
pub struct SharedData {
    pub net: Network,
    pub api_version: ApiVersion,
}

pub(crate) fn api_scope(
    tx_sender: TxSender,
    zk_config: &ZkSyncConfig,
    network_status: SharedNetworkStatus,
) -> Scope {
    let data = SharedData {
        net: zk_config.chain.eth.network,
        api_version: ApiVersion::V02,
    };
    web::scope("/api/v0.2")
        .app_data(web::Data::new(data))
        .service(account::api_scope(
            tx_sender.pool.clone(),
            tx_sender.tokens.clone(),
            zk_config.eth_watch.confirmations_for_eth_event,
        ))
        .service(block::api_scope(
            tx_sender.pool.clone(),
            tx_sender.blocks.clone(),
        ))
        .service(config::api_scope(zk_config))
        .service(fee::api_scope(tx_sender.clone()))
        .service(status::api_scope(network_status))
        .service(token::api_scope(
            zk_config,
            tx_sender.pool.clone(),
            tx_sender.tokens.clone(),
            tx_sender.ticker.clone(),
        ))
        .service(transaction::api_scope(tx_sender))
}
