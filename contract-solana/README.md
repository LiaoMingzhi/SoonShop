# SoonShop Solana智能合约 🌟

[![Solana](https://img.shields.io/badge/Solana-2.2.19-9945FF?logo=solana)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.31.1-00D4AA)](https://anchor-lang.com)
[![Rust](https://img.shields.io/badge/Rust-1.88.0-000000?logo=rust)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-Passing-brightgreen)](tests/)

> 基于共产主义经济原理的区块链商业平台智能合约 - 实现共同富裕、按需消费、按需生产的良性循环经济体系

## 📋 目录

- [概述](#概述)
- [核心功能](#核心功能)
- [技术架构](#技术架构)
- [快速开始](#快速开始)
- [项目结构](#项目结构)
- [部署指南](#部署指南)
- [API文档](#api文档)
- [开发指南](#开发指南)
- [测试](#测试)
- [安全性](#安全性)
- [许可证](#许可证)

## 🎯 概述

SoonShop是全球首个基于共产主义经济原理的区块链商业平台，通过Solana高性能区块链技术实现：

- **🤝 共同富裕**: 通过倍增机制实现财富公平分配
- **🛒 按需消费**: 消费者可免费获取生活必需品
- **🏭 按需生产**: 生产者根据需求获得生产动力和倍增奖励
- **♻️ 良性循环**: 消费推动生产，生产获得倍增，形成可持续发展

## ⚡ 核心功能

### 🎫 提货券系统
- **智能发行**: 企业根据存货和产能发布提货券
- **免费获取**: 消费者按需免费获取提货券
- **灵活消费**: 支持现货、预约、服务等多种消费模式
- **自动核销**: 消费完成后自动触发核销和奖励机制

### 🚀 倍增奖励机制
- **动态计算**: 基于企业评估的2-100倍奖励倍增
- **职工分红**: 至少50%奖励分配给职工，实现共同富裕
- **链式传递**: 奖励自动传递给上游供应商
- **实时结算**: 区块链确保奖励即时到账

### 📊 企业评估体系
- **多维评估**: 产品质量、服务质量、职工福利、环保、安全、价值观等6个维度
- **透明公正**: 区块链确保评估过程透明且不可篡改
- **动态调整**: 根据评估结果实时调整倍增系数
- **持续改进**: 激励企业持续提升综合表现

### 💰 价格稳定系统
- **实时监控**: 24/7监控商品价格波动
- **智能预警**: 检测异常价格变动和操纵行为
- **自动干预**: 通过倍增系数调整引导价格稳定
- **通胀控制**: 防止系统性通胀风险

### 🌐 多场景应用
- **B2C电商**: 传统电商模式升级
- **餐饮服务**: 餐厅预约和消费服务
- **医疗健康**: 医疗服务预约和管理
- **住房租赁**: 住房申请和租赁管理
- **教育培训**: 教育资源共享和技能提升

## 🏗️ 技术架构

```
┌─────────────────────────────────────────────────────────────┐
│                    前端应用层                                │
│     React/Vue.js + TypeScript + Web3 钱包集成               │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                 客户端SDK层                                  │
│   TypeScript SDK + Rust SDK + API封装 + 事件监听            │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                Solana智能合约层                              │
│  核心合约 + 提货券合约 + 评估合约 + 价格合约 (Anchor框架)     │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                  Solana区块链                               │
│     高性能(65K TPS) + 低费用 + 最终性确认                   │
└─────────────────────────────────────────────────────────────┘
```

### 核心组件

- **soonshop-core**: 核心智能合约程序
- **soonshop-voucher**: 提货券专用合约
- **soonshop-evaluation**: 企业评估合约
- **soonshop-price**: 价格监控合约
- **client**: Rust客户端SDK
- **TypeScript SDK**: Web应用集成SDK
- **tests**: 综合测试套件

## 🚀 快速开始

### 环境要求

- **Rust**: 1.88.0+
- **Solana CLI**: 2.2.19+
- **Anchor CLI**: 0.31.1+
- **Node.js**: 18.0.0+
- **TypeScript**: 5.6.0+

### 安装依赖

```bash
# 安装Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v2.2.19/install)"

# 安装Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.31.1
avm use 0.31.1

# 安装Node.js依赖
npm install
```

### 编译合约

```bash
# 编译所有智能合约
anchor build

# 编译特定合约
anchor build --program-name soonshop-core
```

### 运行测试

```bash
# 运行集成测试
anchor test

# 运行单元测试
cargo test

# 运行TypeScript测试
npm test
```

### 本地部署

```bash
# 启动本地验证器
solana-test-validator

# 部署合约到本地网络
anchor deploy --provider.cluster localnet

# 初始化平台
anchor run initialize
```

## 📁 项目结构

```
contract-solana/
├── Anchor.toml                 # Anchor项目配置
├── Cargo.toml                  # Rust工作空间配置
├── package.json                # TypeScript SDK配置
├── tsconfig.json               # TypeScript编译配置
├── programs/                   # 智能合约程序
│   ├── soonshop-core/         # 核心合约
│   │   ├── src/
│   │   │   ├── lib.rs         # 程序入口
│   │   │   ├── instructions/  # 指令实现
│   │   │   ├── state/         # 状态管理
│   │   │   ├── errors.rs      # 错误定义
│   │   │   ├── events.rs      # 事件定义
│   │   │   └── utils.rs       # 工具函数
│   │   └── Cargo.toml
│   ├── soonshop-voucher/      # 提货券合约
│   ├── soonshop-evaluation/   # 评估合约
│   └── soonshop-price/        # 价格合约
├── client/                     # Rust客户端SDK
│   ├── src/
│   │   ├── lib.rs             # SDK入口
│   │   ├── client.rs          # 主客户端
│   │   ├── instructions/      # 指令封装
│   │   ├── accounts/          # 账户管理
│   │   └── utils/             # 工具函数
│   └── Cargo.toml
├── src/                        # TypeScript SDK源码
│   ├── index.ts               # SDK入口
│   ├── client/                # 客户端实现
│   ├── instructions/          # 指令封装
│   ├── accounts/              # 账户管理
│   ├── types/                 # 类型定义
│   ├── utils/                 # 工具函数
│   └── __tests__/            # 单元测试
├── tests/                      # 集成测试
│   ├── integration_tests.rs   # 集成测试
│   ├── performance_tests.rs   # 性能测试
│   ├── security_tests.rs      # 安全测试
│   └── Cargo.toml
├── target/                     # 编译输出
│   ├── deploy/                # 程序二进制文件
│   └── idl/                   # IDL文件
├── migrations/                 # 部署脚本
├── scripts/                    # 工具脚本
├── docs/                       # 文档
└── README.md                   # 项目说明
```

## 🚀 部署指南

### 本地网络部署

```bash
# 1. 启动本地验证器
solana-test-validator --reset

# 2. 设置Solana CLI配置
solana config set --url localhost
solana config set --keypair ~/.config/solana/id.json

# 3. 请求空投（测试代币）
solana airdrop 10

# 4. 编译和部署
anchor build
anchor deploy

# 5. 初始化平台
ts-node scripts/initialize-platform.ts
```

### Devnet部署

```bash
# 1. 切换到Devnet
solana config set --url devnet

# 2. 请求Devnet空投
solana airdrop 5

# 3. 部署到Devnet
anchor deploy --provider.cluster devnet

# 4. 验证部署
solana program show <PROGRAM_ID>
```

### Mainnet部署

```bash
# 1. 切换到Mainnet
solana config set --url mainnet-beta

# 2. 确保有足够的SOL余额
solana balance

# 3. 部署到Mainnet（谨慎操作）
anchor deploy --provider.cluster mainnet-beta

# 4. 验证和初始化
ts-node scripts/mainnet-initialization.ts
```

## 📚 API文档

### 核心指令

#### 平台管理

```typescript
// 初始化平台
await client.initializePlatform({
  config: {
    baseMultiplier: 10,
    maxMultiplier: 100,
    platformFeeRate: 0.01,
    // ... 其他配置
  }
});

// 更新平台配置
await client.updatePlatformConfig({
  newConfig: {
    // 新配置参数
  }
});
```

#### 提货券操作

```typescript
// 发布提货券
const voucherId = await client.issueVoucher({
  productInfo: {
    name: "优质大米",
    description: "东北五常大米",
    category: "食品",
    imageUrl: "https://example.com/rice.jpg"
  },
  voucherConfig: {
    quantity: 1000,
    price: 50,
    voucherType: "CurrentStock",
    expirationTime: Date.now() + 30 * 24 * 60 * 60 * 1000 // 30天
  },
  distributionRules: {
    maxPerUser: 5,
    geographicRestriction: {
      allowedRegions: ["北京", "上海", "广州"]
    }
  }
});

// 获取提货券
const claimId = await client.claimVoucher({
  voucherId: voucherId,
  quantity: 2
});

// 消费提货券
const consumptionId = await client.consumeVoucher({
  claimId: claimId,
  consumptionProof: {
    location: {
      latitude: 39.9042,
      longitude: 116.4074,
      address: "北京市朝阳区"
    },
    timestamp: Date.now()
  }
});

// 验证消费（生产者操作）
await client.verifyConsumption({
  consumptionId: consumptionId,
  qualityScore: 95
});
```

#### 企业评估

```typescript
// 提交企业评估
const evaluationId = await client.submitEnterpriseEvaluation({
  enterpriseId: "enterprise_pubkey",
  evaluationScores: {
    productQuality: 90,
    serviceQuality: 85,
    workerWelfare: 95,
    environmentalScore: 80,
    safetyScore: 92,
    ideologyScore: 88
  },
  evaluationDetails: {
    evaluationPeriod: "2024-Q1",
    evidenceDocuments: ["doc1.pdf", "doc2.pdf"],
    evaluatorNotes: "企业表现优秀，建议提升环保措施"
  }
});
```

### 事件监听

```typescript
// 监听提货券发布事件
client.addEventListener('VoucherIssued', (event) => {
  console.log('新提货券发布:', event.data);
});

// 监听倍增奖励分发事件
client.addEventListener('MultiplierRewardDistributed', (event) => {
  console.log('倍增奖励已分发:', event.data);
});

// 监听价格异常预警事件
client.addEventListener('PriceAnomalyDetected', (event) => {
  console.log('价格异常预警:', event.data);
});
```

## 🛠️ 开发指南

### 代码规范

- **Rust代码**: 遵循Rust官方代码规范，使用`rustfmt`格式化
- **TypeScript代码**: 遵循Prettier配置，使用ESLint检查
- **提交信息**: 使用Conventional Commits规范
- **文档**: 所有公开API必须有完整的文档注释

### 开发流程

1. **Fork项目**: 从主仓库fork到个人仓库
2. **创建分支**: 基于最新main分支创建feature分支
3. **开发功能**: 编写代码并添加相应测试
4. **运行测试**: 确保所有测试通过
5. **提交代码**: 使用规范的提交信息
6. **创建PR**: 详细描述变更内容和原因
7. **代码审查**: 等待维护者审查和反馈
8. **合并代码**: 审查通过后合并到主分支

### 调试技巧

```bash
# 启用详细日志
export RUST_LOG=debug
export ANCHOR_LOG=debug

# 使用Solana日志查看程序执行
solana logs --url localhost

# 查看账户数据
solana account <ACCOUNT_ADDRESS> --output json

# 分析交易详情
solana transaction <TRANSACTION_SIGNATURE>
```

## 🧪 测试

### 测试分类

- **单元测试**: 测试单个函数和模块
- **集成测试**: 测试完整的业务流程
- **性能测试**: 测试高并发和大数据量场景
- **安全测试**: 测试各种攻击场景和边界条件

### 运行测试

```bash
# 运行所有测试
npm run test:all

# 运行集成测试
npm run test:integration

# 运行性能测试
npm run test:performance

# 运行安全测试
npm run test:security

# 生成测试覆盖率报告
npm run test:coverage
```

### 测试示例

```typescript
describe('SoonShop Voucher System', () => {
  let client: SoonShopClient;
  let producer: Keypair;
  let consumer: Keypair;

  beforeAll(async () => {
    // 初始化测试环境
    client = new SoonShopClient(connection, wallet);
    producer = Keypair.generate();
    consumer = Keypair.generate();
  });

  test('should issue voucher successfully', async () => {
    const voucherId = await client.issueVoucher({
      // 测试数据
    });
    
    expect(voucherId).toBeDefined();
    
    const voucher = await client.getVoucher(voucherId);
    expect(voucher.status).toBe('Active');
  });

  test('should handle invalid voucher claim', async () => {
    await expect(client.claimVoucher({
      voucherId: 'invalid_id',
      quantity: 1
    })).rejects.toThrow('VoucherNotFound');
  });
});
```

## 🔒 安全性

### 安全特性

- **访问控制**: 基于角色的权限管理(RBAC)
- **签名验证**: 所有关键操作需要数字签名
- **溢出保护**: 数学运算采用安全检查
- **状态验证**: 严格的状态转换验证
- **重入保护**: 防止重入攻击
- **权限隔离**: 不同角色权限严格分离

### 安全审计

```bash
# 运行安全检查
cargo audit

# 代码质量检查
cargo clippy -- -D warnings

# 检查已知漏洞
npm audit

# 运行安全测试套件
npm run test:security
```

### 报告漏洞

如果发现安全漏洞，请：

1. **不要公开披露**: 请勿在公开场所讨论漏洞细节
2. **邮件报告**: 发送邮件至 security@soonshop.com
3. **详细描述**: 包含重现步骤和潜在影响
4. **等待回复**: 我们会在24小时内回复确认

## 📄 许可证

本项目采用MIT许可证 - 查看[LICENSE](LICENSE)文件了解详情。

## 🤝 贡献

我们欢迎所有形式的贡献！请查看[CONTRIBUTING.md](CONTRIBUTING.md)了解如何参与项目。

### 贡献者

感谢所有贡献者的辛勤工作！

<a href="https://github.com/soonshop/soonshop/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=soonshop/soonshop" />
</a>

## 📞 联系我们

- **官网**: https://soonshop.com
- **邮箱**: dev@soonshop.com
- **GitHub**: https://github.com/soonshop/soonshop
- **Discord**: https://discord.gg/soonshop
- **Twitter**: [@SoonShopDAO](https://twitter.com/SoonShopDAO)

## 🗺️ 路线图

- [x] **V1.0**: 核心提货券系统
- [x] **V1.1**: 倍增奖励机制
- [x] **V1.2**: 企业评估体系
- [ ] **V1.3**: 价格稳定系统
- [ ] **V1.4**: 多场景应用集成
- [ ] **V2.0**: 跨链支持
- [ ] **V2.1**: 去中心化治理
- [ ] **V3.0**: AI智能优化

---

<div align="center">
  <h3>🌟 为共产主义理想而编程 🌟</h3>
  <p>让技术服务人民，让代码改变世界！</p>
</div> 