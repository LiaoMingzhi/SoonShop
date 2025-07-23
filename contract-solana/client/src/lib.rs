//! SoonShop 智能合约客户端 SDK
//! 
//! 该库提供了与 SoonShop Solana 智能合约交互的 Rust 客户端接口

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::Signer,
};

pub mod error;
pub mod client;
pub mod types;
pub mod utils;

pub use client::SoonShopClient;
pub use error::SoonShopError;

/// SoonShop 客户端配置
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// RPC 端点 URL
    pub rpc_url: String,
    /// 承诺级别
    pub commitment: CommitmentConfig,
    /// 程序 ID
    pub program_id: Pubkey,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            commitment: CommitmentConfig::confirmed(),
            program_id: Pubkey::default(), // 需要设置实际的程序 ID
        }
    }
}

/// 创建新的 SoonShop 客户端
pub fn new_client(config: ClientConfig) -> Result<SoonShopClient> {
    let rpc_client = RpcClient::new_with_commitment(config.rpc_url, config.commitment);
    SoonShopClient::new(rpc_client, config.program_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.rpc_url, "https://api.mainnet-beta.solana.com");
        assert_eq!(config.commitment, CommitmentConfig::confirmed());
    }
} 