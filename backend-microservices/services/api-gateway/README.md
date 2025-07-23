# API网关服务

SoonShop微服务架构的API网关，提供统一的API入口、认证授权、限流熔断、监控日志等功能。

## 功能特性

### 🚪 统一API入口
- 路由转发到各个微服务
- 请求/响应转换和适配
- 服务发现和负载均衡
- 故障转移和熔断保护

### 🔐 认证授权
- JWT Token认证
- 基于角色的访问控制(RBAC)
- 刷新Token机制
- 多种认证方式支持

### 🛡️ 安全防护
- 请求限流和防刷
- CORS跨域配置
- 请求体大小限制
- 恶意请求检测

### 📊 监控告警
- Prometheus指标收集
- 请求链路追踪
- 实时监控面板
- 异常告警机制

### 📝 日志管理
- 结构化日志记录
- 请求响应日志
- 错误日志采集
- 日志聚合分析

## 快速开始

### 环境要求

- Rust 1.88.0+
- Redis 6.0+
- PostgreSQL 13+ (可选)

### 安装依赖

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone <repository-url>
cd backend-microservices/services/api-gateway
```

### 配置环境

```bash
# 复制环境变量配置
cp .env.example .env

# 编辑配置文件
nano .env
```

### 运行服务

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/api-gateway
```

## 配置说明

### 环境变量

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `HOST` | `0.0.0.0` | 服务监听地址 |
| `PORT` | `8000` | 服务端口 |
| `JWT_SECRET` | - | JWT密钥 |
| `REDIS_URL` | `redis://localhost:6379` | Redis连接地址 |
| `RATE_LIMIT_RPM` | `60` | 每分钟请求限制 |
| `RATE_LIMIT_BURST` | `10` | 突发请求限制 |

### 服务地址配置

网关需要配置各个微服务的地址：

```bash
USER_SERVICE_URL=http://localhost:8001
PRODUCT_SERVICE_URL=http://localhost:8002
ORDER_SERVICE_URL=http://localhost:8003
PAYMENT_SERVICE_URL=http://localhost:8004
VOUCHER_SERVICE_URL=http://localhost:8005
REWARD_SERVICE_URL=http://localhost:8006
EVALUATION_SERVICE_URL=http://localhost:8007
NOTIFICATION_SERVICE_URL=http://localhost:8008
```

## API文档

### 健康检查

```http
GET /health
```

**响应示例：**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "service": "api-gateway"
}
```

### 认证接口

#### 用户登录

```http
POST /auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password"
}
```

**响应示例：**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 86400,
  "user": {
    "id": "1",
    "username": "admin",
    "email": "admin@soonshop.com",
    "role": "admin"
  }
}
```

#### 用户注册

```http
POST /auth/register
Content-Type: application/json

{
  "username": "newuser",
  "email": "user@example.com",
  "password": "password123",
  "wallet_address": "0x..."
}
```

### 代理接口

所有业务接口都通过网关代理到相应的微服务：

- `/api/users/*` → 用户服务
- `/api/products/*` → 产品服务
- `/api/orders/*` → 订单服务
- `/api/payments/*` → 支付服务
- `/api/vouchers/*` → 提货券服务
- `/api/rewards/*` → 奖励服务
- `/api/evaluations/*` → 评估服务
- `/api/notifications/*` → 通知服务

## 监控指标

### Prometheus指标

```http
GET /metrics
```

主要指标：

- `api_gateway_http_requests_total` - 总请求数
- `api_gateway_http_request_duration_seconds` - 请求延迟
- `api_gateway_http_requests_in_flight` - 并发请求数

### 链路追踪

支持Jaeger链路追踪，可以查看请求在各个服务间的调用链路。

## 部署说明

### Docker部署

```bash
# 构建镜像
docker build -t soonshop/api-gateway .

# 运行容器
docker run -p 8000:8000 \
  -e JWT_SECRET=your-secret \
  -e REDIS_URL=redis://redis:6379 \
  soonshop/api-gateway
```

### Kubernetes部署

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: api-gateway
        image: soonshop/api-gateway:latest
        ports:
        - containerPort: 8000
        env:
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: api-gateway-secret
              key: jwt-secret
```

## 故障排除

### 常见问题

1. **服务启动失败**
   - 检查端口是否被占用
   - 确认Redis连接正常
   - 验证环境变量配置

2. **认证失败**
   - 检查JWT密钥配置
   - 确认Token格式正确
   - 验证Token过期时间

3. **代理失败**
   - 检查目标服务是否正常运行
   - 确认服务地址配置正确
   - 查看网络连接状态

### 日志查看

```bash
# 查看实时日志
tail -f logs/api-gateway.log

# 查看错误日志
grep ERROR logs/api-gateway.log
```

## 贡献指南

1. Fork项目
2. 创建功能分支
3. 提交代码
4. 创建Pull Request

## 许可证

MIT License 