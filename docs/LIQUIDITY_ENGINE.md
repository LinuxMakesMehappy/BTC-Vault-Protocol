# Primitive Liquidity Engine

## Overview

The Primitive Liquidity Engine is a sophisticated cross-chain liquidity infrastructure built on Solana that integrates with Jupiter DEX aggregation and Sanctum's JSOL liquid staking protocol. It provides instant unstaking capabilities, optimal swap routing, and seamless cross-chain asset transfers.

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────────┐
│                    Primitive Liquidity Engine                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Jupiter   │  │   Sanctum   │  │  Wormhole   │  │  Security   │ │
│  │ Integration │  │    JSOL     │  │   Bridge    │  │ Monitoring  │ │
│  │             │  │   Staking   │  │             │  │             │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                       Core Engine State                        │
│  • Liquidity Pools    • Reserve Management  • Fee Calculation  │
│  • Cross-Chain Config • Risk Management     • User Positions   │
└─────────────────────────────────────────────────────────────────┘
```

### Key Features

1. **Jupiter DEX Integration**: Optimal token swaps with minimal slippage
2. **JSOL Liquid Staking**: Stake SOL and receive liquid staking tokens
3. **Instant Unstaking**: Convert JSOL back to SOL without waiting periods
4. **Cross-Chain Bridges**: Transfer assets to Ethereum, BSC, Polygon, etc.
5. **Security Monitoring**: Real-time risk assessment and circuit breakers
6. **Fee Optimization**: Dynamic fee structures based on liquidity conditions

## Getting Started

### Prerequisites

- Solana CLI tools
- Anchor framework
- Node.js and npm/yarn
- A Solana wallet (Phantom, Solflare, etc.)

### Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd btc-vault-protocol
   ```

2. **Install dependencies:**
   ```bash
   # Install Rust dependencies
   cargo build

   # Install Node.js dependencies
   cd frontend && npm install
   ```

3. **Build the program:**
   ```bash
   anchor build
   ```

4. **Deploy to devnet:**
   ```bash
   anchor deploy --provider.cluster devnet
   ```

## API Reference

### Program Instructions

#### Initialize Liquidity Engine

```rust
pub fn initialize_liquidity_engine(ctx: Context<InitializeLiquidityEngine>) -> Result<()>
```

Initializes the liquidity engine with default parameters.

**Accounts:**
- `liquidity_engine`: The main engine state account (PDA)
- `authority`: The authority that can manage the engine
- `system_program`: Solana system program

#### Swap Via Jupiter

```rust
pub fn swap_via_jupiter(
    ctx: Context<SwapViaJupiter>,
    amount_in: u64,
    minimum_amount_out: u64,
    jupiter_data: Vec<u8>,
) -> Result<()>
```

Executes a token swap using Jupiter's optimal routing.

**Parameters:**
- `amount_in`: Amount of input tokens
- `minimum_amount_out`: Minimum acceptable output amount
- `jupiter_data`: Serialized Jupiter routing data

**Accounts:**
- `liquidity_engine`: Main engine state
- `user`: User executing the swap
- `user_source_token`: User's source token account
- `user_destination_token`: User's destination token account
- `source_vault`: Engine's source token vault
- `destination_vault`: Engine's destination token vault
- `jupiter_program`: Jupiter program ID
- `token_program`: SPL Token program

#### Stake to JSOL

```rust
pub fn stake_to_jsol(
    ctx: Context<StakeToJSOL>,
    sol_amount: u64,
) -> Result<()>
```

Stakes SOL to receive JSOL liquid staking tokens.

**Parameters:**
- `sol_amount`: Amount of SOL to stake

**Accounts:**
- `liquidity_engine`: Main engine state
- `user`: User staking SOL
- `user_sol_account`: User's SOL token account
- `user_jsol_account`: User's JSOL token account
- `stake_pool`: Sanctum stake pool
- `sanctum_program`: Sanctum program ID

#### Instant Unstake JSOL

```rust
pub fn instant_unstake_jsol(
    ctx: Context<InstantUnstakeJSOL>,
    jsol_amount: u64,
) -> Result<()>
```

Instantly converts JSOL back to SOL using liquidity reserves.

**Parameters:**
- `jsol_amount`: Amount of JSOL to unstake

**Accounts:**
- `liquidity_engine`: Main engine state
- `user`: User unstaking JSOL
- `user_jsol_account`: User's JSOL token account
- `user_sol_account`: User's SOL token account
- `liquidity_pool`: Engine's liquidity pool
- `reserve_pool`: Engine's reserve pool

#### Cross Chain Bridge

```rust
pub fn cross_chain_bridge(
    ctx: Context<CrossChainBridge>,
    amount: u64,
    target_chain: u16,
    recipient: [u8; 32],
) -> Result<()>
```

Initiates a cross-chain transfer to another blockchain.

**Parameters:**
- `amount`: Amount to transfer
- `target_chain`: Target blockchain ID
- `recipient`: Recipient address on target chain

## TypeScript SDK Usage

### Basic Setup

```typescript
import { createLiquidityEngineClient } from './lib/liquidity-engine';
import { Connection, PublicKey } from '@solana/web3.js';

// Initialize connection
const connection = new Connection('https://api.devnet.solana.com');

// Create client
const client = createLiquidityEngineClient(connection);
await client.initialize();
```

### Token Swaps

```typescript
import { BN } from '@project-serum/anchor';

// Swap 1 SOL for USDC
const swapParams = {
  inputMint: new PublicKey('So11111111111111111111111111111111111111112'), // SOL
  outputMint: new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'), // USDC
  amount: new BN(1 * 1e9), // 1 SOL
  slippageBps: 100, // 1% slippage
  userPublicKey: wallet.publicKey,
};

const instruction = await client.swapViaJupiter(swapParams);
const transaction = await client.buildTransaction([instruction], wallet.publicKey);
const signature = await wallet.sendTransaction(transaction, connection);
```

### JSOL Staking

```typescript
// Stake 10 SOL for JSOL
const stakeParams = {
  amount: new BN(10 * 1e9), // 10 SOL
  userPublicKey: wallet.publicKey,
};

const instruction = await client.stakeToJSOL(stakeParams);
const transaction = await client.buildTransaction([instruction], wallet.publicKey);
const signature = await wallet.sendTransaction(transaction, connection);
```

### Instant Unstaking

```typescript
// Instantly unstake 5 JSOL
const unstakeParams = {
  jsolAmount: new BN(5 * 1e9), // 5 JSOL
  userPublicKey: wallet.publicKey,
};

const instruction = await client.instantUnstakeJSOL(unstakeParams);
const transaction = await client.buildTransaction([instruction], wallet.publicKey);
const signature = await wallet.sendTransaction(transaction, connection);
```

### Cross-Chain Transfers

```typescript
// Bridge 2 SOL to Ethereum
const bridgeParams = {
  amount: new BN(2 * 1e9), // 2 SOL
  targetChain: 1, // Ethereum
  recipient: new Uint8Array(32), // Ethereum address as bytes
  userPublicKey: wallet.publicKey,
};

const instruction = await client.crossChainBridge(bridgeParams);
const transaction = await client.buildTransaction([instruction], wallet.publicKey);
const signature = await wallet.sendTransaction(transaction, connection);
```

## React Component Usage

### Basic Implementation

```tsx
import React from 'react';
import { LiquidityEngine } from './components/LiquidityEngine';
import { WalletProvider } from '@solana/wallet-adapter-react';

function App() {
  return (
    <WalletProvider wallets={[]}>
      <div className="App">
        <LiquidityEngine />
      </div>
    </WalletProvider>
  );
}
```

### Custom Integration

```tsx
import { createLiquidityEngineClient } from './lib/liquidity-engine';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';

function CustomLiquidityInterface() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  
  const [client, setClient] = useState(null);
  
  useEffect(() => {
    const initClient = async () => {
      const liquidityClient = createLiquidityEngineClient(connection);
      await liquidityClient.initialize();
      setClient(liquidityClient);
    };
    
    if (connection) {
      initClient();
    }
  }, [connection]);
  
  // Your custom UI implementation
  return (
    <div>
      {/* Custom liquidity interface */}
    </div>
  );
}
```

## Security Considerations

### Risk Management

1. **Slippage Protection**: All swaps include maximum slippage limits
2. **Reserve Monitoring**: Instant unstaking requires sufficient reserves
3. **Circuit Breakers**: Automatic pause during anomalous conditions
4. **Multi-signature**: Critical operations require multiple signatures

### Best Practices

1. **Always verify transaction details** before signing
2. **Use appropriate slippage tolerances** for market conditions
3. **Monitor reserve levels** before large unstaking operations
4. **Keep private keys secure** and use hardware wallets when possible

## Error Handling

### Common Errors

```rust
#[error_code]
pub enum VaultError {
    #[msg("Liquidity engine is paused")]
    LiquidityEnginePaused,
    
    #[msg("Insufficient liquidity for instant unstaking")]
    InsufficientLiquidity,
    
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    
    #[msg("Unsupported chain")]
    UnsupportedChain,
}
```

### Error Recovery

```typescript
try {
  const instruction = await client.swapViaJupiter(params);
  // ... execute transaction
} catch (error) {
  if (error.message.includes('SlippageExceeded')) {
    // Increase slippage tolerance or retry
    console.log('Slippage exceeded, consider increasing tolerance');
  } else if (error.message.includes('InsufficientLiquidity')) {
    // Wait for liquidity or use alternative method
    console.log('Insufficient liquidity, try again later');
  }
}
```

## Performance Optimization

### Transaction Batching

```typescript
// Batch multiple operations
const instructions = [
  await client.swapViaJupiter(swapParams),
  await client.stakeToJSOL(stakeParams),
];

const transaction = await client.buildTransaction(instructions, wallet.publicKey);
```

### Compute Unit Optimization

```typescript
// Set compute unit limit for complex operations
const instruction = await client.swapViaJupiter(params);
instruction.keys.push({
  pubkey: ComputeBudgetProgram.programId,
  isSigner: false,
  isWritable: false,
});
```

## Monitoring and Analytics

### Engine State Monitoring

```typescript
// Get current engine state
const state = await client.getLiquidityEngineState();
console.log({
  totalVolume: state.totalVolume.toString(),
  totalFees: state.totalFees.toString(),
  isPaused: state.isPaused,
  feeRate: state.feeRate,
});
```

### User Position Tracking

```typescript
// Track user's liquidity positions
const position = await client.getUserPosition(wallet.publicKey);
console.log({
  jsolStaked: position.jsolStaked.toString(),
  rewardsEarned: position.rewardsEarned.toString(),
  totalVolume: position.totalVolume.toString(),
});
```

## Testing

### Unit Tests

```bash
# Run Rust tests
cargo test

# Run TypeScript tests
npm test
```

### Integration Tests

```bash
# Run full integration test suite
anchor test
```

### Local Development

```bash
# Start local validator
solana-test-validator

# Deploy to local
anchor deploy --provider.cluster localnet
```

## Deployment

### Devnet Deployment

```bash
# Configure for devnet
solana config set --url https://api.devnet.solana.com

# Deploy program
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment

```bash
# Configure for mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Deploy with security audit
anchor deploy --provider.cluster mainnet-beta
```

## Support and Contributing

### Getting Help

- **Documentation**: Check this guide and inline code comments
- **Issues**: Create GitHub issues for bugs or feature requests
- **Discord**: Join our community Discord for real-time support

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Code Standards

- Follow Rust and TypeScript best practices
- Add comprehensive tests
- Update documentation
- Use semantic commit messages

## Roadmap

### Phase 1 (Current)
- ✅ Jupiter DEX integration
- ✅ JSOL liquid staking
- ✅ Instant unstaking
- ✅ Cross-chain bridges
- ✅ Security monitoring

### Phase 2 (Q2 2024)
- 🔄 Advanced routing algorithms
- 🔄 Multi-asset liquidity pools
- 🔄 Yield farming integration
- 🔄 Mobile wallet support

### Phase 3 (Q3 2024)
- 📋 Institutional features
- 📋 Advanced analytics
- 📋 Governance token
- 📋 DAO integration

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Disclaimer

This software is provided "as is" without warranty of any kind. Users are responsible for understanding the risks involved in DeFi operations. Always conduct your own research and consider the volatility of cryptocurrency markets.

---

**Last Updated**: January 2024  
**Version**: 1.0.0  
**Maintainers**: BTC Vault Protocol Team