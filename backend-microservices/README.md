# SoonShop 后端微服务系统

一个基于Rust构建的现代化微服务电商平台后端系统，采用领域驱动设计(DDD)和事件驱动架构。使用 SeaORM、PostgreSQL 17.0、Redis 7.0.15 和 ElasticSearch 8.7.1。

## 📋 目录结构

```
backend-microservices/
├── services/                          # 微服务目录
│   ├── user-service/                  # 用户服务
│   ├── product-service/               # 商品服务
│   ├── order-service/                 # 订单服务
│   ├── payment-service/               # 支付服务
│   ├── inventory-service/             # 库存服务
│   ├── notification-service/          # 通知服务
│   └── analytics-service/             # 分析服务
├── shared/                            # 共享库
│   ├── common/                        # 通用工具
│   ├── events/                        # 事件定义
│   └── schemas/                       # 数据模式
├── gateway/                           # API网关
├── infrastructure/                    # 基础设施
│   ├── docker/                        # Docker配置
│   ├── kubernetes/                    # K8s配置
│   ├── monitoring/                    # 监控配置
│   └── databases/                     # 数据库配置
├── tools/                             # 工具脚本
├── docs/                              # 文档
└── tests/                             # 集成测试
```

## 🚀 核心技术栈

### 后端技术
- **Rust 1.88.0** - 系统编程语言
- **Actix-web 4.11** - Web框架
- **SeaORM 1.1** - ORM框架 (简单、灵活且高性能)
- **PostgreSQL 17.0** - 主数据库
- **Redis 7.0.15** - 缓存和会话存储
- **RabbitMQ 3.11** - 消息队列
- **ElasticSearch 8.7.1** - 搜索引擎

### 基础设施
- **Docker & Docker Compose** - 容器化
- **Kubernetes** - 容器编排
- **Prometheus & Grafana** - 监控
- **Jaeger** - 分布式追踪
- **Consul** - 服务发现

## 🏗️ 架构特点

### 微服务架构
- **服务拆分基于业务域**: 每个服务独立负责特定的业务领域
- **独立部署和扩展**: 每个服务可以独立部署、扩展和升级
- **服务间通过API通信**: 使用RESTful API和事件进行服务间通信
- **数据库分库分表**: 每个服务管理自己的数据存储

### 事件驱动架构
- **异步事件处理**: 使用RabbitMQ进行异步事件处理
- **消息队列解耦**: 通过消息队列实现服务解耦
- **事件溯源机制**: 支持事件溯源和重放
- **最终一致性保证**: 通过事件确保数据的最终一致性

### 分布式架构
- **多节点部署**: 支持多节点水平扩展
- **水平扩展能力**: 通过负载均衡实现水平扩展
- **故障转移机制**: 内置健康检查和故障转移
- **数据分片策略**: 支持数据库分片和读写分离

## 🛠️ 快速开始

### 环境要求
- Rust 1.88.0+
- Docker & Docker Compose
- PostgreSQL 17.0+
- Redis 7.0.15+
- ElasticSearch 8.7.1+
- SeaORM CLI (用于数据库迁移)

### 一键启动开发环境
```bash
# 克隆项目
git clone <repository-url>
cd backend-microservices

# 安装开发依赖
make install-dev

# 启动开发环境 (一键启动所有服务)
make dev
```

### 手动启动步骤
```bash
# 1. 启动基础设施 (PostgreSQL, Redis, ElasticSearch)
make docker-up

# 2. 运行数据库迁移
make migrate

# 3. 构建项目
make build

# 4. 启动服务
make run-user-service &
make run-product-service &
```

## 📊 服务列表

| 服务名称 | 端口 | 描述 | 状态 |
|---------|------|------|------|
| API Gateway | 8000 | API网关和负载均衡 | ✅ |
| User Service | 8001 | 用户认证和管理 | ✅ |
| Product Service | 8002 | 商品管理和搜索 | ✅ |
| Order Service | 8003 | 订单处理 | 🚧 |
| Payment Service | 8004 | 支付处理 | 🚧 |
| Inventory Service | 8005 | 库存管理 | 🚧 |
| Notification Service | 8006 | 通知服务 | 🚧 |
| Analytics Service | 8007 | 数据分析 | 🚧 |

## 🔧 开发工具

### 常用命令
```bash
# 构建项目
make build

# 运行测试
make test

# 格式化代码
make format

# 代码检查
make lint

# 健康检查
make health-check

# 查看日志
make logs
```

### 数据库操作
```bash
# 设置数据库
make db-setup

# 运行迁移
make migrate

# 重置数据库
make db-reset
```

## 📡 API文档

### 用户服务 API
```
POST   /users              # 创建用户
GET    /users/:id          # 获取用户
PUT    /users/:id          # 更新用户
DELETE /users/:id          # 删除用户
GET    /users              # 用户列表
POST   /auth/login         # 用户登录
POST   /auth/refresh       # 刷新Token
```

### 产品服务 API
```
POST   /products           # 创建产品
GET    /products/:id       # 获取产品
PUT    /products/:id       # 更新产品
DELETE /products/:id       # 删除产品
GET    /products           # 产品列表
GET    /products/search    # 搜索产品
```

## 🔐 安全性

### 认证和授权
- JWT Token认证
- 基于角色的访问控制(RBAC)
- API密钥管理
- OAuth2.0集成

### 数据保护
- 数据加密传输(TLS)
- 敏感数据脱敏
- 数据备份和恢复
- 审计日志记录

## 📈 监控和日志

### 监控指标
- 服务健康状态
- 响应时间和吞吐量
- 错误率和成功率
- 资源使用情况

### 日志管理
- 结构化日志
- 分布式追踪
- 日志聚合和搜索
- 异常告警

## 🚀 部署

### Docker部署
```bash
# 构建镜像
make docker-build

# 启动容器
make docker-up
```

### Kubernetes部署
```bash
# 部署到K8s
kubectl apply -f infrastructure/kubernetes/
```

## 🧪 测试

### 单元测试
```bash
# 运行单元测试
make test
```

### 集成测试
```bash
# 运行集成测试
cargo test --test integration
```

### 性能测试
```bash
# 运行性能测试
make bench
```

## 📚 文档

- [架构设计文档](docs/architecture.md)
- [API接口文档](docs/api.md)
- [数据库设计文档](docs/database.md)
- [部署指南](docs/deployment.md)
- [开发指南](docs/development.md)

## 🤝 贡献指南

1. Fork项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建Pull Request

## 📄 许可证

本项目采用MIT许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 👥 团队

- **架构师**: 负责系统架构设计
- **后端开发**: 负责微服务开发
- **运维工程师**: 负责基础设施管理
- **测试工程师**: 负责质量保证

## 📧 联系方式

- 邮箱: team@soonshop.com
- 项目地址: https://github.com/soonshop/backend-microservices
- 文档地址: https://docs.soonshop.com

---

⭐ 如果您觉得这个项目有帮助，请给它一个星标！ 