//! SoonShop 客户端错误类型

use thiserror::Error;
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::ParsePubkeyError;

/// SoonShop 客户端错误类型
#[derive(Debug, Error)]
pub enum SoonShopError {
    #[error("Solana RPC 客户端错误: {0}")]
    RpcClient(#[from] ClientError),
    
    #[error("公钥解析错误: {0}")]
    ParsePubkey(#[from] ParsePubkeyError),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Anchor 错误: {0}")]
    Anchor(#[from] anchor_lang::error::Error),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("交易错误: {0}")]
    Transaction(String),
    
    #[error("账户不存在: {0}")]
    AccountNotFound(String),
    
    #[error("权限不足")]
    Unauthorized,
    
    #[error("无效参数: {0}")]
    InvalidParameter(String),
    
    #[error("操作失败: {0}")]
    OperationFailed(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, SoonShopError>; 