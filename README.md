# Vault Protocol

A security-focused, non-custodial Bitcoin protocol that allows users to commit BTC without transferring custody while earning rewards from protocol-owned staking activities.

## Project Structure

```
vault-protocol/
├── programs/vault/              # Solana program (Rust/Anchor)
│   ├── src/
│   │   ├── instructions/        # Program instructions
│   │   ├── state/              # Account structures
│   │   ├── traits.rs           # Core interfaces
│   │   ├── errors.rs           # Error definitions
│   │   └── lib.rs              # Main program entry
│   ├── Cargo.toml
│   └── Anchor.toml
├── contracts/cosmos/           # Cosmos contracts (CosmWasm)
│   ├── src/
│   │   ├── contract.rs         # Main contract logic
│   │   ├── state.rs            # State management
│   │   ├── traits.rs           # ATOM staking interfaces
│   │   └── lib.rs
│   └── Cargo.toml
├── frontend/                   # NextJS frontend
│   ├── src/
│   │   ├── app/                # Next.js 13+ app directory
│   │   ├── components/         # React components
│   │   ├── lib/                # Utility libraries
│   │   ├── types/              # TypeScript definitions
│   │   └── i18n/               # Internationalization
│   ├── package.json
│   ├── next.config.js
│   └── tailwind.config.js
├── config/                     # Python configurations
│   ├── chainlink.py            # Oracle settings
│   └── validators.py           # Staking validators
├── tests/                      # Python test suite
│   └── test_btc_commitment.py  # BTC commitment tests
└── Anchor.toml                 # Root Anchor config
```

## Core Features

### 1. Non-Custodial BTC Commitment
- Users commit BTC without transferring custody
- ECDSA proof validation prevents spoofing
- Chainlink oracle verification every 60 seconds

### 2. Protocol Asset Staking
- SOL (40%), ETH (30%), ATOM (30%) allocation
- Cross-chain staking via Cosmos and Ethereum L2
- 1:2 reward ratio (user BTC commitment to staked assets)

### 3. Multi-Currency Rewards
- Default BTC payments via Lightning Network
- USDC payment option available
- Auto-reinvestment functionality

### 4. Enterprise Security
- 2-of-3 multisig with Yubico HSMs
- Optional KYC via Chainalysis (1 BTC limit without KYC)
- 2FA/passkey authentication required

### 5. Multi-Language Frontend
- English, Spanish, Chinese, Japanese support
- BlueWallet and Ledger integration
- Real-time dashboard with treasury data

## Development Setup

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor Framework 0.29+
- Node.js 18+
- Python 3.9+

### Installation

1. **Clone and setup Solana program:**
```bash
cd programs/vault
anchor build
anchor test
```

2. **Setup frontend:**
```bash
cd frontend
npm install
npm run dev
```

3. **Setup Python environment:**
```bash
pip install pytest pytest-asyncio
python -m pytest tests/ -v
```

### Configuration

Edit configuration files in `config/` directory:
- `chainlink.py`: Oracle feeds and verification settings
- `validators.py`: Staking validator selection and allocations

These files are designed to be editable by non-programmers.

## Testing

### Concurrent Python Testing
```bash
# Run all tests with ThreadPoolExecutor
python -m pytest tests/ -v --tb=short

# Run specific test file
python -m pytest tests/test_btc_commitment.py -v
```

### Solana Program Testing
```bash
cd programs/vault
anchor test
```

### Frontend Testing
```bash
cd frontend
npm run test
```

## Deployment

### Devnet Deployment
```bash
anchor build
anchor deploy --provider.cluster devnet
```

### Frontend Deployment
```bash
cd frontend
npm run build
npm run start
```

## Security Features

- **Multisig Security**: 2-of-3 multisig with Yubico HSM integration
- **Oracle Security**: ECDSA proof validation and Chainlink integration
- **Smart Contract Audits**: Certik audited with Slither scanning
- **KYC Compliance**: Chainalysis integration for regulatory compliance
- **2FA Authentication**: Required for all sensitive operations

## Resource Requirements

Optimized for low-resource systems:
- **RAM**: 8GB minimum
- **Storage**: 256GB minimum
- **Network**: Stable internet connection for oracle updates

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## Support

For technical support or questions:
- Create an issue in this repository
- Check the documentation in each module
- Review the test files for usage examples