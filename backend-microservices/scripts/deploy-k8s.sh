#!/bin/bash

# SoonShop微服务Kubernetes部署脚本
# 作者: SoonShop团队
# 描述: 在Kubernetes集群中部署整个微服务架构

set -e

echo "🚀 开始在Kubernetes中部署SoonShop微服务架构..."

# 检查kubectl是否安装
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl未安装，请先安装kubectl"
    exit 1
fi

# 检查是否连接到Kubernetes集群
if ! kubectl cluster-info &> /dev/null; then
    echo "❌ 无法连接到Kubernetes集群，请检查配置"
    exit 1
fi

# 显示当前集群信息
echo "🔍 当前Kubernetes集群信息:"
kubectl cluster-info

# 确认部署
echo ""
read -p "是否继续在此集群中部署？(y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 部署已取消"
    exit 1
fi

# 创建命名空间
echo "📝 创建命名空间..."
kubectl apply -f k8s/namespace.yaml

# 等待命名空间创建
echo "⏳ 等待命名空间创建..."
kubectl wait --for=condition=Ready namespace/soonshop --timeout=30s

# 部署基础设施服务
echo "🏗️  部署基础设施服务..."
kubectl apply -f k8s/infrastructure.yaml

# 等待基础设施服务就绪
echo "⏳ 等待PostgreSQL就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/postgres -n soonshop

echo "⏳ 等待Redis就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/redis -n soonshop

echo "⏳ 等待RabbitMQ就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/rabbitmq -n soonshop

echo "⏳ 等待ElasticSearch就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/elasticsearch -n soonshop

# 部署监控服务
echo "📊 部署监控服务..."
kubectl apply -f k8s/monitoring.yaml

# 等待监控服务就绪
echo "⏳ 等待Prometheus就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/prometheus -n soonshop

echo "⏳ 等待Grafana就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/grafana -n soonshop

echo "⏳ 等待Jaeger就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/jaeger -n soonshop

# 部署微服务
echo "🚀 部署微服务..."
kubectl apply -f k8s/services.yaml

# 等待微服务就绪
echo "⏳ 等待API Gateway就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/api-gateway -n soonshop

echo "⏳ 等待用户服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/user-service -n soonshop

echo "⏳ 等待产品服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/product-service -n soonshop

echo "⏳ 等待订单服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/order-service -n soonshop

echo "⏳ 等待支付服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/payment-service -n soonshop

echo "⏳ 等待库存服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/inventory-service -n soonshop

echo "⏳ 等待通知服务就绪..."
kubectl wait --for=condition=available --timeout=300s deployment/notification-service -n soonshop

# 部署Ingress（可选）
if [ -f k8s/ingress.yaml ]; then
    echo "🌐 部署Ingress控制器..."
    kubectl apply -f k8s/ingress.yaml
fi

# 显示部署状态
echo ""
echo "🔍 检查部署状态..."
kubectl get all -n soonshop

# 显示服务端点
echo ""
echo "🌐 服务端点:"
kubectl get services -n soonshop

# 获取外部访问地址
echo ""
echo "🔗 外部访问地址:"

# API Gateway
API_GATEWAY_IP=$(kubectl get service api-gateway -n soonshop -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "")
if [ -n "$API_GATEWAY_IP" ]; then
    echo "  - API Gateway: http://$API_GATEWAY_IP:8000"
else
    echo "  - API Gateway: 使用 'kubectl port-forward service/api-gateway 8000:8000 -n soonshop' 进行本地访问"
fi

# 监控服务
echo ""
echo "📊 监控服务本地访问命令:"
echo "  - Grafana: kubectl port-forward service/grafana 4000:4000 -n soonshop"
echo "  - Prometheus: kubectl port-forward service/prometheus 9090:9090 -n soonshop"
echo "  - Jaeger: kubectl port-forward service/jaeger 16686:16686 -n soonshop"
echo "  - Kibana: kubectl port-forward service/kibana 5601:5601 -n soonshop"

# 显示有用的命令
echo ""
echo "🔧 有用的管理命令:"
echo "  - 查看Pod状态: kubectl get pods -n soonshop"
echo "  - 查看日志: kubectl logs -f deployment/[service-name] -n soonshop"
echo "  - 进入Pod: kubectl exec -it deployment/[service-name] -n soonshop -- /bin/sh"
echo "  - 删除部署: kubectl delete namespace soonshop"
echo "  - 扩容服务: kubectl scale deployment [service-name] --replicas=3 -n soonshop"

# 显示密钥更新提示
echo ""
echo "⚠️  安全提示:"
echo "请更新以下Kubernetes密钥中的敏感信息:"
echo "  kubectl edit secret microservices-secret -n soonshop"
echo ""

# 检查健康状态
echo "🏥 检查服务健康状态..."
for i in {1..30}; do
    if kubectl get pods -n soonshop | grep -q "Running"; then
        echo "✅ 发现运行中的Pod"
        break
    fi
    echo "⏳ 等待Pod启动... ($i/30)"
    sleep 5
done

# 显示Pod状态
echo ""
echo "📋 当前Pod状态:"
kubectl get pods -n soonshop

echo ""
echo "🎉 SoonShop微服务架构在Kubernetes中部署完成！"

# 创建快速访问脚本
cat > access-services.sh << 'EOF'
#!/bin/bash
echo "🌐 SoonShop服务快速访问脚本"
echo "选择要访问的服务:"
echo "1) API Gateway (8000)"
echo "2) Grafana (4000)"
echo "3) Prometheus (9090)"
echo "4) Jaeger (16686)"
echo "5) Kibana (5601)"

read -p "请选择 (1-5): " choice

case $choice in
    1) kubectl port-forward service/api-gateway 8000:8000 -n soonshop ;;
    2) kubectl port-forward service/grafana 4000:4000 -n soonshop ;;
    3) kubectl port-forward service/prometheus 9090:9090 -n soonshop ;;
    4) kubectl port-forward service/jaeger 16686:16686 -n soonshop ;;
    5) kubectl port-forward service/kibana 5601:5601 -n soonshop ;;
    *) echo "无效选择" ;;
esac
EOF

chmod +x access-services.sh
echo "📝 已创建 access-services.sh 脚本，运行它可以快速访问服务" 