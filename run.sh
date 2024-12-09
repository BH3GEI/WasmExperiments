#!/bin/bash

# 设置项目根目录
PROJECT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WEB_DIR="$PROJECT_DIR/web-test"

echo "🚀 Starting performance tests..."

# 杀掉可能存在的 Node.js 进程
echo "🔄 Cleaning up old processes..."
pkill -f "node.*vite"

# 编译 Rust 原生版本并运行
echo -e "\n📦 Building and running native Rust version..."
cd "$PROJECT_DIR"
cargo build --release
echo -e "\n🔬 Native Performance Test Results:"
cargo run --release

# 构建 WebAssembly
echo -e "\n🔧 Building WebAssembly..."
wasm-pack build --target web

# 复制 WebAssembly 文件到 web-test 目录
echo "📋 Copying WebAssembly files..."
rm -rf "$WEB_DIR/pkg"
cp -r pkg "$WEB_DIR/"

# 启动 web 服务器
echo -e "\n🌐 Starting web server..."
cd "$WEB_DIR"
npm run dev &

# 等待几秒钟让服务器启动
sleep 2

# 打开浏览器
echo "🌎 Opening browser..."
open http://localhost:5173

echo -e "\n✨ Setup complete!"
echo "🔍 Native results are shown above"
echo "🌐 Web tests are available at http://localhost:5173"
echo "⌨️  Press Ctrl+C to stop the web server when done"
