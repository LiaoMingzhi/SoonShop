//! SoonShop 客户端工具函数

use anyhow::Result;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_program,
    transaction::Transaction,
    instruction::Instruction,
    program_pack::Pack,
};
use spl_token::state::Mint;
use bs58;

/// 生成新的密钥对
pub fn generate_keypair() -> Keypair {
    Keypair::new()
}

/// 从 Base58 字符串解析公钥
pub fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey> {
    let pubkey = pubkey_str.parse::<Pubkey>()?;
    Ok(pubkey)
}

/// 将公钥转换为 Base58 字符串
pub fn pubkey_to_string(pubkey: &Pubkey) -> String {
    pubkey.to_string()
}

/// 从私钥字节创建密钥对
pub fn keypair_from_bytes(bytes: &[u8]) -> Result<Keypair> {
    let keypair = Keypair::from_bytes(bytes)?;
    Ok(keypair)
}

/// 从 Base58 字符串创建密钥对
pub fn keypair_from_base58(base58_str: &str) -> Result<Keypair> {
    let bytes = bs58::decode(base58_str).into_vec()?;
    keypair_from_bytes(&bytes)
}

/// 获取关联代币账户地址
pub fn get_associated_token_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(wallet, mint)
}

/// 创建关联代币账户指令
pub fn create_associated_token_account_instruction(
    payer: &Pubkey,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> Instruction {
    spl_associated_token_account::instruction::create_associated_token_account(
        payer,
        wallet,
        mint,
        &spl_token::id(),
    )
}

/// 验证签名
pub fn verify_signature(
    pubkey: &Pubkey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    signature.verify(pubkey.as_ref(), message)
}

/// 计算手续费
pub fn calculate_rent_exempt_minimum(data_size: usize) -> u64 {
    // 这个值应该从 RPC 获取，这里提供一个估算
    // 实际使用时应该调用 rpc_client.get_minimum_balance_for_rent_exemption()
    let rent_per_byte = 10_000_000u64; // 大约 0.01 SOL per byte
    data_size as u64 * rent_per_byte
}

/// 格式化 SOL 数量
pub fn format_sol(lamports: u64) -> String {
    let sol = lamports as f64 / 1_000_000_000.0;
    format!("{:.9} SOL", sol)
}

/// 解析 SOL 数量到 lamports
pub fn parse_sol(sol_str: &str) -> Result<u64> {
    let sol: f64 = sol_str.parse()?;
    Ok((sol * 1_000_000_000.0) as u64)
}

/// 获取当前时间戳
pub fn get_current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// 验证电子邮件格式
pub fn is_valid_email(email: &str) -> bool {
    // 简单的电子邮件验证
    email.contains('@') && email.contains('.')
}

/// 生成随机字符串
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair();
        assert_eq!(keypair.pubkey().to_bytes().len(), 32);
    }

    #[test]
    fn test_parse_pubkey() {
        let pubkey_str = "11111111111111111111111111111112";
        let pubkey = parse_pubkey(pubkey_str).unwrap();
        assert_eq!(pubkey, system_program::ID);
    }

    #[test]
    fn test_format_sol() {
        assert_eq!(format_sol(1_000_000_000), "1.000000000 SOL");
        assert_eq!(format_sol(500_000_000), "0.500000000 SOL");
    }

    #[test]
    fn test_parse_sol() {
        assert_eq!(parse_sol("1.0").unwrap(), 1_000_000_000);
        assert_eq!(parse_sol("0.5").unwrap(), 500_000_000);
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
    }

    #[test]
    fn test_generate_random_string() {
        let random_str = generate_random_string(10);
        assert_eq!(random_str.len(), 10);
    }
} 