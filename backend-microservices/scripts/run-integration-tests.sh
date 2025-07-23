#!/bin/bash

# SoonShop 微服务集成测试运行脚本
# 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/scripts/run-integration-tests.sh

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 SoonShop 微服务集成测试启动器${NC}"
echo "======================================"

# 检查Docker环境
check_docker() {
    echo -e "${YELLOW}📋 检查Docker环境...${NC}"
    
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker未安装${NC}"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        echo -e "${RED}❌ Docker守护进程未运行${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Docker环境正常${NC}"
}

# 启动测试环境
start_test_environment() {
    echo -e "${YELLOW}🚀 启动测试环境...${NC}"
    
    # 设置测试环境变量
    export DATABASE_URL="postgres://soonshop:soonshop123@localhost:5432/soonshop_test"
    export REDIS_URL="redis://localhost:6379/1"
    export RABBITMQ_URL="amqp://soonshop:soonshop123@localhost:5672"
    
    # 创建测试数据库
    echo -e "${YELLOW}📊 准备测试数据库...${NC}"
    docker-compose -f docker-compose.yml up -d postgres redis rabbitmq elasticsearch
    
    # 等待数据库启动
    echo -e "${YELLOW}⏳ 等待数据库服务启动...${NC}"
    sleep 15
    
    # 创建测试数据库
    docker exec soonshop-postgres psql -U soonshop -c "DROP DATABASE IF EXISTS soonshop_test;"
    docker exec soonshop-postgres psql -U soonshop -c "CREATE DATABASE soonshop_test;"
    
    echo -e "${GREEN}✅ 测试环境准备完成${NC}"
}

# 启动微服务
start_microservices() {
    echo -e "${YELLOW}🔧 启动微服务...${NC}"
    
    # 启动所有微服务
    docker-compose up -d
    
    # 等待服务启动
    echo -e "${YELLOW}⏳ 等待微服务启动...${NC}"
    sleep 30
    
    # 检查服务健康状态
    check_service_health() {
        local service_name=$1
        local url=$2
        local max_attempts=10
        local attempt=1
        
        while [ $attempt -le $max_attempts ]; do
            if curl -s "$url" > /dev/null 2>&1; then
                echo -e "${GREEN}✅ $service_name 已就绪${NC}"
                return 0
            fi
            echo -e "${YELLOW}⏳ 等待 $service_name 启动 (尝试 $attempt/$max_attempts)${NC}"
            sleep 3
            attempt=$((attempt + 1))
        done
        
        echo -e "${RED}❌ $service_name 启动失败${NC}"
        return 1
    }
    
    # 检查各个服务
    check_service_health "API Gateway" "http://localhost:8000/health"
    check_service_health "User Service" "http://localhost:8001/health"
    check_service_health "Product Service" "http://localhost:8002/health"
    check_service_health "Order Service" "http://localhost:8003/health"
    check_service_health "Payment Service" "http://localhost:8004/health"
    check_service_health "Inventory Service" "http://localhost:8009/health"
    check_service_health "Notification Service" "http://localhost:8008/health"
    
    echo -e "${GREEN}✅ 所有微服务已启动${NC}"
}

# 运行集成测试
run_tests() {
    echo -e "${YELLOW}🧪 运行集成测试...${NC}"
    
    cd tests
    
    # 设置测试环境变量
    export RUST_LOG=info
    export TEST_API_GATEWAY_URL="http://localhost:8000"
    export TEST_DATABASE_URL="postgres://soonshop:soonshop123@localhost:5432/soonshop_test"
    
    # 运行不同类型的测试
    case "${1:-all}" in
        "api-gateway"|"gateway")
            echo -e "${BLUE}🔍 运行API网关测试...${NC}"
            cargo test api_gateway_tests --release -- --nocapture
            ;;
        "e2e"|"end-to-end")
            echo -e "${BLUE}🔍 运行端到端测试...${NC}"
            cargo test e2e_tests --release -- --nocapture
            ;;
        "user")
            echo -e "${BLUE}🔍 运行用户服务测试...${NC}"
            cargo test user_service_tests --release -- --nocapture
            ;;
        "product")
            echo -e "${BLUE}🔍 运行商品服务测试...${NC}"
            cargo test product_service_tests --release -- --nocapture
            ;;
        "order")
            echo -e "${BLUE}🔍 运行订单服务测试...${NC}"
            cargo test order_service_tests --release -- --nocapture
            ;;
        "payment")
            echo -e "${BLUE}🔍 运行支付服务测试...${NC}"
            cargo test payment_service_tests --release -- --nocapture
            ;;
        "inventory")
            echo -e "${BLUE}🔍 运行库存服务测试...${NC}"
            cargo test inventory_service_tests --release -- --nocapture
            ;;
        "notification")
            echo -e "${BLUE}🔍 运行通知服务测试...${NC}"
            cargo test notification_service_tests --release -- --nocapture
            ;;
        "all"|*)
            echo -e "${BLUE}🔍 运行所有集成测试...${NC}"
            cargo test --release -- --nocapture
            ;;
    esac
    
    local test_result=$?
    cd ..
    
    if [ $test_result -eq 0 ]; then
        echo -e "${GREEN}✅ 测试通过！${NC}"
    else
        echo -e "${RED}❌ 测试失败！${NC}"
        return 1
    fi
}

# 清理测试环境
cleanup() {
    echo -e "${YELLOW}🧹 清理测试环境...${NC}"
    
    # 停止服务
    docker-compose down
    
    # 清理测试数据
    docker volume prune -f
    
    echo -e "${GREEN}✅ 清理完成${NC}"
}

# 显示使用说明
show_usage() {
    echo "使用方法: $0 [测试类型] [选项]"
    echo ""
    echo "测试类型:"
    echo "  all                运行所有测试 (默认)"
    echo "  api-gateway        运行API网关测试"
    echo "  e2e                运行端到端测试"
    echo "  user               运行用户服务测试"
    echo "  product            运行商品服务测试"
    echo "  order              运行订单服务测试"
    echo "  payment            运行支付服务测试"
    echo "  inventory          运行库存服务测试"
    echo "  notification       运行通知服务测试"
    echo ""
    echo "选项:"
    echo "  --no-setup         跳过环境启动（假设服务已运行）"
    echo "  --no-cleanup       跳过清理（保留测试环境）"
    echo "  --help             显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0                 # 运行所有测试"
    echo "  $0 e2e             # 只运行端到端测试"
    echo "  $0 api-gateway --no-cleanup  # 运行网关测试，不清理环境"
}

# 主函数
main() {
    local test_type="${1:-all}"
    local no_setup=false
    local no_cleanup=false
    
    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-setup)
                no_setup=true
                shift
                ;;
            --no-cleanup)
                no_cleanup=true
                shift
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                test_type=$1
                shift
                ;;
        esac
    done
    
    # 设置陷阱函数，确保在脚本退出时清理
    if [ "$no_cleanup" = false ]; then
        trap cleanup EXIT
    fi
    
    # 检查环境
    check_docker
    
    # 启动测试环境
    if [ "$no_setup" = false ]; then
        start_test_environment
        start_microservices
    fi
    
    # 运行测试
    run_tests "$test_type"
    
    echo -e "${GREEN}🎉 集成测试完成！${NC}"
}

# 运行主函数
main "$@" 