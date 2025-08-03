'use client';

import { useTranslation } from 'react-i18next';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function Home() {
  const { t, i18n } = useTranslation();

  const changeLanguage = (lng: string) => {
    i18n.changeLanguage(lng);
  };

  return (
    <main className="min-h-screen bg-gradient-to-br from-vault-primary to-vault-secondary text-white">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <header className="flex justify-between items-center mb-12">
          <h1 className="text-3xl font-bold text-vault-accent">
            {t('dashboard.title')}
          </h1>
          
          <div className="flex items-center gap-4">
            {/* Language Selector */}
            <select 
              onChange={(e) => changeLanguage(e.target.value)}
              className="bg-vault-secondary border border-gray-600 rounded px-3 py-1 text-sm"
              defaultValue={i18n.language}
            >
              <option value="en">English</option>
              <option value="es">Español</option>
              <option value="zh">中文</option>
              <option value="ja">日本語</option>
            </select>
            
            {/* Wallet Connection */}
            <WalletMultiButton />
          </div>
        </header>

        {/* Welcome Section */}
        <section className="text-center mb-16">
          <h2 className="text-5xl font-bold mb-6">
            Vault Protocol
          </h2>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto mb-8">
            A security-focused, non-custodial Bitcoin protocol that allows users to commit BTC 
            without transferring custody while earning rewards from protocol-owned staking activities.
          </p>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mt-12">
            <div className="bg-vault-secondary/50 p-6 rounded-lg border border-gray-700">
              <h3 className="text-xl font-semibold mb-3 text-vault-accent">
                Non-Custodial
              </h3>
              <p className="text-gray-300">
                Keep full control of your Bitcoin while earning staking rewards through ECDSA proof verification.
              </p>
            </div>
            
            <div className="bg-vault-secondary/50 p-6 rounded-lg border border-gray-700">
              <h3 className="text-xl font-semibold mb-3 text-vault-accent">
                Multi-Chain Staking
              </h3>
              <p className="text-gray-300">
                Protocol stakes across SOL (40%), ETH (30%), and ATOM (30%) to generate rewards.
              </p>
            </div>
            
            <div className="bg-vault-secondary/50 p-6 rounded-lg border border-gray-700">
              <h3 className="text-xl font-semibold mb-3 text-vault-accent">
                Enterprise Security
              </h3>
              <p className="text-gray-300">
                2-of-3 multisig with Yubico HSMs, optional KYC, and 2FA authentication.
              </p>
            </div>
          </div>
        </section>

        {/* Stats Section */}
        <section className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
          <div className="bg-vault-secondary/30 p-6 rounded-lg border border-gray-700">
            <h3 className="text-sm font-medium text-gray-400 mb-2">
              {t('dashboard.btcCommitted')}
            </h3>
            <p className="text-2xl font-bold text-vault-accent">0.00 BTC</p>
          </div>
          
          <div className="bg-vault-secondary/30 p-6 rounded-lg border border-gray-700">
            <h3 className="text-sm font-medium text-gray-400 mb-2">
              {t('dashboard.rewardsEarned')}
            </h3>
            <p className="text-2xl font-bold text-vault-success">$0.00</p>
          </div>
          
          <div className="bg-vault-secondary/30 p-6 rounded-lg border border-gray-700">
            <h3 className="text-sm font-medium text-gray-400 mb-2">
              {t('dashboard.treasuryAssets')}
            </h3>
            <p className="text-2xl font-bold">$0.00</p>
          </div>
          
          <div className="bg-vault-secondary/30 p-6 rounded-lg border border-gray-700">
            <h3 className="text-sm font-medium text-gray-400 mb-2">
              {t('dashboard.totalRewards')}
            </h3>
            <p className="text-2xl font-bold">$0.00</p>
          </div>
        </section>

        {/* Action Buttons */}
        <section className="flex flex-col sm:flex-row gap-4 justify-center">
          <button className="bg-vault-accent hover:bg-orange-600 text-white font-medium py-3 px-8 rounded-lg transition-colors">
            {t('btc.commitTitle')}
          </button>
          
          <button className="bg-vault-success hover:bg-green-600 text-white font-medium py-3 px-8 rounded-lg transition-colors">
            {t('rewards.claim')}
          </button>
          
          <button className="border border-gray-600 hover:bg-vault-secondary text-white font-medium py-3 px-8 rounded-lg transition-colors">
            {t('kyc.title')}
          </button>
        </section>
      </div>
    </main>
  );
}