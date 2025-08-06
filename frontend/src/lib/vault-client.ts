import { Connection, PublicKey, Transaction, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Program, AnchorProvider, Wallet, BN } from '@project-serum/anchor';
import { 
  VaultAccount,
  PaymentMethod,
  Transaction as VaultTransaction,
  KYCProfile,
  AuthStatus,
  MultisigWallet,
  RewardPool,
  StakingPool,
  ApiResponse,
  PaginatedResponse,
  SecurityEvent,
  PaymentHistory,
  AuthMethod,
  AuthSession,
  TransactionStatus,
  KYCStatus,
  PaymentConfig,
  RewardClaim,
  VaultConfig,
  WalletState,
  VaultError
} from '../types/vault';

export class VaultClient {
  private connection: Connection;
  private program: Program | null = null;
  private wallet: Wallet | null = null;
  private config: VaultConfig;
  private programId: PublicKey;

  constructor(config: VaultConfig) {
    this.config = config;
    this.connection = new Connection(config.rpcEndpoint, 'confirmed');
    this.programId = new PublicKey(config.programId);
  }

  async connectWallet(wallet: Wallet): Promise<void> {
    this.wallet = wallet;
    const provider = new AnchorProvider(this.connection, wallet, {
      commitment: 'confirmed',
      preflightCommitment: 'confirmed',
    });
    // Initialize program with IDL when available
    // this.program = new Program(IDL, this.programId, provider);
  }

  async disconnectWallet(): Promise<void> {
    this.wallet = null;
    this.program = null;
  }

  get isConnected(): boolean {
    return this.wallet !== null;
  }

  get publicKey(): PublicKey | null {
    return this.wallet?.publicKey || null;
  }

  async commitBTC(amount: number, btcAddress: string, proof: string): Promise<TransactionResult> {
    if (!this.program || !this.wallet) {
      return { success: false, error: 'Wallet not connected' };
    }

    try {
      // Convert amount to satoshis
      const amountSatoshis = Math.floor(amount * 100_000_000);
      
      // Call the commit_btc instruction
      const tx = await this.program.methods
        .commitBtc(amountSatoshis, btcAddress, Buffer.from(proof, 'hex'))
        .accounts({
          user: this.wallet.publicKey,
        })
        .rpc();

      return { success: true, signature: tx };
    } catch (error) {
      return { success: false, error: error instanceof Error ? error.message : 'Unknown error' };
    }
  }

  async verifyBalance(): Promise<BalanceInfo> {
    if (!this.program || !this.wallet) {
      throw new Error('Wallet not connected');
    }

    try {
      await this.program.methods
        .verifyBalance()
        .accounts({
          user: this.wallet.publicKey,
        })
        .rpc();

      // Fetch the updated commitment account
      const [commitmentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('btc_commitment'), this.wallet.publicKey.toBuffer()],
        this.program.programId
      );

      const commitment = await this.program.account.btcCommitment.fetch(commitmentPda);
      
      return {
        verified: commitment.verified,
        balance: commitment.amount / 100_000_000, // Convert from satoshis
        lastVerification: new Date(commitment.lastVerification * 1000)
      };
    } catch (error) {
      throw new Error(`Balance verification failed: ${error}`);
    }
  }

  async updateCommitment(newAmount: number): Promise<TransactionResult> {
    // Implementation would call update_commitment instruction
    throw new Error('Not implemented');
  }

  async claimRewards(paymentType: PaymentType): Promise<TransactionResult> {
    // Implementation would call reward distribution instructions
    throw new Error('Not implemented');
  }

  async setAutoReinvest(enabled: boolean): Promise<void> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      // Simulate setting auto-reinvest - in production this would update user settings
      console.log(`Auto-reinvest ${enabled ? 'enabled' : 'disabled'}`);
      // Would call program instruction to update user preferences
    } catch (error) {
      console.error('Failed to set auto-reinvest:', error);
      throw error;
    }
  }

  async getRewardHistory(): Promise<any[]> {
    if (!this.wallet) {
      throw new Error('Wallet not connected');
    }

    try {
      // Simulate reward history - in production this would fetch from program
      const mockHistory = [
        {
          id: 'reward_001',
          amount: 0.00125,
          currency: 'BTC',
          timestamp: Date.now() - 86400000, // 1 day ago
          status: 'completed',
          type: 'staking_reward'
        },
        {
          id: 'reward_002', 
          amount: 0.00075,
          currency: 'BTC',
          timestamp: Date.now() - 172800000, // 2 days ago
          status: 'completed',
          type: 'commitment_reward'
        }
      ];
      return mockHistory;
    } catch (error) {
      console.error('Failed to fetch reward history:', error);
      return [];
    }
  }

  async getUserStats(): Promise<UserStats> {
    if (!this.program || !this.wallet) {
      throw new Error('Wallet not connected');
    }

    try {
      const [userAccountPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_account'), this.wallet.publicKey.toBuffer()],
        this.program.programId
      );

      const userAccount = await this.program.account.userAccount.fetch(userAccountPda);
      
      return {
        btcCommitted: userAccount.btcCommitmentAmount / 100_000_000,
        btcAddress: userAccount.btcAddress,
        rewardsEarned: userAccount.rewardBalance / 100_000_000,
        rewardsPending: 0, // Would be calculated from pending rewards
        kycStatus: userAccount.kycTier === 0 ? 'none' : 'verified',
        paymentPreference: this.mapPaymentType(userAccount.paymentPreference),
        autoReinvestEnabled: userAccount.autoReinvest,
        twoFAEnabled: userAccount.twoFaEnabled,
        lastActivity: new Date(userAccount.lastActivity * 1000)
      };
    } catch (error) {
      throw new Error(`Failed to fetch user stats: ${error}`);
    }
  }

  async getTreasuryStats(): Promise<TreasuryStats> {
    if (!this.program) {
      throw new Error('Program not initialized');
    }

    try {
      const [treasuryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('treasury')],
        this.program.programId
      );

      const treasury = await this.program.account.treasury.fetch(treasuryPda);
      
      return {
        totalAssets: treasury.totalAssets / 100_000_000,
        totalRewardsUSD: treasury.userRewardsPool / 100_000_000
      };
    } catch (error) {
      throw new Error(`Failed to fetch treasury stats: ${error}`);
    }
  }

  async submitKYC(documents: any): Promise<any> {
    // Implementation would handle KYC document submission
    throw new Error('Not implemented');
  }

  async checkKYCStatus(): Promise<any> {
    // Implementation would check KYC verification status
    throw new Error('Not implemented');
  }

  // Enhanced Vault Account Management
  async getVaultAccount(userPubkey?: PublicKey): Promise<VaultAccount | null> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      const [vaultAccountPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('vault_account'), pubkey.toBuffer()],
        this.programId
      );

      const accountInfo = await this.connection.getAccountInfo(vaultAccountPDA);
      if (!accountInfo) return null;

      // Parse account data (would use program.account.vaultAccount.fetch in real implementation)
      return {
        user: pubkey.toString(),
        btcCommitted: 0,
        rewardsEarned: 0,
        lastActivity: Date.now(),
        kycStatus: 'not_verified',
        authStatus: {
          isAuthenticated: false,
          twoFactorEnabled: false,
          authMethods: [],
          activeSessions: [],
          accountLocked: false,
          lastLogin: 0,
        },
        multisigWallets: [],
      };
    } catch (error) {
      console.error('Error fetching vault account:', error);
      return null;
    }
  }

  // Enhanced Payment System
  async makePayment(payment: PaymentMethod): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      
      if (payment.type === 'lightning') {
        // Add Lightning payment instruction
        // transaction.add(await this.createLightningPaymentInstruction(payment));
      } else if (payment.type === 'usdc') {
        // Add USDC payment instruction
        // transaction.add(await this.createUSDCPaymentInstruction(payment));
      }

      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error making payment:', error);
      throw error;
    }
  }

  async getPaymentHistory(userPubkey?: PublicKey, page = 1, limit = 20): Promise<PaginatedResponse<PaymentHistory>> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      // Fetch payment history from program accounts or API
      return {
        items: [],
        total: 0,
        page,
        limit,
        hasNext: false,
        hasPrev: false,
      };
    } catch (error) {
      console.error('Error fetching payment history:', error);
      throw error;
    }
  }

  async getPaymentConfig(): Promise<PaymentConfig> {
    try {
      return {
        lightningEnabled: true,
        usdcEnabled: true,
        autoReinvestment: {
          enabled: false,
          percentage: 0,
          frequency: 'monthly',
        },
        emergencyPaused: false,
      };
    } catch (error) {
      console.error('Error fetching payment config:', error);
      throw error;
    }
  }

  // KYC System
  async getKYCProfile(userPubkey?: PublicKey): Promise<KYCProfile | null> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      const [kycProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('kyc_profile'), pubkey.toBuffer()],
        this.programId
      );

      const accountInfo = await this.connection.getAccountInfo(kycProfilePDA);
      if (!accountInfo) return null;

      // Simulate KYC profile data
      return {
        user: pubkey.toString(),
        tier: 'none',
        status: 'not_started',
        documents: [],
        commitmentLimit: 100_000_000, // 1 BTC in satoshis
        dailyLimit: 10_000_000, // 0.1 BTC
        monthlyVolume: 0,
        lastScreeningDate: 0,
        complianceScreening: null
      };
    } catch (error) {
      console.error('Error fetching KYC profile:', error);
      return null;
    }
  }

  async startKYCVerification(targetTier: 'basic' | 'enhanced' | 'institutional'): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add start KYC verification instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error starting KYC verification:', error);
      throw error;
    }
  }

  async submitKYCDocument(documentType: string, documentHash: string): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add submit KYC document instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error submitting KYC document:', error);
      throw error;
    }
  }

  async uploadKYCDocument(documentType: string, file: File): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      // Upload document to IPFS or secure storage
      const documentHash = await this.hashDocument(file);
      
      // Submit document hash to blockchain
      return await this.submitKYCDocument(documentType, documentHash);
    } catch (error) {
      console.error('Error uploading KYC document:', error);
      throw error;
    }
  }

  async checkKYCStatus(): Promise<any> {
    if (!this.publicKey) throw new Error('No public key available');

    try {
      const profile = await this.getKYCProfile();
      return {
        tier: profile?.tier || 'none',
        status: profile?.status || 'not_started',
        commitmentLimit: profile?.commitmentLimit || 100_000_000,
        dailyLimit: profile?.dailyLimit || 10_000_000,
        complianceScreening: profile?.complianceScreening
      };
    } catch (error) {
      console.error('Error checking KYC status:', error);
      throw error;
    }
  }

  private async hashDocument(file: File): Promise<string> {
    const buffer = await file.arrayBuffer();
    const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }

  // Authentication System
  async getAuthStatus(userPubkey?: PublicKey): Promise<AuthStatus> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      const [userAuthPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_auth'), pubkey.toBuffer()],
        this.programId
      );

      const accountInfo = await this.connection.getAccountInfo(userAuthPDA);
      if (!accountInfo) {
        return {
          isAuthenticated: false,
          twoFactorEnabled: false,
          authMethods: [],
          activeSessions: [],
          accountLocked: false,
          lastLogin: 0,
        };
      }

      // Simulate auth status data
      return {
        isAuthenticated: true,
        twoFactorEnabled: false,
        authMethods: [],
        activeSessions: [
          {
            sessionId: 'session_123',
            deviceId: 'device_456',
            ipAddress: '192.168.1.1',
            userAgent: 'Chrome/120.0.0.0',
            status: 'active',
            createdAt: new Date(),
            lastActivity: new Date(),
            expiresAt: new Date(Date.now() + 3600000),
            authMethods: ['wallet'],
            riskScore: 10,
            isCurrent: true
          }
        ],
        accountLocked: false,
        lastLogin: Date.now(),
      };
    } catch (error) {
      console.error('Error fetching auth status:', error);
      throw error;
    }
  }

  async initializeUserAuth(): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add initialize user auth instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error initializing user auth:', error);
      throw error;
    }
  }

  async addAuthFactor(method: 'totp' | 'webauthn' | 'sms' | 'email' | 'passkey', identifier: string, secretHash: Uint8Array, backupCodes: string[]): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add auth factor instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error adding auth factor:', error);
      throw error;
    }
  }

  async verifyAuthFactor(method: 'totp' | 'webauthn' | 'sms' | 'email' | 'passkey', identifier: string, providedCode: string): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add verify auth factor instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error verifying auth factor:', error);
      throw error;
    }
  }

  async createSession(deviceId: string, ipAddress: string, userAgent: string, authMethods: string[]): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add create session instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error creating session:', error);
      throw error;
    }
  }

  async validateSession(sessionId: string): Promise<boolean> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add validate session instruction
      
      await this.program.provider.sendAndConfirm(transaction);
      return true;
    } catch (error) {
      console.error('Error validating session:', error);
      return false;
    }
  }

  async revokeSession(sessionId: string): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add revoke session instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error revoking session:', error);
      throw error;
    }
  }

  async updateSecuritySettings(settings: any): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add update security settings instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error updating security settings:', error);
      throw error;
    }
  }

  async generateBackupCodes(): Promise<string[]> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      // Generate 10 backup codes
      const codes = [];
      for (let i = 0; i < 10; i++) {
        const code = Math.random().toString(36).substring(2, 10).toUpperCase();
        codes.push(code);
      }
      return codes;
    } catch (error) {
      console.error('Error generating backup codes:', error);
      throw error;
    }
  }

  async verifyBackupCode(backupCode: string): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add verify backup code instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error verifying backup code:', error);
      throw error;
    }
  }

  async setupTwoFactor(method: 'totp' | 'webauthn' | 'passkey', identifier: string): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      // Generate secret for TOTP or handle WebAuthn/Passkey setup
      const secretHash = new Uint8Array(32);
      crypto.getRandomValues(secretHash);
      
      const backupCodes = await this.generateBackupCodes();
      
      return await this.addAuthFactor(method, identifier, secretHash, backupCodes);
    } catch (error) {
      console.error('Error setting up 2FA:', error);
      throw error;
    }
  }

  async verifyTwoFactor(method: string, code: string): Promise<boolean> {
    try {
      await this.verifyAuthFactor(method as any, method, code);
      return true;
    } catch (error) {
      console.error('Error verifying 2FA:', error);
      return false;
    }
  }

  // Multisig System
  async getMultisigWallets(userPubkey?: PublicKey): Promise<MultisigWallet[]> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      // Fetch multisig wallets where user is an owner
      return []; // Placeholder
    } catch (error) {
      console.error('Error fetching multisig wallets:', error);
      return [];
    }
  }

  async createMultisigWallet(owners: string[], threshold: number): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add create multisig instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error creating multisig wallet:', error);
      throw error;
    }
  }

  // Rewards System
  async getRewardPool(): Promise<RewardPool> {
    try {
      const [rewardPoolPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('reward_pool')],
        this.programId
      );

      const accountInfo = await this.connection.getAccountInfo(rewardPoolPDA);
      if (!accountInfo) {
        return {
          totalRewards: 0,
          availableRewards: 0,
          userRewards: 0,
          stakingRewards: 0,
          commitmentRewards: 0,
          referralRewards: 0,
          lastUpdate: Date.now(),
        };
      }

      return {
        totalRewards: 1000000,
        availableRewards: 500000,
        userRewards: 10000,
        stakingRewards: 5000,
        commitmentRewards: 3000,
        referralRewards: 2000,
        lastUpdate: Date.now(),
      };
    } catch (error) {
      console.error('Error fetching reward pool:', error);
      throw error;
    }
  }

  async claimRewards(userPubkey?: PublicKey): Promise<string> {
    if (!this.wallet || !this.program) {
      throw new Error('Wallet not connected');
    }

    try {
      const transaction = new Transaction();
      // Add claim rewards instruction
      
      const signature = await this.program.provider.sendAndConfirm(transaction);
      return signature;
    } catch (error) {
      console.error('Error claiming rewards:', error);
      throw error;
    }
  }

  // Transaction History
  async getTransactionHistory(userPubkey?: PublicKey, page = 1, limit = 20): Promise<PaginatedResponse<VaultTransaction>> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      const transactions: VaultTransaction[] = [];
      
      return {
        items: transactions,
        total: transactions.length,
        page,
        limit,
        hasNext: false,
        hasPrev: false,
      };
    } catch (error) {
      console.error('Error fetching transaction history:', error);
      return {
        items: [],
        total: 0,
        page,
        limit,
        hasNext: false,
        hasPrev: false,
      };
    }
  }

  // Utility Methods
  async getBalance(pubkey?: PublicKey): Promise<number> {
    const targetPubkey = pubkey || this.publicKey;
    if (!targetPubkey) throw new Error('No public key available');

    try {
      const balance = await this.connection.getBalance(targetPubkey);
      return balance / LAMPORTS_PER_SOL;
    } catch (error) {
      console.error('Error fetching balance:', error);
      return 0;
    }
  }

  async waitForConfirmation(signature: string, commitment = 'confirmed'): Promise<boolean> {
    try {
      const result = await this.connection.confirmTransaction(signature, commitment as any);
      return !result.value.err;
    } catch (error) {
      console.error('Error waiting for confirmation:', error);
      return false;
    }
  }

  getExplorerUrl(signature: string): string {
    const network = this.connection.rpcEndpoint.includes('devnet') ? 'devnet' : 'mainnet-beta';
    return `https://explorer.solana.com/tx/${signature}?cluster=${network}`;
  }

  getAccountExplorerUrl(pubkey: string): string {
    const network = this.connection.rpcEndpoint.includes('devnet') ? 'devnet' : 'mainnet-beta';
    return `https://explorer.solana.com/address/${pubkey}?cluster=${network}`;
  }

  // Dashboard-specific methods
  async getBTCBalance(userPubkey?: PublicKey): Promise<number> {
    // Fetch BTC balance from Chainlink oracle
    // This would integrate with the oracle system from Task 3
    try {
      // Simulate oracle call - in production this would call the actual oracle
      const mockBalance = Math.random() * 2; // Random balance between 0-2 BTC
      return parseFloat(mockBalance.toFixed(8));
    } catch (error) {
      console.error('Failed to fetch BTC balance:', error);
      return 0;
    }
  }

  async getUserCommitments(userPubkey?: PublicKey): Promise<any[]> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      // Simulate fetching commitments - in production this would query the program
      const mockCommitments = [
        {
          user: pubkey.toString(),
          btc_amount: '0.25000000',
          commitment_hash: '0x1234567890abcdef',
          timestamp: Date.now() - 86400000, // 1 day ago
          status: 'verified'
        },
        {
          user: pubkey.toString(),
          btc_amount: '0.15000000',
          commitment_hash: '0xabcdef1234567890',
          timestamp: Date.now() - 172800000, // 2 days ago
          status: 'verified'
        }
      ];
      return mockCommitments;
    } catch (error) {
      console.error('Failed to fetch commitments:', error);
      return [];
    }
  }

  async getRewardSummary(userPubkey?: PublicKey): Promise<any> {
    const pubkey = userPubkey || this.publicKey;
    if (!pubkey) throw new Error('No public key available');

    try {
      // Simulate reward data - in production this would query the rewards program
      const mockRewards = {
        total_rewards: '0.00500000',
        pending_rewards: '0.00125000',
        claimed_rewards: '0.00375000',
        staking_rewards: '0.00300000',
        commitment_rewards: '0.00150000',
        referral_rewards: '0.00050000',
        next_distribution: new Date(Date.now() + 86400000).toISOString(), // Tomorrow
        apy: '8.5'
      };
      return mockRewards;
    } catch (error) {
      console.error('Failed to fetch reward summary:', error);
      return {
        total_rewards: '0.00000000',
        pending_rewards: '0.00000000',
        claimed_rewards: '0.00000000',
        staking_rewards: '0.00000000',
        commitment_rewards: '0.00000000',
        referral_rewards: '0.00000000',
        next_distribution: new Date().toISOString(),
        apy: '0.0'
      };
    }
  }

  async getTreasuryData(): Promise<any> {
    // Fetch treasury data (public information)
    try {
      // Simulate treasury data - in production this would query the treasury program
      const mockTreasury = {
        total_value_usd: '2,450,000.00',
        sol_allocation: '40',
        eth_allocation: '30',
        atom_allocation: '30',
        last_rebalance: new Date(Date.now() - 604800000).toISOString(), // 1 week ago
        next_deposit: new Date(Date.now() + 86400000 * 7).toISOString() // Next week
      };
      return mockTreasury;
    } catch (error) {
      console.error('Failed to fetch treasury data:', error);
      return {
        total_value_usd: '0.00',
        sol_allocation: '0',
        eth_allocation: '0',
        atom_allocation: '0',
        last_rebalance: new Date().toISOString(),
        next_deposit: new Date().toISOString()
      };
    }
  }

  // Legacy method compatibility
  private mapPaymentType(type: number): string {
    switch (type) {
      case 0: return 'BTC';
      case 1: return 'USDC';
      case 2: return 'AutoReinvest';
      default: return 'BTC';
    }
  }
}