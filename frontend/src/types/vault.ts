import { PublicKey } from '@solana/web3.js';

export interface VaultProtocolAPI {
  // BTC Operations
  commitBTC(amount: number, btcAddress: string, proof: string): Promise<TransactionResult>;
  verifyBalance(): Promise<BalanceInfo>;
  updateCommitment(newAmount: number): Promise<TransactionResult>;
  
  // Reward Operations
  claimRewards(paymentType: PaymentType): Promise<TransactionResult>;
  setAutoReinvest(enabled: boolean): Promise<void>;
  getRewardHistory(): Promise<RewardTransaction[]>;
  
  // Dashboard Data
  getUserStats(): Promise<UserStats>;
  getTreasuryStats(): Promise<TreasuryStats>;
  
  // Wallet Integration
  connectWallet(type: WalletType): Promise<WalletConnection>;
  disconnectWallet(): Promise<void>;
  
  // KYC Operations
  submitKYC(documents: KYCDocuments): Promise<KYCResult>;
  checkKYCStatus(): Promise<KYCStatus>;
}

export interface UserStats {
  btcCommitted: number;
  btcAddress: string;
  rewardsEarned: number;
  rewardsPending: number;
  kycStatus: KYCTier;
  paymentPreference: PaymentType;
  autoReinvestEnabled: boolean;
  twoFAEnabled: boolean;
  lastActivity: Date;
}

export interface TreasuryStats {
  totalAssets: number;
  totalRewardsUSD: number;
  // Note: Allocation details hidden to prevent whale manipulation
}

export interface TransactionResult {
  success: boolean;
  signature?: string;
  error?: string;
}

export interface BalanceInfo {
  verified: boolean;
  balance: number;
  lastVerification: Date;
}

export interface RewardTransaction {
  id: string;
  amount: number;
  currency: PaymentType;
  timestamp: Date;
  status: 'pending' | 'completed' | 'failed';
}

export interface WalletConnection {
  connected: boolean;
  publicKey?: PublicKey;
  walletType: WalletType;
}

export interface KYCDocuments {
  idDocument: File;
  proofOfAddress: File;
  additionalDocs?: File[];
}

export interface KYCResult {
  submitted: boolean;
  verificationId: string;
  estimatedProcessingTime: string;
}

export interface KYCStatus {
  tier: KYCTier;
  verificationDate?: Date;
  commitmentLimit: number;
  documentsRequired: string[];
}

export type PaymentType = 'BTC' | 'USDC' | 'AutoReinvest';

export type WalletType = 'BlueWallet' | 'Ledger' | 'Phantom' | 'Solflare';

export type KYCTier = 'none' | 'pending' | 'verified';

export interface VaultConfig {
  network: 'devnet' | 'testnet' | 'mainnet';
  programId: string;
  rpcEndpoint: string;
  chainlinkOracles: {
    btcUsd: string;
    utxoVerification: string;
  };
}

// Dashboard-specific types
export interface BTCCommitment {
  user: string;
  btc_amount: string;
  commitment_hash: string;
  timestamp: number;
  status: 'pending' | 'verified' | 'failed';
}

export interface StakingPool {
  pool_id: string;
  asset_type: 'SOL' | 'ETH' | 'ATOM';
  total_staked: string;
  apy: string;
  validator: string;
}

export interface RewardPool {
  total_rewards: number;
  available_rewards: number;
  userRewards: number;
  stakingRewards: number;
  commitmentRewards: number;
  referralRewards: number;
  lastUpdate: number;
}

export interface RewardSummary {
  total_rewards: string;
  pending_rewards: string;
  claimed_rewards: string;
  staking_rewards: string;
  commitment_rewards: string;
  referral_rewards: string;
  next_distribution: string;
  apy: string;
}

export interface TreasuryData {
  total_value_usd: string;
  sol_allocation: string;
  eth_allocation: string;
  atom_allocation: string;
  last_rebalance: string;
  next_deposit: string;
}