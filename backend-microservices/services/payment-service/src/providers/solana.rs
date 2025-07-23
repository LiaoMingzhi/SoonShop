use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
    transaction::Transaction,
};
use std::str::FromStr;
use crate::config::SolanaConfig;
use crate::models::payment::{Payment, PaymentStatus};

pub struct SolanaProvider {
    client: RpcClient,
    program_id: Pubkey,
    token_mint: Pubkey,
    treasury_account: Pubkey,
}

impl SolanaProvider {
    pub async fn new(config: &SolanaConfig) -> Result<Self> {
        let client = RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );
        
        let program_id = Pubkey::from_str(&config.program_id)?;
        let token_mint = Pubkey::from_str(&config.token_mint)?;
        let treasury_account = Pubkey::from_str(&config.treasury_account)?;
        
        Ok(SolanaProvider {
            client,
            program_id,
            token_mint,
            treasury_account,
        })
    }
    
    pub async fn verify_transaction(&self, signature: &str) -> Result<bool> {
        let sig = Signature::from_str(signature)?;
        let result = self.client.confirm_transaction(&sig)?;
        Ok(result)
    }
    
    pub async fn get_transaction_status(&self, signature: &str) -> Result<PaymentStatus> {
        let sig = Signature::from_str(signature)?;
        match self.client.get_signature_status(&sig)? {
            Some(Ok(())) => Ok(PaymentStatus::Completed),
            Some(Err(_)) => Ok(PaymentStatus::Failed),
            None => Ok(PaymentStatus::Pending),
        }
    }
    
    pub async fn health_check(&self) -> Result<()> {
        let _health = self.client.get_health()?;
        Ok(())
    }
} 