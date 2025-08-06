'use client';

import { useTranslation } from 'react-i18next';
import { useWallet } from '@solana/wallet-adapter-react';
import { Bitcoin, TrendingUp, Shield, Zap, ArrowRight, ExternalLink } from 'lucide-react';
import Link from 'next/link';
import { motion } from 'framer-motion';
import { StatsCard } from '@/components/StatsCard';
import { FeatureCard } from '@/components/FeatureCard';

export default function Home() {
  const { t } = useTranslation();
  const { connected } = useWallet();

  const features = [
    {
      icon: Shield,
      title: 'Non-Custodial',
      description: 'Keep full control of your Bitcoin while earning staking rewards through ECDSA proof verification.',
      color: 'text-blue-400',
    },
    {
      icon: TrendingUp,
      title: 'Multi-Chain Staking',
      description: 'Protocol stakes across SOL (40%), ETH (30%), and ATOM (30%) to generate rewards.',
      color: 'text-green-400',
    },
    {
      icon: Zap,
      title: 'Enterprise Security',
      description: '2-of-3 multisig with Yubico HSMs, optional KYC, and 2FA authentication.',
      color: 'text-vault-accent',
    },
  ];

  const stats = [
    {
      label: t('dashboard.btcCommitted'),
      value: '0.00 BTC',
      change: '+0%',
      trend: 'neutral' as const,
    },
    {
      label: t('dashboard.rewardsEarned'),
      value: '$0.00',
      change: '+0%',
      trend: 'neutral' as const,
    },
    {
      label: t('dashboard.treasuryAssets'),
      value: '$0.00',
      change: '+0%',
      trend: 'neutral' as const,
    },
    {
      label: t('dashboard.totalRewards'),
      value: '$0.00',
      change: '+0%',
      trend: 'neutral' as const,
    },
  ];

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <motion.section
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6 }}
        className="text-center mb-16"
      >
        <div className="flex justify-center mb-6">
          <div className="w-16 h-16 bg-gradient-to-br from-vault-accent to-orange-400 rounded-2xl flex items-center justify-center">
            <Bitcoin className="w-8 h-8 text-white" />
          </div>
        </div>

        <h1 className="text-5xl md:text-6xl font-bold mb-6">
          <span className="text-gradient">Vault Protocol</span>
        </h1>

        <p className="text-xl text-gray-300 max-w-3xl mx-auto mb-8 leading-relaxed">
          A security-focused, non-custodial Bitcoin protocol that allows users to commit BTC
          without transferring custody while earning rewards from protocol-owned staking activities.
        </p>

        {!connected && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
            className="bg-vault-accent/10 border border-vault-accent/20 rounded-lg p-4 max-w-md mx-auto mb-8"
          >
            <p className="text-vault-accent text-sm font-medium">
              Connect your wallet to get started with Bitcoin staking
            </p>
          </motion.div>
        )}
      </motion.section>

      {/* Stats Section */}
      <motion.section
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, delay: 0.2 }}
        className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-16"
      >
        {stats.map((stat, index) => (
          <StatsCard key={stat.label} {...stat} delay={index * 0.1} />
        ))}
      </motion.section>

      {/* Features Section */}
      <motion.section
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, delay: 0.4 }}
        className="mb-16"
      >
        <h2 className="text-3xl font-bold text-center mb-12">
          Why Choose Vault Protocol?
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <FeatureCard key={feature.title} {...feature} delay={index * 0.1} />
          ))}
        </div>
      </motion.section>

      {/* Action Section */}
      <motion.section
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, delay: 0.6 }}
        className="text-center"
      >
        <h2 className="text-2xl font-bold mb-8">
          Ready to start earning rewards on your Bitcoin?
        </h2>

        <div className="flex flex-col sm:flex-row gap-4 justify-center max-w-2xl mx-auto">
          <Link href="/btc" className="btn-primary flex items-center justify-center space-x-2">
            <Bitcoin className="w-5 h-5" />
            <span>{t('btc.commitTitle')}</span>
            <ArrowRight className="w-4 h-4" />
          </Link>

          <Link href="/rewards" className="btn-success flex items-center justify-center space-x-2">
            <TrendingUp className="w-5 h-5" />
            <span>{t('rewards.claim')}</span>
          </Link>

          <Link href="/kyc" className="btn-secondary flex items-center justify-center space-x-2">
            <Shield className="w-5 h-5" />
            <span>{t('kyc.title')}</span>
          </Link>
        </div>

        <div className="mt-8 text-sm text-gray-400">
          <p>
            Learn more about our{' '}
            <a
              href="#"
              className="text-vault-accent hover:text-orange-400 inline-flex items-center space-x-1"
            >
              <span>security practices</span>
              <ExternalLink className="w-3 h-3" />
            </a>
            {' '}and{' '}
            <a
              href="#"
              className="text-vault-accent hover:text-orange-400 inline-flex items-center space-x-1"
            >
              <span>audit reports</span>
              <ExternalLink className="w-3 h-3" />
            </a>
          </p>
        </div>
      </motion.section>
    </div>
  );
}