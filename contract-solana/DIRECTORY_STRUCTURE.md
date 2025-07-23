# SoonShop Solana智能合约目录结构说明

本文档详细说明了SoonShop Solana智能合约项目的目录结构和文件组织方式。

## 📁 完整目录结构

```
contract-solana/
├── 📄 .gitignore                          # Git忽略规则
├── 📄 Anchor.toml                         # Anchor项目配置文件
├── 📄 Cargo.toml                          # Rust工作空间配置
├── 📄 package.json                        # TypeScript SDK包配置
├── 📄 tsconfig.json                       # TypeScript编译配置
├── 📄 README.md                           # 项目说明文档
├── 📄 DIRECTORY_STRUCTURE.md              # 目录结构说明（本文件）
├── 📄 LICENSE                             # MIT许可证
├── 📄 CONTRIBUTING.md                     # 贡献指南
├── 📄 CHANGELOG.md                        # 版本变更日志
├── 📄 SECURITY.md                         # 安全政策说明
│
├── 📁 programs/                           # 智能合约程序目录
│   ├── 📁 soonshop-core/                 # 核心智能合约
│   │   ├── 📄 Cargo.toml                 # 核心合约依赖配置
│   │   ├── 📄 Xargo.toml                 # Solana编译配置
│   │   └── 📁 src/                       # 核心合约源码
│   │
│   ├── 📁 mango-v4/                      # Mango v4 DeFi协议合约
│   │   ├── 📄 Cargo.toml                 # Mango v4依赖配置
│   │   ├── 📄 Xargo.toml                 # Solana编译配置
│   │   ├── 📁 src/                       # Mango v4源码
│   │   │   ├── 📄 lib.rs                 # Mango v4合约入口
│   │   │   ├── 📁 instructions/          # 80+交易指令
│   │   │   ├── 📁 state/                 # 状态管理
│   │   │   ├── 📁 health/                # 健康度计算
│   │   │   └── 📄 error.rs               # 错误定义
│   │   ├── 📁 tests/                     # 完整测试套件
│   │   └── 📁 resources/                 # 测试资源文件
│   │       ├── 📄 lib.rs                 # 合约入口点和主程序
│   │       ├── 📁 instructions/          # 指令实现模块
│   │       │   ├── 📄 mod.rs             # 指令模块入口
│   │       │   ├── 📁 platform/          # 平台管理指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 initialize.rs  # 平台初始化
│   │       │   │   └── 📄 config.rs      # 配置管理
│   │       │   ├── 📁 voucher/           # 提货券指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 issue.rs       # 发布提货券
│   │       │   │   ├── 📄 claim.rs       # 获取提货券
│   │       │   │   ├── 📄 consume.rs     # 消费提货券
│   │       │   │   └── 📄 verify.rs      # 验证消费
│   │       │   ├── 📁 multiplier/        # 倍增奖励指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 calculate.rs   # 倍增计算
│   │       │   │   ├── 📄 distribute.rs  # 奖励分发
│   │       │   │   └── 📄 propagate.rs   # 链式传递
│   │       │   ├── 📁 evaluation/        # 企业评估指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 submit.rs      # 提交评估
│   │       │   │   └── 📄 update.rs      # 更新倍增系数
│   │       │   ├── 📁 price/             # 价格监控指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 update.rs      # 更新价格数据
│   │       │   │   └── 📄 detect.rs      # 异常检测
│   │       │   ├── 📁 scenarios/         # 应用场景指令
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 b2c.rs         # B2C电商
│   │       │   │   ├── 📄 restaurant.rs  # 餐饮服务
│   │       │   │   ├── 📄 healthcare.rs  # 医疗服务
│   │       │   │   ├── 📄 housing.rs     # 住房租赁
│   │       │   │   └── 📄 education.rs   # 教育培训
│   │       │   └── 📁 admin/             # 管理指令
│   │       │       ├── 📄 mod.rs
│   │       │       ├── 📄 pause.rs       # 紧急暂停
│   │       │       └── 📄 upgrade.rs     # 程序升级
│   │       ├── 📁 state/                 # 状态管理模块
│   │       │   ├── 📄 mod.rs             # 状态模块入口
│   │       │   ├── 📁 platform/          # 平台状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 config.rs      # 平台配置
│   │       │   │   └── 📄 statistics.rs  # 统计数据
│   │       │   ├── 📁 user/              # 用户状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 account.rs     # 用户账户
│   │       │   │   └── 📄 profile.rs     # 用户资料
│   │       │   ├── 📁 voucher/           # 提货券状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 voucher.rs     # 提货券结构
│   │       │   │   ├── 📄 claim.rs       # 获取记录
│   │       │   │   └── 📄 product.rs     # 商品信息
│   │       │   ├── 📁 consumption/       # 消费状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 record.rs      # 消费记录
│   │       │   │   └── 📄 proof.rs       # 消费证明
│   │       │   ├── 📁 evaluation/        # 评估状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 enterprise.rs  # 企业评估
│   │       │   │   └── 📄 criteria.rs    # 评估标准
│   │       │   ├── 📁 price/             # 价格状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 data.rs        # 价格数据
│   │       │   │   └── 📄 history.rs     # 历史价格
│   │       │   ├── 📁 scenarios/         # 场景状态
│   │       │   │   ├── 📄 mod.rs
│   │       │   │   ├── 📄 b2c.rs         # B2C状态
│   │       │   │   ├── 📄 restaurant.rs  # 餐饮状态
│   │       │   │   ├── 📄 healthcare.rs  # 医疗状态
│   │       │   │   ├── 📄 housing.rs     # 住房状态
│   │       │   │   └── 📄 education.rs   # 教育状态
│   │       │   └── 📁 common/            # 通用状态
│   │       │       ├── 📄 mod.rs
│   │       │       ├── 📄 types.rs       # 通用类型
│   │       │       └── 📄 constants.rs   # 常量定义
│   │       ├── 📄 errors.rs              # 错误定义
│   │       ├── 📄 events.rs              # 事件定义
│   │       ├── 📄 utils.rs               # 工具函数
│   │       └── 📄 constants.rs           # 常量定义
│   │
│   ├── 📁 soonshop-voucher/              # 提货券专用合约
│   │   ├── 📄 Cargo.toml
│   │   ├── 📄 Xargo.toml
│   │   └── 📁 src/
│   │       ├── 📄 lib.rs                 # 提货券合约入口
│   │       ├── 📁 instructions/          # 提货券专用指令
│   │       ├── 📁 state/                 # 提货券状态管理
│   │       ├── 📄 errors.rs              # 提货券错误定义
│   │       └── 📄 events.rs              # 提货券事件定义
│   │
│   ├── 📁 soonshop-evaluation/           # 企业评估合约
│   │   ├── 📄 Cargo.toml
│   │   ├── 📄 Xargo.toml
│   │   └── 📁 src/
│   │       ├── 📄 lib.rs                 # 评估合约入口
│   │       ├── 📁 instructions/          # 评估专用指令
│   │       ├── 📁 state/                 # 评估状态管理
│   │       ├── 📄 errors.rs              # 评估错误定义
│   │       └── 📄 events.rs              # 评估事件定义
│   │
│   └── 📁 soonshop-price/                # 价格监控合约
│       ├── 📄 Cargo.toml
│       ├── 📄 Xargo.toml
│       └── 📁 src/
│           ├── 📄 lib.rs                 # 价格合约入口
│           ├── 📁 instructions/          # 价格专用指令
│           ├── 📁 state/                 # 价格状态管理
│           ├── 📄 errors.rs              # 价格错误定义
│           └── 📄 events.rs              # 价格事件定义
│
├── 📁 client/                            # Rust客户端SDK
│   ├── 📄 Cargo.toml                     # 客户端依赖配置
│   ├── 📄 README.md                      # 客户端SDK文档
│   └── 📁 src/
│       ├── 📄 lib.rs                     # SDK入口
│       ├── 📄 client.rs                  # 主客户端实现
│       ├── 📄 config.rs                  # 配置管理
│       ├── 📁 instructions/              # 指令封装
│       │   ├── 📄 mod.rs
│       │   ├── 📄 platform.rs            # 平台指令
│       │   ├── 📄 voucher.rs             # 提货券指令
│       │   ├── 📄 multiplier.rs          # 倍增指令
│       │   ├── 📄 evaluation.rs          # 评估指令
│       │   ├── 📄 price.rs               # 价格指令
│       │   └── 📄 scenarios.rs           # 场景指令
│       ├── 📁 accounts/                  # 账户管理
│       │   ├── 📄 mod.rs
│       │   ├── 📄 fetcher.rs             # 账户获取器
│       │   └── 📄 resolver.rs            # 账户解析器
│       ├── 📁 events/                    # 事件监听
│       │   ├── 📄 mod.rs
│       │   ├── 📄 listener.rs            # 事件监听器
│       │   └── 📄 parser.rs              # 事件解析器
│       ├── 📁 utils/                     # 工具函数
│       │   ├── 📄 mod.rs
│       │   ├── 📄 keypair.rs             # 密钥对工具
│       │   ├── 📄 transaction.rs         # 交易工具
│       │   └── 📄 serialization.rs       # 序列化工具
│       └── 📁 examples/                  # 使用示例
│           ├── 📄 basic_usage.rs         # 基础用法
│           ├── 📄 voucher_flow.rs        # 提货券流程
│           └── 📄 evaluation_flow.rs     # 评估流程
│
├── 📁 bin/                               # 可执行二进制程序
│   ├── 📁 cli/                           # 命令行接口工具
│   │   ├── 📄 Cargo.toml                 # CLI工具依赖配置
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # CLI主程序
│   │       ├── 📄 save_snapshot.rs       # 快照保存工具
│   │       └── 📄 test_oracles.rs        # 预言机测试
│   ├── 📁 keeper/                        # 守护进程
│   │   ├── 📄 Cargo.toml                 # 守护进程依赖配置
│   │   ├── 📄 Dockerfile.keeper          # Docker配置
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 守护进程主程序
│   │       ├── 📄 crank.rs               # 曲柄机制
│   │       └── 📄 taker.rs               # 接受者模块
│   ├── 📁 liquidator/                    # 清算器
│   │   ├── 📄 Cargo.toml                 # 清算器依赖配置
│   │   ├── 📄 Dockerfile.liquidator      # Docker配置
│   │   ├── 📄 README.md                  # 清算器文档
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 清算器主程序
│   │       ├── 📄 liquidate.rs           # 清算逻辑
│   │       └── 📄 liquidation_state.rs   # 清算状态
│   ├── 📁 service-mango-crank/           # Mango曲柄服务
│   │   ├── 📄 Cargo.toml
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 服务主程序
│   │       └── 📄 mango_v4_perp_crank_sink.rs # 期货曲柄
│   ├── 📁 service-mango-fills/           # Mango成交服务
│   │   ├── 📄 Cargo.toml
│   │   ├── 📄 README.md
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 服务主程序
│   │       └── 📄 fill_event_filter.rs   # 成交事件过滤
│   ├── 📁 service-mango-health/          # Mango健康度服务
│   │   ├── 📄 Cargo.toml
│   │   ├── 📄 README.md
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 服务主程序
│   │       └── 📄 health.rs              # 健康度计算
│   ├── 📁 service-mango-orderbook/       # Mango订单簿服务
│   │   ├── 📄 Cargo.toml
│   │   ├── 📄 README.md
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 服务主程序
│   │       └── 📄 orderbook_filter.rs    # 订单簿过滤
│   ├── 📁 service-mango-pnl/             # Mango PnL服务
│   │   ├── 📄 Cargo.toml
│   │   └── 📁 src/
│   │       ├── 📄 main.rs                # 服务主程序
│   │       └── 📄 memory_target.rs       # 内存目标
│   └── 📁 settler/                       # 结算器
│       ├── 📄 Cargo.toml                 # 结算器依赖配置
│       ├── 📄 Dockerfile.settler         # Docker配置
│       └── 📁 src/
│           ├── 📄 main.rs                # 结算器主程序
│           ├── 📄 settle.rs              # 结算逻辑
│           └── 📄 metrics.rs             # 监控指标
│
├── 📁 src/                               # TypeScript SDK源码
│   ├── 📄 index.ts                       # TypeScript SDK入口
│   ├── 📁 client/                        # 客户端实现
│   │   ├── 📄 index.ts                   # 客户端入口
│   │   ├── 📄 SoonShopClient.ts          # 主客户端类
│   │   ├── 📄 config.ts                  # 配置管理
│   │   └── 📄 connection.ts              # 连接管理
│   ├── 📁 instructions/                  # 指令封装
│   │   ├── 📄 index.ts
│   │   ├── 📄 platform.ts                # 平台指令
│   │   ├── 📄 voucher.ts                 # 提货券指令
│   │   ├── 📄 multiplier.ts              # 倍增指令
│   │   ├── 📄 evaluation.ts              # 评估指令
│   │   ├── 📄 price.ts                   # 价格指令
│   │   └── 📄 scenarios.ts               # 场景指令
│   ├── 📁 accounts/                      # 账户管理
│   │   ├── 📄 index.ts
│   │   ├── 📄 fetcher.ts                 # 账户获取器
│   │   ├── 📄 resolver.ts                # 账户解析器
│   │   └── 📄 parser.ts                  # 账户解析器
│   ├── 📁 types/                         # 类型定义
│   │   ├── 📄 index.ts                   # 类型入口
│   │   ├── 📄 platform.ts                # 平台类型
│   │   ├── 📄 voucher.ts                 # 提货券类型
│   │   ├── 📄 user.ts                    # 用户类型
│   │   ├── 📄 consumption.ts             # 消费类型
│   │   ├── 📄 evaluation.ts              # 评估类型
│   │   ├── 📄 price.ts                   # 价格类型
│   │   ├── 📄 scenarios.ts               # 场景类型
│   │   └── 📄 common.ts                  # 通用类型
│   ├── 📁 events/                        # 事件处理
│   │   ├── 📄 index.ts
│   │   ├── 📄 listener.ts                # 事件监听器
│   │   ├── 📄 parser.ts                  # 事件解析器
│   │   └── 📄 types.ts                   # 事件类型
│   ├── 📁 utils/                         # 工具函数
│   │   ├── 📄 index.ts
│   │   ├── 📄 keypair.ts                 # 密钥对工具
│   │   ├── 📄 transaction.ts             # 交易工具
│   │   ├── 📄 serialization.ts           # 序列化工具
│   │   ├── 📄 validation.ts              # 验证工具
│   │   └── 📄 constants.ts               # 常量定义
│   ├── 📁 errors/                        # 错误处理
│   │   ├── 📄 index.ts
│   │   ├── 📄 SoonShopError.ts           # 自定义错误类
│   │   └── 📄 errorCodes.ts              # 错误码定义
│   └── 📁 __tests__/                     # 单元测试
│       ├── 📄 setup.ts                   # 测试设置
│       ├── 📄 client.test.ts             # 客户端测试
│       ├── 📄 instructions.test.ts       # 指令测试
│       ├── 📄 accounts.test.ts           # 账户测试
│       ├── 📄 events.test.ts             # 事件测试
│       └── 📄 utils.test.ts              # 工具测试
│
├── 📁 tests/                             # 集成测试
│   ├── 📄 Cargo.toml                     # 测试依赖配置
│   ├── 📄 integration_tests.rs           # 集成测试主文件
│   ├── 📄 performance_tests.rs           # 性能测试
│   ├── 📄 security_tests.rs              # 安全测试
│   ├── 📁 common/                        # 测试公共模块
│   │   ├── 📄 mod.rs
│   │   ├── 📄 setup.rs                   # 测试环境设置
│   │   ├── 📄 fixtures.rs                # 测试夹具
│   │   └── 📄 helpers.rs                 # 测试助手函数
│   ├── 📁 platform/                      # 平台功能测试
│   │   ├── 📄 mod.rs
│   │   ├── 📄 initialization.rs          # 初始化测试
│   │   └── 📄 configuration.rs           # 配置测试
│   ├── 📁 voucher/                       # 提货券功能测试
│   │   ├── 📄 mod.rs
│   │   ├── 📄 issue.rs                   # 发布测试
│   │   ├── 📄 claim.rs                   # 获取测试
│   │   ├── 📄 consume.rs                 # 消费测试
│   │   └── 📄 lifecycle.rs               # 生命周期测试
│   ├── 📁 multiplier/                    # 倍增功能测试
│   │   ├── 📄 mod.rs
│   │   ├── 📄 calculation.rs             # 计算测试
│   │   ├── 📄 distribution.rs            # 分发测试
│   │   └── 📄 propagation.rs             # 传递测试
│   ├── 📁 evaluation/                    # 评估功能测试
│   │   ├── 📄 mod.rs
│   │   ├── 📄 submission.rs              # 提交测试
│   │   └── 📄 scoring.rs                 # 评分测试
│   ├── 📁 price/                         # 价格功能测试
│   │   ├── 📄 mod.rs
│   │   ├── 📄 monitoring.rs              # 监控测试
│   │   └── 📄 detection.rs               # 检测测试
│   └── 📁 scenarios/                     # 场景功能测试
│       ├── 📄 mod.rs
│       ├── 📄 b2c.rs                     # B2C测试
│       ├── 📄 restaurant.rs              # 餐饮测试
│       ├── 📄 healthcare.rs              # 医疗测试
│       ├── 📄 housing.rs                 # 住房测试
│       └── 📄 education.rs               # 教育测试
│
├── 📁 target/                            # 编译输出目录
│   ├── 📁 deploy/                        # 程序部署文件
│   │   ├── 📄 soonshop_core.so           # 核心合约二进制
│   │   ├── 📄 soonshop_voucher.so        # 提货券合约二进制
│   │   ├── 📄 soonshop_evaluation.so     # 评估合约二进制
│   │   ├── 📄 soonshop_price.so          # 价格合约二进制
│   │   └── 📁 keypairs/                  # 程序密钥对
│   │       ├── 📄 soonshop_core-keypair.json
│   │       ├── 📄 soonshop_voucher-keypair.json
│   │       ├── 📄 soonshop_evaluation-keypair.json
│   │       └── 📄 soonshop_price-keypair.json
│   ├── 📁 idl/                           # IDL文件
│   │   ├── 📄 soonshop_core.json         # 核心合约IDL
│   │   ├── 📄 soonshop_voucher.json      # 提货券合约IDL
│   │   ├── 📄 soonshop_evaluation.json   # 评估合约IDL
│   │   └── 📄 soonshop_price.json        # 价格合约IDL
│   ├── 📁 types/                         # 生成的类型文件
│   └── 📁 debug/                         # 调试版本输出
│
├── 📁 lib/                               # TypeScript编译输出
│   ├── 📄 index.js                       # 编译后的入口文件
│   ├── 📄 index.d.ts                     # 类型声明文件
│   ├── 📄 index.mjs                      # ES模块版本
│   └── 📁 ...                            # 其他编译输出
│
├── 📁 migrations/                        # 部署脚本
│   ├── 📄 deploy.ts                      # 主部署脚本
│   ├── 📄 initialize.ts                  # 初始化脚本
│   ├── 📄 upgrade.ts                     # 升级脚本
│   └── 📁 environments/                  # 环境配置
│       ├── 📄 localnet.json              # 本地网络配置
│       ├── 📄 devnet.json                # 开发网络配置
│       └── 📄 mainnet.json               # 主网配置
│
├── 📁 scripts/                           # 工具脚本
│   ├── 📄 build.sh                       # 构建脚本
│   ├── 📄 test.sh                        # 测试脚本
│   ├── 📄 deploy.sh                      # 部署脚本
│   ├── 📄 clean.sh                       # 清理脚本
│   ├── 📁 generators/                    # 代码生成器
│   │   ├── 📄 idl-to-types.ts            # IDL转类型生成器
│   │   └── 📄 client-generator.ts        # 客户端生成器
│   ├── 📁 validators/                    # 验证脚本
│   │   ├── 📄 program-validator.ts       # 程序验证器
│   │   └── 📄 account-validator.ts       # 账户验证器
│   └── 📁 utilities/                     # 实用工具
│       ├── 📄 keypair-manager.ts         # 密钥对管理
│       ├── 📄 airdrop.ts                 # 空投工具
│       └── 📄 account-viewer.ts          # 账户查看器
│
├── 📁 docs/                              # 文档目录
│   ├── 📄 api.md                         # API文档
│   ├── 📄 architecture.md                # 架构文档
│   ├── 📄 deployment.md                  # 部署文档
│   ├── 📄 development.md                 # 开发文档
│   ├── 📄 testing.md                     # 测试文档
│   ├── 📄 security.md                    # 安全文档
│   ├── 📁 examples/                      # 示例文档
│   │   ├── 📄 basic-usage.md             # 基础用法
│   │   ├── 📄 advanced-features.md       # 高级功能
│   │   └── 📄 integration.md             # 集成指南
│   ├── 📁 tutorials/                     # 教程
│   │   ├── 📄 getting-started.md         # 入门教程
│   │   ├── 📄 voucher-system.md          # 提货券系统教程
│   │   └── 📄 evaluation-system.md       # 评估系统教程
│   └── 📁 generated/                     # 自动生成的文档
│       ├── 📄 typedoc/                   # TypeDoc生成的文档
│       └── 📄 rustdoc/                   # RustDoc生成的文档
│
├── 📁 config/                            # 配置文件目录
│   ├── 📄 development.toml               # 开发环境配置
│   ├── 📄 testing.toml                   # 测试环境配置
│   ├── 📄 production.toml                # 生产环境配置
│   ├── 📄 jest.config.js                 # Jest测试配置
│   ├── 📄 jest.integration.config.js     # 集成测试配置
│   ├── 📄 rollup.config.js               # Rollup打包配置
│   └── 📄 .env.example                   # 环境变量示例
│
├── 📁 .github/                           # GitHub配置
│   ├── 📁 workflows/                     # GitHub Actions工作流
│   │   ├── 📄 ci.yml                     # 持续集成
│   │   ├── 📄 cd.yml                     # 持续部署
│   │   ├── 📄 security.yml               # 安全检查
│   │   └── 📄 release.yml                # 发布流程
│   ├── 📄 PULL_REQUEST_TEMPLATE.md       # PR模板
│   ├── 📄 ISSUE_TEMPLATE.md              # Issue模板
│   └── 📄 CODEOWNERS                     # 代码所有者
│
└── 📁 .vscode/                           # VS Code配置
    ├── 📄 settings.json                  # 编辑器设置
    ├── 📄 launch.json                    # 调试配置
    ├── 📄 tasks.json                     # 任务配置
    └── 📄 extensions.json                # 推荐扩展
```

## 📝 目录说明

### 🏗️ 核心结构

#### `programs/` - 智能合约程序
- **soonshop-core**: 平台核心合约，包含所有主要业务逻辑
- **soonshop-voucher**: 提货券专用合约，优化提货券操作性能
- **soonshop-evaluation**: 企业评估合约，独立处理评估逻辑
- **soonshop-price**: 价格监控合约，实时价格数据处理

#### `client/` - Rust客户端SDK
- 提供完整的Rust语言API封装
- 支持异步操作和事件监听
- 包含丰富的使用示例和工具函数

#### `src/` - TypeScript客户端SDK
- Web应用和Node.js项目的首选SDK
- 完整的类型定义和IDE支持
- 现代JavaScript/TypeScript开发体验

### 🧪 测试体系

#### `tests/` - 综合测试套件
- **集成测试**: 完整业务流程测试
- **性能测试**: 高并发和大数据量测试
- **安全测试**: 攻击场景和边界条件测试
- **单元测试**: 分布在各模块的`__tests__`目录

### 📚 文档体系

#### `docs/` - 完整文档
- **API文档**: 详细的接口说明
- **架构文档**: 系统设计和技术选型
- **教程**: 从入门到高级的学习路径
- **示例**: 实际使用场景和代码示例

### 🛠️ 开发工具

#### `scripts/` - 开发脚本
- **构建工具**: 自动化编译和打包
- **部署工具**: 多环境部署自动化
- **代码生成器**: IDL到类型的自动转换
- **验证工具**: 程序和账户状态验证

#### `migrations/` - 部署管理
- **环境配置**: 不同网络的部署参数
- **升级脚本**: 智能合约升级流程
- **初始化脚本**: 平台首次部署设置

### ⚙️ 配置管理

#### `config/` - 配置文件
- **环境配置**: 开发、测试、生产环境参数
- **工具配置**: 测试、打包、代码检查配置
- **示例配置**: 快速开始的模板文件

## 🎯 设计原则

### 1. **模块化设计**
- 每个功能模块独立开发和测试
- 清晰的依赖关系和接口定义
- 便于维护和扩展

### 2. **分层架构**
- 智能合约层：核心业务逻辑
- SDK层：客户端接口封装
- 应用层：具体业务实现

### 3. **多语言支持**
- Rust SDK：系统级应用和服务端开发
- TypeScript SDK：Web应用和现代前端开发
- 统一的API设计和文档

### 4. **完整的工具链**
- 从开发到部署的全流程工具
- 自动化测试和质量保证
- 丰富的示例和文档

### 5. **安全优先**
- 严格的权限控制和验证
- 全面的测试覆盖
- 安全审计和漏洞检测

## 🚀 快速导航

### 新手开始
1. 阅读 [README.md](README.md) 了解项目概况
2. 查看 [docs/getting-started.md](docs/tutorials/getting-started.md) 快速上手
3. 运行 [examples/](client/src/examples/) 中的示例代码

### 开发者
1. 研读 [docs/architecture.md](docs/architecture.md) 了解系统架构
2. 查看 [programs/soonshop-core/src/](programs/soonshop-core/src/) 核心合约实现
3. 使用 [client/](client/) 或 [src/](src/) 中的SDK进行开发

### 部署运维
1. 参考 [docs/deployment.md](docs/deployment.md) 部署指南
2. 使用 [migrations/](migrations/) 中的部署脚本
3. 查看 [scripts/](scripts/) 中的运维工具

---

这个目录结构设计体现了现代软件开发的最佳实践，为SoonShop项目提供了坚实的技术基础和良好的开发体验。 