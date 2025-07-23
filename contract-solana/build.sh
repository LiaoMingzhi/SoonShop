#!/bin/bash

# SoonShop Contract Solana Build Script
# 用于构建整个contract-solana项目

echo "🚀 开始构建 SoonShop Contract Solana 项目..."

# 设置环境变量
export RUST_LOG=info

# 检查必要的工具
check_tools() {
    echo "📋 检查必要的工具..."
    
    if ! command -v anchor &> /dev/null; then
        echo "❌ Anchor CLI 未安装，请先安装: npm install -g @project-serum/anchor-cli"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo "❌ Cargo 未安装，请先安装 Rust"
        exit 1
    fi
    
    if ! command -v solana &> /dev/null; then
        echo "❌ Solana CLI 未安装，请先安装 Solana CLI"
        exit 1
    fi
    
    echo "✅ 所有必要工具已安装"
}

# 构建智能合约
build_programs() {
    echo "🏗️  构建智能合约程序..."
    
    # 构建 Anchor 项目
    echo "📦 构建 Anchor 项目..."
    anchor build
    
    if [ $? -eq 0 ]; then
        echo "✅ Anchor 项目构建成功"
    else
        echo "❌ Anchor 项目构建失败"
        exit 1
    fi
}

# 构建 Rust 客户端
build_rust_client() {
    echo "🦀 构建 Rust 客户端..."
    
    cd client
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo "✅ Rust 客户端构建成功"
    else
        echo "❌ Rust 客户端构建失败"
        exit 1
    fi
    
    cd ..
}

# 构建 bin 模块
build_bin_modules() {
    echo "🔧 构建 bin 模块..."
    
    # 构建所有 bin 模块
    local bin_modules=("cli" "keeper" "liquidator" "service-mango-crank" "service-mango-fills" "service-mango-health" "service-mango-orderbook" "service-mango-pnl" "settler")
    
    for module in "${bin_modules[@]}"; do
        echo "📦 构建 $module..."
        cd bin/$module
        cargo build --release
        
        if [ $? -eq 0 ]; then
            echo "✅ $module 构建成功"
        else
            echo "❌ $module 构建失败"
            exit 1
        fi
        
        cd ../..
    done
}

# 构建 TypeScript SDK
build_typescript_sdk() {
    echo "📝 构建 TypeScript SDK..."
    
    # 安装依赖
    echo "📦 安装 TypeScript 依赖..."
    npm install
    
    if [ $? -eq 0 ]; then
        echo "✅ TypeScript 依赖安装成功"
    else
        echo "❌ TypeScript 依赖安装失败"
        exit 1
    fi
    
    # 构建 TypeScript
    echo "🔨 编译 TypeScript..."
    npm run build
    
    if [ $? -eq 0 ]; then
        echo "✅ TypeScript SDK 构建成功"
    else
        echo "❌ TypeScript SDK 构建失败"
        exit 1
    fi
}

# 运行测试
run_tests() {
    echo "🧪 运行测试..."
    
    # 运行 Anchor 测试
    echo "🧪 运行 Anchor 测试..."
    anchor test
    
    if [ $? -eq 0 ]; then
        echo "✅ Anchor 测试通过"
    else
        echo "❌ Anchor 测试失败"
        exit 1
    fi
    
    # 运行 Rust 测试
    echo "🧪 运行 Rust 测试..."
    cargo test --all
    
    if [ $? -eq 0 ]; then
        echo "✅ Rust 测试通过"
    else
        echo "❌ Rust 测试失败"
        exit 1
    fi
}

# 主函数
main() {
    echo "🎯 SoonShop Contract Solana 构建脚本"
    echo "===================================="
    
    check_tools
    build_programs
    build_rust_client
    build_bin_modules
    build_typescript_sdk
    
    if [ "$1" = "--test" ]; then
        run_tests
    fi
    
    echo ""
    echo "🎉 构建完成！"
    echo "📁 构建输出位置："
    echo "   - 智能合约: target/deploy/"
    echo "   - Rust 客户端: client/target/release/"
    echo "   - bin 模块: bin/*/target/release/"
    echo "   - TypeScript SDK: dist/"
    echo ""
    echo "🚀 接下来可以运行部署脚本或进行测试"
}

# 运行主函数
main "$@" 