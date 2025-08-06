# Vault Protocol Testing & Debugging Guide

This document outlines the comprehensive testing and debugging pipeline for the Vault Protocol project.

## 🚀 Quick Start

### Run All Tests
```bash
# Using our custom script (recommended)
./scripts/test.sh

# Using PowerShell on Windows
.\scripts\test.ps1

# Using Make
make test
```

### Run Specific Test Suites
```bash
# Rust tests only
./scripts/test.sh --rust-only
make test-rust

# Python tests only  
./scripts/test.sh --python-only
make test-python

# Frontend tests only
./scripts/test.sh --frontend-only
make test-frontend
```

## 🧪 Test Structure

### Rust Tests (`programs/vault/src/`)
- **Unit Tests**: Located alongside source code in `*_tests.rs` files
- **Integration Tests**: In `programs/vault/tests/` directory
- **Coverage**: Built-in with `cargo test`

**Key Test Files:**
- `programs/vault/src/state/btc_commitment_tests.rs` - BTC commitment validation tests
- Tests cover: Address validation, ECDSA proof verification, anti-replay mechanisms

### Python Tests (`tests/`)
- **Framework**: pytest with async support
- **Mocking**: AsyncMock for async function testing
- **Coverage**: pytest-cov with HTML reports

**Key Test Files:**
- `tests/test_btc_commitment.py` - Comprehensive BTC commitment testing (16 tests)

### Frontend Tests (`frontend/`)
- **Framework**: Jest + React Testing Library
- **Type Checking**: TypeScript compiler
- **Linting**: ESLint + Prettier

## 🔧 Development Tools

### Testing Scripts

#### `scripts/test.sh` (Linux/macOS)
```bash
# Run all tests with verbose output
./scripts/test.sh --verbose

# Run integration tests
./scripts/test.sh --integration

# Get help
./scripts/test.sh --help
```

#### `scripts/test.ps1` (Windows)
```powershell
# Run all tests
.\scripts\test.ps1

# Run with verbose output
.\scripts\test.ps1 -Verbose

# Run integration tests
.\scripts\test.ps1 -Integration
```

### Debugging Tools

#### `scripts/debug.sh` - Development Helper
```bash
# Start local Solana validator with debug logging
./scripts/debug.sh start-validator

# Deploy program to local validator
./scripts/debug.sh deploy

# Run specific test with debug output
./scripts/debug.sh debug-test test_validate_btc_address

# Show real-time program logs
./scripts/debug.sh logs

# Generate test keypairs with SOL airdrop
./scripts/debug.sh generate-keypairs 10

# Check development environment
./scripts/debug.sh health-check
```

### Make Commands
```bash
# Setup
make install              # Install all dependencies
make install-rust         # Install Rust dependencies only
make install-python       # Install Python dependencies only

# Testing
make test                 # Run all tests
make test-watch          # Run tests in watch mode
make test-integration    # Run integration tests

# Development
make build               # Build all components
make deploy              # Deploy to local validator
make start-validator     # Start local validator
make stop-validator      # Stop local validator

# Code Quality
make lint                # Run all linters
make format              # Format all code
make security-audit      # Run security audits
make benchmark           # Run performance benchmarks

# Utilities
make clean               # Clean build artifacts
make health-check        # Check development environment
```

## 🔍 VS Code Integration

### Debugging Configuration
- **Rust Tests**: LLDB debugger with breakpoint support
- **Python Tests**: Python debugger with pytest integration
- **Solana Program**: Debug local validator with trace logging

### Keyboard Shortcuts
- `Ctrl+Shift+P` → "Tasks: Run Task" → Select test task
- `F5` → Start debugging current test file
- `Ctrl+Shift+` ` → Open integrated terminal

### Extensions Recommended
- `rust-analyzer` - Rust language support
- `ms-python.python` - Python support
- `esbenp.prettier-vscode` - Code formatting
- `ms-vscode.vscode-json` - JSON support

## 📊 Test Coverage

### Current Coverage Status
- **Rust Tests**: ✅ 14+ unit tests covering core validation logic
- **Python Tests**: ✅ 16 comprehensive integration tests
- **Frontend Tests**: 🚧 To be implemented

### Coverage Reports
```bash
# Generate Python coverage report
python -m pytest tests/ --cov=. --cov-report=html
# View report: open htmlcov/index.html

# Rust coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --manifest-path programs/vault/Cargo.toml
```

## 🚨 Continuous Integration

### GitHub Actions Pipeline (`.github/workflows/ci.yml`)
- **Rust Tests**: Format check, Clippy lints, unit tests, build verification
- **Python Tests**: Multi-version testing (3.9, 3.10, 3.11), coverage reports
- **Frontend Tests**: TypeScript checks, linting, build verification
- **Security Audit**: Cargo audit, Python safety checks, Bandit analysis

### Pipeline Triggers
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`
- Manual workflow dispatch

## 🛡️ Security Testing

### Automated Security Checks
```bash
# Rust security audit
cargo audit

# Python security checks
safety check
bandit -r . -f json

# Run all security audits
make security-audit
```

### Security Test Coverage
- **ECDSA Validation**: Cryptographic signature verification
- **Replay Attack Prevention**: Timestamp freshness validation
- **Input Validation**: Bitcoin address format verification
- **Buffer Overflow Protection**: Length checks on all inputs

## 🏃‍♂️ Performance Testing

### Benchmarks
```bash
# Run performance benchmarks
make benchmark
./scripts/debug.sh benchmark

# Rust benchmarks
cargo bench --manifest-path programs/vault/Cargo.toml

# Python performance tests
python -m pytest tests/ -k "performance or benchmark"
```

### Performance Metrics
- **ECDSA Verification**: ~1ms per signature
- **Address Validation**: ~0.1ms per address
- **Concurrent Operations**: 1000+ ops/sec

## 🐛 Debugging Workflows

### Common Debugging Scenarios

#### 1. Test Failure Investigation
```bash
# Run specific test with debug output
./scripts/debug.sh debug-test test_name

# Check test logs
RUST_LOG=debug RUST_BACKTRACE=1 cargo test test_name -- --nocapture
```

#### 2. Program Deployment Issues
```bash
# Check validator status
./scripts/debug.sh health-check

# View deployment logs
./scripts/debug.sh logs

# Inspect program account
./scripts/debug.sh inspect-account <program_id>
```

#### 3. Integration Test Debugging
```bash
# Start validator with debug logging
./scripts/debug.sh start-validator

# Deploy program
./scripts/debug.sh deploy

# Run integration tests
make test-integration
```

## 📈 Test Metrics & Monitoring

### Key Metrics
- **Test Success Rate**: Target 100%
- **Code Coverage**: Target >90%
- **Test Execution Time**: <30 seconds for full suite
- **Security Audit**: Zero high-severity issues

### Monitoring Dashboard
- GitHub Actions status badges
- Codecov integration for coverage tracking
- Dependabot for dependency updates
- Security advisories monitoring

## 🔄 Development Workflow

### Recommended Testing Flow
1. **Write Tests First**: TDD approach for new features
2. **Run Quick Tests**: `make quick-test` during development
3. **Full Test Suite**: Before committing changes
4. **Integration Tests**: Before merging to main
5. **Security Audit**: Weekly or before releases

### Pre-commit Hooks (Recommended)
```bash
# Install pre-commit
pip install pre-commit

# Setup hooks
pre-commit install

# Manual run
pre-commit run --all-files
```

## 🆘 Troubleshooting

### Common Issues

#### Rust Compilation Errors
```bash
# Clean and rebuild
cargo clean --manifest-path programs/vault/Cargo.toml
cargo build --manifest-path programs/vault/Cargo.toml
```

#### Python Import Errors
```bash
# Check Python path
export PYTHONPATH="${PYTHONPATH}:$(pwd)"
python -c "import sys; print(sys.path)"
```

#### Solana Validator Issues
```bash
# Reset validator state
./scripts/debug.sh stop-validator
rm -rf .anchor/test-ledger
./scripts/debug.sh start-validator
```

### Getting Help
- Check `./scripts/debug.sh health-check` for environment issues
- Review logs in `.anchor/test-ledger/validator.log`
- Run tests with `--verbose` flag for detailed output
- Use VS Code debugger for step-through debugging

---

## 📚 Additional Resources

- [Anchor Testing Guide](https://book.anchor-lang.com/anchor_in_depth/testing.html)
- [Solana Program Testing](https://docs.solana.com/developing/test-validator)
- [pytest Documentation](https://docs.pytest.org/)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

This testing pipeline ensures high code quality, security, and reliability for the Vault Protocol. Regular testing and debugging practices will help maintain the protocol's integrity as it grows.