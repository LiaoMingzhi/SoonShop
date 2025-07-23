#!/bin/bash

# SoonShop微服务Docker部署脚本
# 作者: SoonShop团队
# 描述: 一键部署整个微服务架构

set -e

echo "🚀 开始部署SoonShop微服务架构..."

# 检查Docker和Docker Compose是否安装
if ! command -v docker &> /dev/null; then
    echo "❌ Docker未安装，请先安装Docker"
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose未安装，请先安装Docker Compose"
    exit 1
fi

# 创建必要的目录
echo "📁 创建必要的目录..."
mkdir -p scripts
mkdir -p monitoring/grafana/dashboards
mkdir -p monitoring/grafana/datasources

# 创建初始化数据库脚本
echo "📝 创建数据库初始化脚本..."
cat > scripts/init-db.sql << 'EOF'
-- 创建扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- 创建用户（如果不存在）
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'soonshop') THEN
      CREATE ROLE soonshop LOGIN PASSWORD 'soonshop123';
   END IF;
END
$$;

-- 授予权限
GRANT ALL PRIVILEGES ON DATABASE soonshop TO soonshop;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO soonshop;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO soonshop;

-- 设置默认权限
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO soonshop;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO soonshop;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON FUNCTIONS TO soonshop;
EOF

# 检查环境变量
echo "🔍 检查环境变量..."
if [ ! -f .env ]; then
    echo "📝 创建.env文件..."
    cat > .env << 'EOF'
# 数据库配置
POSTGRES_DB=soonshop
POSTGRES_USER=soonshop
POSTGRES_PASSWORD=soonshop123

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

# 环境
RUST_LOG=info
ENV=development
EOF
    echo "⚠️  请编辑.env文件，设置正确的配置值"
fi

# 拉取最新镜像
echo "📥 拉取最新镜像..."
docker-compose pull

# 构建服务镜像
echo "🔨 构建微服务镜像..."
docker-compose build

# 启动基础设施服务
echo "🏗️  启动基础设施服务..."
docker-compose up -d postgres redis rabbitmq elasticsearch

# 等待数据库启动
echo "⏳ 等待数据库启动..."
sleep 30

# 启动监控服务
echo "📊 启动监控服务..."
docker-compose up -d prometheus grafana jaeger kibana

# 启动微服务
echo "🚀 启动微服务..."
docker-compose up -d api-gateway user-service product-service order-service payment-service inventory-service notification-service

# 检查服务状态
echo "🔍 检查服务状态..."
sleep 10
docker-compose ps

# 显示访问信息
echo ""
echo "✅ 部署完成！"
echo ""
echo "🌐 服务访问地址:"
echo "  - API Gateway: http://localhost:8000"
echo "  - Grafana监控: http://localhost:4000 (admin/admin123)"
echo "  - Prometheus: http://localhost:9090"
echo "  - Jaeger追踪: http://localhost:16686"
echo "  - Kibana可视化: http://localhost:5601"
echo "  - RabbitMQ管理: http://localhost:15672 (soonshop/soonshop123)"
echo ""
echo "🔧 管理命令:"
echo "  - 查看日志: docker-compose logs -f [service-name]"
echo "  - 停止服务: docker-compose down"
echo "  - 重启服务: docker-compose restart [service-name]"
echo "  - 清理数据: docker-compose down -v"
echo ""

# 检查健康状态
echo "🏥 检查服务健康状态..."
for i in {1..30}; do
    if curl -s http://localhost:8000/health > /dev/null; then
        echo "✅ API Gateway已就绪"
        break
    fi
    echo "⏳ 等待API Gateway启动... ($i/30)"
    sleep 2
done

echo "🎉 SoonShop微服务架构部署完成！" 