import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { Program, AnchorProvider, Wallet } from '@project-serum/anchor';
import { 
  VaultProtocolAPI, 
  UserStats, 
  TreasuryStats, 
  TransactionResult, 
  BalanceInfo,
  PaymentType,
  WalletType,
  WalletConnection,
  VaultConfig
} from '../types/vault';

export class VaultClient implements VaultProtocolAPI {
  private connection: Connection;
  private program: Program | null = null;
  private wallet: Wallet | null = null;
  private config: VaultConfig;

  constructor(config: VaultConfig) {
    this.config = config;
    this.connection = new Connection(config.rpcEndpoint, 'confirmed');
  }

  async connectWallet(type: WalletType): Promise<WalletConnection> {
    // Implementation would integrate with specific wallet adapters
    throw new Error('Not implemented - requires wallet adapter integration');
  }

  async disconnectWallet(): Promise<void> {
    this.wallet = null;
    this.program = null;
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
    // Implementation would update user account settings
    throw new Error('Not implemented');
  }

  async getRewardHistory(): Promise<any[]> {
    // Implementation would fetch reward transaction history
    throw new Error('Not implemented');
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

  private mapPaymentType(type: number): PaymentType {
    switch (type) {
      case 0: return 'BTC';
      case 1: return 'USDC';
      case 2: return 'AutoReinvest';
      default: return 'BTC';
    }
  }
}