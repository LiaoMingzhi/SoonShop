//! SoonShop CLI 工具
//! 
//! 用于与 SoonShop 智能合约交互的命令行界面

use clap::{Args, Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;
use soonshop_client::{ClientConfig, SoonShopClient};
use std::str::FromStr;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[clap(name = "soonshop-cli", version = "1.0.0")]
#[clap(about = "SoonShop 智能合约命令行工具")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Args, Debug, Clone)]
struct Rpc {
    #[clap(short, long, default_value = "https://api.devnet.solana.com")]
    url: String,

    #[clap(short, long, default_value = "")]
    fee_payer: String,
}

#[derive(Args, Debug, Clone)]
struct CreateUser {
    /// 用户名
    #[clap(short, long)]
    username: String,

    /// 电子邮件
    #[clap(short, long)]
    email: String,

    /// 钱包私钥文件路径
    #[clap(short, long)]
    keypair: String,

    #[clap(flatten)]
    rpc: Rpc,
}

#[derive(Args, Debug, Clone)]
struct GetUser {
    /// 用户公钥
    #[clap(short, long)]
    pubkey: String,

    #[clap(flatten)]
    rpc: Rpc,
}

#[derive(Args, Debug, Clone)]
struct CreateProduct {
    /// 产品名称
    #[clap(short, long)]
    name: String,

    /// 产品描述
    #[clap(short, long)]
    description: String,

    /// 产品价格（以 lamports 为单位）
    #[clap(short, long)]
    price: u64,

    /// 库存数量
    #[clap(short, long)]
    stock: u64,

    /// 商家私钥文件路径
    #[clap(short, long)]
    keypair: String,

    #[clap(flatten)]
    rpc: Rpc,
}

#[derive(Args, Debug, Clone)]
struct ListProducts {
    #[clap(flatten)]
    rpc: Rpc,
}

#[derive(Args, Debug, Clone)]
struct PlaceOrder {
    /// 产品 ID
    #[clap(short, long)]
    product_id: u64,

    /// 购买数量
    #[clap(short, long)]
    quantity: u64,

    /// 买家私钥文件路径
    #[clap(short, long)]
    keypair: String,

    #[clap(flatten)]
    rpc: Rpc,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// 创建用户账户
    CreateUser(CreateUser),
    /// 获取用户信息
    GetUser(GetUser),
    /// 创建产品
    CreateProduct(CreateProduct),
    /// 列出产品
    ListProducts(ListProducts),
    /// 下单
    PlaceOrder(PlaceOrder),
    /// 显示程序 ID
    ProgramId {
        #[clap(flatten)]
        rpc: Rpc,
    },
}

impl Rpc {
    fn client_config(&self) -> ClientConfig {
        ClientConfig {
            rpc_url: self.url.clone(),
            commitment: solana_sdk::commitment_config::CommitmentConfig::confirmed(),
            program_id: soonshop_core::id(), // 使用 SoonShop 程序 ID
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::CreateUser(args) => {
            println!("创建用户: {}", args.username);
            println!("电子邮件: {}", args.email);
            println!("RPC URL: {}", args.rpc.url);
            // TODO: 实现创建用户逻辑
            println!("注意：创建用户功能还未实现");
        }
        Command::GetUser(args) => {
            println!("获取用户信息: {}", args.pubkey);
            println!("RPC URL: {}", args.rpc.url);
            // TODO: 实现获取用户信息逻辑
            println!("注意：获取用户信息功能还未实现");
        }
        Command::CreateProduct(args) => {
            println!("创建产品: {}", args.name);
            println!("描述: {}", args.description);
            println!("价格: {} lamports", args.price);
            println!("库存: {}", args.stock);
            println!("RPC URL: {}", args.rpc.url);
            // TODO: 实现创建产品逻辑
            println!("注意：创建产品功能还未实现");
        }
        Command::ListProducts(args) => {
            println!("列出产品");
            println!("RPC URL: {}", args.rpc.url);
            // TODO: 实现列出产品逻辑
            println!("注意：列出产品功能还未实现");
        }
        Command::PlaceOrder(args) => {
            println!("下单");
            println!("产品 ID: {}", args.product_id);
            println!("数量: {}", args.quantity);
            println!("RPC URL: {}", args.rpc.url);
            // TODO: 实现下单逻辑
            println!("注意：下单功能还未实现");
        }
        Command::ProgramId { rpc } => {
            println!("SoonShop 程序 ID: {}", soonshop_core::id());
            println!("RPC URL: {}", rpc.url);
        }
    }

    Ok(())
}

fn load_keypair(keypair_path: &str) -> Result<solana_sdk::signature::Keypair> {
    let keypair = solana_sdk::signature::read_keypair_file(keypair_path)
        .map_err(|e| anyhow::anyhow!("无法读取密钥文件 {}: {}", keypair_path, e))?;
    Ok(keypair)
}

fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey> {
    Pubkey::from_str(pubkey_str)
        .map_err(|e| anyhow::anyhow!("无法解析公钥 {}: {}", pubkey_str, e))
}
