# SoonShop Backend Microservices Makefile

.PHONY: help build run test clean docker-up docker-down migrate dev fmt lint check

# 默认目标
help:
	@echo "SoonShop Backend Microservices - Available commands:"
	@echo "  build         - 构建所有微服务"
	@echo "  run           - 运行所有微服务"
	@echo "  test          - 运行测试"
	@echo "  clean         - 清理构建缓存"
	@echo "  docker-up     - 启动Docker服务"
	@echo "  docker-down   - 停止Docker服务"
	@echo "  migrate       - 运行数据库迁移"
	@echo "  dev           - 开发模式运行"
	@echo "  fmt           - 格式化代码"
	@echo "  lint          - 代码检查"
	@echo "  check         - 检查代码"

# 构建所有微服务
build:
	@echo "构建所有微服务..."
	cargo build --release

# 运行所有微服务 (开发模式)
run:
	@echo "运行所有微服务..."
	cargo run --bin user-service &
	cargo run --bin product-service &
	wait

# 运行测试
test:
	@echo "运行测试..."
	cargo test

# 清理构建缓存
clean:
	@echo "清理构建缓存..."
	cargo clean

# 启动Docker服务
docker-up:
	@echo "启动Docker服务..."
	docker-compose up -d

# 停止Docker服务
docker-down:
	@echo "停止Docker服务..."
	docker-compose down

# 运行数据库迁移
migrate:
	@echo "运行数据库迁移..."
	cd migrations && sea-orm-cli migrate up

# 开发模式 (启动依赖服务 + 运行微服务)
dev: docker-up
	@echo "等待数据库启动..."
	sleep 10
	@echo "运行数据库迁移..."
	make migrate
	@echo "启动开发服务..."
	cargo run --bin user-service &
	cargo run --bin product-service &
	wait

# 格式化代码
fmt:
	@echo "格式化代码..."
	cargo fmt

# 代码检查
lint:
	@echo "代码检查..."
	cargo clippy -- -D warnings

# 检查代码 (编译检查)
check:
	@echo "检查代码..."
	cargo check

# 创建新的微服务模板
new-service:
	@read -p "请输入微服务名称: " service_name; \
	mkdir -p services/$$service_name-service/src; \
	echo "创建微服务: $$service_name-service"

# 运行特定的微服务
run-user-service:
	@echo "运行用户服务..."
	cargo run --bin user-service

run-product-service:
	@echo "运行产品服务..."
	cargo run --bin product-service

# 构建Docker镜像
docker-build:
	@echo "构建Docker镜像..."
	docker build -t soonshop/user-service:latest ./services/user-service/
	docker build -t soonshop/product-service:latest ./services/product-service/

# 运行完整的Docker Stack
docker-full: docker-build
	@echo "运行完整的Docker Stack..."
	docker-compose up -d

# 查看服务状态
status:
	@echo "查看服务状态..."
	docker-compose ps

# 查看服务日志
logs:
	@echo "查看服务日志..."
	docker-compose logs -f

# 数据库操作
db-reset:
	@echo "重置数据库..."
	docker-compose down postgres
	docker volume rm backend-microservices_postgres_data
	docker-compose up -d postgres

# 性能测试
perf-test:
	@echo "运行性能测试..."
	# 使用 wrk 或其他性能测试工具

# 备份数据
backup:
	@echo "备份数据..."
	docker exec soonshop-postgres pg_dump -U postgres soonshop > backup_$(shell date +%Y%m%d_%H%M%S).sql

# 恢复数据
restore:
	@read -p "请输入备份文件路径: " backup_file; \
	docker exec -i soonshop-postgres psql -U postgres soonshop < $$backup_file

# 环境变量检查
env-check:
	@echo "检查环境变量..."
	@echo "DATABASE_URL: $${DATABASE_URL:-未设置}"
	@echo "REDIS_URL: $${REDIS_URL:-未设置}"
	@echo "ELASTICSEARCH_URL: $${ELASTICSEARCH_URL:-未设置}"

# 安装开发依赖
install-dev:
	@echo "安装开发依赖..."
	cargo install sea-orm-cli
	cargo install cargo-watch
	cargo install cargo-audit

# 安全审计
audit:
	@echo "运行安全审计..."
	cargo audit

# 文档生成
docs:
	@echo "生成文档..."
	cargo doc --open

# 代码覆盖率
coverage:
	@echo "生成代码覆盖率报告..."
	cargo tarpaulin --out html 