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

// Enhanced KYC Types
export interface KYCProfile {
  user: string;
  tier: 'none' | 'basic' | 'enhanced' | 'institutional';
  status: 'not_started' | 'pending' | 'approved' | 'rejected' | 'expired' | 'suspended';
  documents: KYCDocument[];
  commitmentLimit: number;
  dailyLimit: number;
  monthlyVolume: number;
  lastScreeningDate: number;
  kycExpiryDate?: number;
  complianceScreening?: ComplianceScreening;
}

export interface KYCDocument {
  type: 'passport' | 'drivers_license' | 'national_id' | 'proof_of_address' | 'bank_statement' | 'tax_document' | 'corporate_registration' | 'beneficial_ownership';
  hash: string;
  uploaded: boolean;
  verified: boolean;
  uploadDate?: Date;
  verificationDate?: Date;
  expiryDate?: Date;
}

export interface ComplianceScreening {
  riskLevel: 'low' | 'medium' | 'high' | 'prohibited';
  sanctionsMatch: boolean;
  pepMatch: boolean;
  adverseMedia: boolean;
  screeningDate: number;
  expiryDate: number;
  notes?: string;
}

// Enhanced Authentication Types
export interface AuthStatus {
  isAuthenticated: boolean;
  twoFactorEnabled: boolean;
  authMethods: AuthFactor[];
  activeSessions: UserSession[];
  accountLocked: boolean;
  lastLogin: number;
  securityEvents?: SecurityEvent[];
  securitySettings?: SecuritySettings;
}

export interface AuthFactor {
  method: 'totp' | 'webauthn' | 'sms' | 'email' | 'passkey';
  identifier: string;
  enabled: boolean;
  verified: boolean;
  createdAt: Date;
  lastUsed: Date;
  failureCount: number;
  locked: boolean;
}

export interface UserSession {
  sessionId: string;
  deviceId: string;
  ipAddress: string;
  userAgent: string;
  status: 'active' | 'expired' | 'revoked' | 'compromised';
  createdAt: Date;
  lastActivity: Date;
  expiresAt: Date;
  authMethods: string[];
  riskScore: number;
  isCurrent: boolean;
}

export interface SecurityEvent {
  eventId: string;
  eventType: 'login_success' | 'login_failure' | 'two_factor_success' | 'two_factor_failure' | 'session_created' | 'session_expired' | 'suspicious_activity' | 'account_locked';
  timestamp: Date;
  details: string;
  riskLevel: number;
  resolved: boolean;
  ipAddress?: string;
  deviceId?: string;
}

export interface SecuritySettings {
  require2FAForAll: boolean;
  require2FAForPayments: boolean;
  require2FAForHighValue: boolean;
  sessionTimeout: number;
  maxConcurrentSessions: number;
  enableEmailNotifications: boolean;
  enableSMSNotifications: boolean;
  autoLockOnSuspicious: boolean;
  trustedDevices: string[];
  ipWhitelist: string[];
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

export interface PaymentHistory {
  id: string;
  amount: number;
  currency: 'BTC' | 'USDC';
  timestamp: number;
  status: 'pending' | 'completed' | 'failed';
  type?: string;
}