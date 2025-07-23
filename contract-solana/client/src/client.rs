//! SoonShop 客户端主要实现

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

use crate::error::SoonShopError;
use crate::types::*;

/// SoonShop 智能合约客户端
pub struct SoonShopClient {
    /// RPC 客户端
    rpc_client: RpcClient,
    /// 程序 ID
    program_id: Pubkey,
}

impl SoonShopClient {
    /// 创建新的客户端实例
    pub fn new(rpc_client: RpcClient, program_id: Pubkey) -> Result<Self> {
        Ok(Self {
            rpc_client,
            program_id,
        })
    }

    /// 获取程序 ID
    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }

    /// 获取 RPC 客户端
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// 发送并确认交易
    pub async fn send_and_confirm_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<Signature, SoonShopError> {
        let signature = self.rpc_client
            .send_and_confirm_transaction(transaction)
            .map_err(SoonShopError::RpcClient)?;
        
        Ok(signature)
    }

    /// 获取账户信息
    pub async fn get_account_info(&self, pubkey: &Pubkey) -> Result<Option<solana_sdk::account::Account>, SoonShopError> {
        let account = self.rpc_client
            .get_account(pubkey)
            .map_err(SoonShopError::RpcClient)?;
        
        Ok(Some(account))
    }

    // 这里可以添加更多的智能合约交互方法
    // 例如：创建产品、下单、支付等
    
    /// 创建用户账户
    pub async fn create_user_account(
        &self,
        user_keypair: &Keypair,
        user_data: CreateUserParams,
    ) -> Result<Signature, SoonShopError> {
        // 这里应该实现创建用户账户的逻辑
        // 暂时返回一个占位符
        todo!("Implement create_user_account")
    }

    /// 获取用户信息
    pub async fn get_user_info(&self, user_pubkey: &Pubkey) -> Result<Option<UserInfo>, SoonShopError> {
        // 这里应该实现获取用户信息的逻辑
        // 暂时返回一个占位符
        todo!("Implement get_user_info")
    }
} 