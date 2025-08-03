# Vault Protocol Development Makefile

.PHONY: help install test test-rust test-python test-frontend build deploy clean lint format security-audit benchmark

# Default target
help:
	@echo "Vault Protocol Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  install          Install all dependencies"
	@echo "  install-rust     Install Rust dependencies"
	@echo "  install-python   Install Python dependencies"
	@echo "  install-frontend Install frontend dependencies"
	@echo ""
	@echo "Testing:"
	@echo "  test             Run all tests"
	@echo "  test-rust        Run Rust tests only"
	@echo "  test-python      Run Python tests only"
	@echo "  test-frontend    Run frontend tests only"
	@echo "  test-integration Run integration tests"
	@echo "  test-watch       Run tests in watch mode"
	@echo ""
	@echo "Development:"
	@echo "  build            Build all components"
	@echo "  deploy           Deploy to local validator"
	@echo "  start-validator  Start local Solana validator"
	@echo "  stop-validator   Stop local Solana validator"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint             Run all linters"
	@echo "  format           Format all code"
	@echo "  security-audit   Run security audits"
	@echo "  benchmark        Run performance benchmarks"
	@echo ""
	@echo "Utilities:"
	@echo "  clean            Clean build artifacts"
	@echo "  health-check     Check development environment"

# Installation targets
install: install-rust install-python install-frontend
	@echo "✅ All dependencies installed"

install-rust:
	@echo "📦 Installing Rust dependencies..."
	cd programs/vault && cargo build
	@echo "✅ Rust dependencies installed"

install-python:
	@echo "📦 Installing Python dependencies..."
	python -m pip install --upgrade pip
	pip install pytest pytest-asyncio pytest-mock pytest-cov safety bandit
	@echo "✅ Python dependencies installed"

install-frontend:
	@echo "📦 Installing frontend dependencies..."
	cd frontend && npm install
	@echo "✅ Frontend dependencies installed"

# Testing targets
test:
	@echo "🧪 Running all tests..."
	./scripts/test.sh

test-rust:
	@echo "🦀 Running Rust tests..."
	./scripts/test.sh --rust-only

test-python:
	@echo "🐍 Running Python tests..."
	./scripts/test.sh --python-only

test-frontend:
	@echo "⚛️ Running frontend tests..."
	./scripts/test.sh --frontend-only

test-integration:
	@echo "🔗 Running integration tests..."
	./scripts/test.sh --integration

test-watch:
	@echo "👀 Running tests in watch mode..."
	cargo watch -x "test --manifest-path programs/vault/Cargo.toml --lib"

# Build targets
build:
	@echo "🔨 Building all components..."
	cd programs/vault && anchor build
	cd frontend && npm run build
	@echo "✅ Build complete"

deploy:
	@echo "🚀 Deploying to local validator..."
	./scripts/debug.sh deploy

start-validator:
	@echo "🟢 Starting local Solana validator..."
	./scripts/debug.sh start-validator

stop-validator:
	@echo "🔴 Stopping local Solana validator..."
	./scripts/debug.sh stop-validator

# Code quality targets
lint:
	@echo "🔍 Running linters..."
	cd programs/vault && cargo clippy --all-targets --all-features -- -D warnings
	python -m pytest tests/ --collect-only > /dev/null
	cd frontend && npm run lint
	@echo "✅ Linting complete"

format:
	@echo "✨ Formatting code..."
	cd programs/vault && cargo fmt --all
	cd frontend && npm run format
	@echo "✅ Formatting complete"

security-audit:
	@echo "🔒 Running comprehensive security audit..."
	./scripts/security-audit.sh

benchmark:
	@echo "⚡ Running benchmarks..."
	./scripts/debug.sh benchmark

# Utility targets
clean:
	@echo "🧹 Cleaning build artifacts..."
	cd programs/vault && cargo clean
	cd frontend && rm -rf .next node_modules/.cache
	rm -rf .anchor/test-ledger
	rm -rf htmlcov/
	rm -f .coverage
	rm -f bandit-report.json
	@echo "✅ Clean complete"

health-check:
	@echo "🏥 Checking development environment..."
	./scripts/debug.sh health-check

# Development shortcuts
dev: start-validator deploy
	@echo "🚀 Development environment ready!"

quick-test: test-rust test-python
	@echo "⚡ Quick tests complete!"

full-check: lint security-audit test
	@echo "✅ Full check complete!"