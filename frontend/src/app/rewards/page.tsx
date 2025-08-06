'use client';

import { useTranslation } from 'react-i18next';
import { Gift, TrendingUp, Clock, DollarSign } from 'lucide-react';
import { motion } from 'framer-motion';

export default function RewardsPage() {
    const { t } = useTranslation();

    return (
        <div className="container mx-auto px-4 py-8">
            <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6 }}
            >
                {/* Header */}
                <div className="flex items-center space-x-3 mb-8">
                    <div className="w-12 h-12 bg-gradient-to-br from-vault-success to-green-400 rounded-xl flex items-center justify-center">
                        <Gift className="w-6 h-6 text-white" />
                    </div>
                    <div>
                        <h1 className="text-3xl font-bold text-white">{t('rewards.title')}</h1>
                        <p className="text-gray-400">Manage your staking rewards and payment preferences</p>
                    </div>
                </div>

                {/* Rewards Overview */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                    <div className="stat-card">
                        <div className="flex items-center space-x-3 mb-2">
                            <TrendingUp className="w-5 h-5 text-vault-success" />
                            <h3 className="text-sm font-medium text-gray-400">Total Earned</h3>
                        </div>
                        <p className="text-2xl font-bold text-vault-success">$0.00</p>
                    </div>

                    <div className="stat-card">
                        <div className="flex items-center space-x-3 mb-2">
                            <Clock className="w-5 h-5 text-vault-warning" />
                            <h3 className="text-sm font-medium text-gray-400">Pending</h3>
                        </div>
                        <p className="text-2xl font-bold text-vault-warning">$0.00</p>
                    </div>

                    <div className="stat-card">
                        <div className="flex items-center space-x-3 mb-2">
                            <DollarSign className="w-5 h-5 text-vault-accent" />
                            <h3 className="text-sm font-medium text-gray-400">Available</h3>
                        </div>
                        <p className="text-2xl font-bold text-vault-accent">$0.00</p>
                    </div>
                </div>

                {/* Payment Preferences */}
                <div className="card mb-8">
                    <h2 className="text-xl font-semibold mb-4">Payment Preferences</h2>
                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                {t('rewards.paymentType')}
                            </label>
                            <select className="input-field">
                                <option value="btc">{t('rewards.btcPayment')}</option>
                                <option value="usdc">{t('rewards.usdcPayment')}</option>
                                <option value="reinvest">{t('rewards.autoReinvest')}</option>
                            </select>
                        </div>

                        <div className="flex items-center space-x-3">
                            <input type="checkbox" id="autoReinvest" className="rounded" />
                            <label htmlFor="autoReinvest" className="text-sm text-gray-300">
                                Enable auto-reinvestment for future rewards
                            </label>
                        </div>
                    </div>
                </div>

                {/* Claim Rewards */}
                <div className="card">
                    <h2 className="text-xl font-semibold mb-4">Claim Rewards</h2>
                    <p className="text-gray-400 mb-6">
                        No rewards available to claim at this time. Start by committing BTC to earn staking rewards.
                    </p>
                    <button className="btn-success" disabled>
                        {t('rewards.claim')}
                    </button>
                </div>
            </motion.div>
        </div>
    );
}