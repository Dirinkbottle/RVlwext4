.PHONY: help fmt fmt-check clippy test build clean all

# 默认目标
help:
	@echo "可用的命令："
	@echo "  make fmt         - 格式化代码"
	@echo "  make fmt-check   - 检查代码格式（不修改）"
	@echo "  make clippy      - 运行 clippy 代码检查"
	@echo "  make clippy-fix  - 运行 clippy 并自动修复"
	@echo "  make test        - 运行测试"
	@echo "  make build       - 编译项目"
	@echo "  make clean       - 清理构建文件"
	@echo "  make all         - 格式化 + clippy + 测试 + 编译"

# 格式化代码
fmt:
	@echo "🎨 格式化代码..."
	cargo fmt --all

# 检查代码格式
fmt-check:
	@echo "🔍 检查代码格式..."
	cargo fmt --all -- --check

# 运行 clippy（使用默认配置）
clippy:
	@echo "📎 运行 clippy 检查..."
	cargo clippy --all-targets --all-features

# 运行 clippy 并自动修复
clippy-fix:
	@echo "🔧 运行 clippy 自动修复..."
	cargo clippy --fix --allow-dirty --allow-staged

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test --lib

# 编译项目
build:
	@echo "🔨 编译项目..."
	cargo build --lib

# 清理
clean:
	@echo "🧹 清理构建文件..."
	cargo clean

# 完整检查流程
all: fmt clippy test build
	@echo "✅ 所有检查完成！"
