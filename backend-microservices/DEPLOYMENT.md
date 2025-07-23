# SoonShop微服务部署指南

本文档提供了SoonShop微服务架构的完整部署指南，包括Docker Compose和Kubernetes两种部署方式。

## 🏗️ 架构概览

SoonShop微服务架构包含以下组件：

### 核心微服务
- **API Gateway** (端口 8000) - 统一入口和路由
- **User Service** (端口 8001) - 用户管理服务
- **Product Service** (端口 8002) - 产品管理服务
- **Order Service** (端口 8003) - 订单管理服务
- **Payment Service** (端口 8004) - 支付处理服务
- **Inventory Service** (端口 8009) - 库存管理服务
- **Notification Service** (端口 8008) - 通知服务

### 基础设施服务
- **PostgreSQL** (端口 5432) - 主数据库
- **Redis** (端口 6379) - 缓存和会话存储
- **RabbitMQ** (端口 5672/15672) - 消息队列
- **ElasticSearch** (端口 9200/9300) - 搜索引擎

### 监控服务
- **Prometheus** (端口 9090) - 监控数据收集
- **Grafana** (端口 3000) - 可视化仪表板
- **Jaeger** (端口 16686) - 分布式链路追踪
- **Kibana** (端口 5601) - ElasticSearch可视化

## 🐋 Docker Compose部署

### 前置要求
- Docker 20.0+
- Docker Compose 2.0+
- 至少 4GB 可用内存
- 20GB 可用磁盘空间

### 快速部署

1. **使用部署脚本（推荐）**
```bash
# 确保脚本可执行
chmod +x scripts/deploy-docker.sh

# 运行部署脚本
./scripts/deploy-docker.sh
```

2. **手动部署**
```bash
# 创建环境变量文件
cp .env.example .env

# 编辑配置
nano .env

# 启动服务
docker-compose up -d
```

### 环境配置

编辑 `.env` 文件，设置以下关键配置：

```env
# 数据库配置
POSTGRES_DB=soonshop
POSTGRES_USER=soonshop
POSTGRES_PASSWORD=your-secure-password

# JWT密钥
JWT_SECRET=your-super-secret-jwt-key-change-in-production

# Stripe支付配置
STRIPE_SECRET_KEY=sk_test_your_stripe_secret_key

# PayPal支付配置
PAYPAL_CLIENT_ID=your_paypal_client_id
PAYPAL_CLIENT_SECRET=your_paypal_client_secret

# 邮件配置
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

### 访问服务

部署完成后，可以通过以下地址访问服务：

- **API Gateway**: http://localhost:8000
- **Grafana监控**: http://localhost:4000 (admin/admin123)
- **Prometheus**: http://localhost:9090
- **Jaeger追踪**: http://localhost:16686
- **Kibana**: http://localhost:5601
- **RabbitMQ管理**: http://localhost:15672 (soonshop/soonshop123)

### 常用命令

```bash
# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f [service-name]

# 重启服务
docker-compose restart [service-name]

# 停止所有服务
docker-compose down

# 删除所有数据
docker-compose down -v
```

## ☸️ Kubernetes部署

### 前置要求
- Kubernetes 1.20+
- kubectl 配置正确
- 至少 8GB 集群内存
- 50GB 可用存储空间
- Ingress控制器（可选）

### 快速部署

1. **使用部署脚本（推荐）**
```bash
# 确保脚本可执行
chmod +x scripts/deploy-k8s.sh

# 运行部署脚本
./scripts/deploy-k8s.sh
```

2. **手动部署**
```bash
# 创建命名空间
kubectl apply -f k8s/namespace.yaml

# 部署基础设施
kubectl apply -f k8s/infrastructure.yaml

# 部署监控服务
kubectl apply -f k8s/monitoring.yaml

# 部署微服务
kubectl apply -f k8s/services.yaml

# 部署Ingress（可选）
kubectl apply -f k8s/ingress.yaml
```

### 配置密钥

更新Kubernetes密钥中的敏感信息：

```bash
# 编辑微服务密钥
kubectl edit secret microservices-secret -n soonshop
```

将以下值替换为base64编码的实际值：
- `JWT_SECRET`
- `STRIPE_SECRET_KEY`
- `PAYPAL_CLIENT_ID`
- `PAYPAL_CLIENT_SECRET`
- `SMTP_PASSWORD`

### 本地访问服务

使用端口转发访问服务：

```bash
# API Gateway
kubectl port-forward service/api-gateway 8000:8000 -n soonshop

# Grafana
kubectl port-forward service/grafana 4000:4000 -n soonshop

# Prometheus
kubectl port-forward service/prometheus 9090:9090 -n soonshop

# Jaeger
kubectl port-forward service/jaeger 16686:16686 -n soonshop
```

### 常用命令

```bash
# 查看Pod状态
kubectl get pods -n soonshop

# 查看服务
kubectl get services -n soonshop

# 查看日志
kubectl logs -f deployment/[service-name] -n soonshop

# 扩容服务
kubectl scale deployment [service-name] --replicas=3 -n soonshop

# 删除部署
kubectl delete namespace soonshop
```

## 🔧 故障排除

### 常见问题

1. **服务无法启动**
   - 检查端口是否被占用
   - 确认Docker/Kubernetes资源充足
   - 查看服务日志排错

2. **数据库连接失败**
   - 确认PostgreSQL服务正常运行
   - 检查数据库连接字符串
   - 验证用户权限

3. **内存不足**
   - 调整服务副本数量
   - 增加系统内存
   - 优化JVM参数

4. **网络连接问题**
   - 检查防火墙设置
   - 验证服务发现配置
   - 确认DNS解析

### 日志分析

```bash
# Docker环境
docker-compose logs -f --tail=100 [service-name]

# Kubernetes环境
kubectl logs -f deployment/[service-name] -n soonshop --tail=100
```

### 性能监控

1. **Grafana仪表板**
   - 访问 http://localhost:4000
   - 使用 admin/admin123 登录
   - 查看预配置的微服务监控面板

2. **Prometheus指标**
   - 访问 http://localhost:9090
   - 查询自定义指标
   - 设置告警规则

3. **Jaeger链路追踪**
   - 访问 http://localhost:16686
   - 追踪请求调用链
   - 分析性能瓶颈

## 🚀 生产部署建议

### 安全配置
- 更换所有默认密码
- 使用强随机密钥
- 启用TLS加密
- 配置网络策略

### 高可用性
- 多副本部署
- 负载均衡配置
- 故障转移策略
- 备份恢复方案

### 性能优化
- 资源限制设置
- 缓存策略优化
- 数据库索引调优
- 监控告警配置

### 扩容策略
- 水平Pod自动扩容（HPA）
- 垂直Pod自动扩容（VPA）
- 集群自动扩容
- 数据库分片

## 📞 支持

如果遇到问题，请：

1. 查看日志文件
2. 检查监控指标
3. 参考故障排除指南
4. 联系技术支持团队

---

**维护团队**: SoonShop Backend Team  
**更新时间**: 2024年1月  
**版本**: v1.0.0 