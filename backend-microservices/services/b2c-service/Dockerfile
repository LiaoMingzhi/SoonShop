FROM rust:1.88-slim as builder

WORKDIR /app

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./
COPY services/b2c-service/Cargo.toml ./services/b2c-service/
COPY shared/ ./shared/

# 构建依赖
RUN cargo build --release --bin b2c-service

# 复制源代码
COPY services/b2c-service/src ./services/b2c-service/src

# 构建应用
RUN cargo build --release --bin b2c-service

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# 创建用户
RUN useradd -m -u 1000 appuser

# 复制二进制文件
COPY --from=builder /app/target/release/b2c-service /usr/local/bin/b2c-service

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 8010

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8010/api/v1/b2c/health || exit 1

# 启动应用
CMD ["b2c-service"] 