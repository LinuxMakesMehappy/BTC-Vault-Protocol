'use client';

import React, { useState, useEffect } from 'react';
import { useWallet } from './WalletProvider';
import { useToast } from './ToastProvider';
import { VaultClient } from '../lib/vault-client';
import { RewardSummary, PaymentHistory } from '../types/vault';

interface RewardClaimData {
    amount: string;
    paymentMethod: 'btc' | 'usdc';
    autoReinvest: boolean;
}

// Comprehensive reward claiming interface with BTC/USDC payment selection
export default function RewardManager() {
    const { wallet, connected } = useWallet();
    const { showToast } = useToast();
    const [rewards, setRewards] = useState<RewardSummary | null>(null);
    const [paymentHistory, setPaymentHistory] = useState<PaymentHistory[]>([]);
    const [loading, setLoading] = useState(true);
    const [claiming, setClaiming] = useState(false);
    const [autoReinvest, setAutoReinvest] = useState(false);
    const [selectedPaymentMethod, setSelectedPaymentMethod] = useState<'btc' | 'usdc'>('btc');
    const [claimAmount, setClaimAmount] = useState('');
    const [showClaimModal, setShowClaimModal] = useState(false);

    const vaultClient = new VaultClient();

    const fetchRewardData = async () => {
        if (!connected || !wallet) return;

        try {
            setLoading(true);

            // Fetch reward summary
            const rewardSummary = await vaultClient.getRewardSummary(wallet.publicKey);
            setRewards(rewardSummary);

            // Fetch payment history
            const history = await vaultClient.getPaymentHistory(wallet.publicKey);
            setPaymentHistory(history.items || []);

            // Get auto-reinvest setting
            const config = await vaultClient.getPaymentConfig();
            setAutoReinvest(config.autoReinvestment.enabled);

        } catch (error) {
            console.error('Failed to fetch reward data:', error);
            showToast('Failed to load reward data', 'error');
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchRewardData();
    }, [connected, wallet]);

    const handleClaimRewards = async () => {
        if (!connected || !wallet || !rewards) return;

        try {
            setClaiming(true);

            const claimData: RewardClaimData = {
                amount: claimAmount || rewards.pending_rewards,
                paymentMethod: selectedPaymentMethod,
                autoReinvest: false
            };

            const signature = await vaultClient.claimRewards(wallet.publicKey);

            showToast(`Rewards claimed successfully! Transaction: ${signature.slice(0, 8)}...`, 'success');

            // Refresh data
            await fetchRewardData();
            setShowClaimModal(false);
            setClaimAmount('');

        } catch (error) {
            console.error('Failed to claim rewards:', error);
            showToast('Failed to claim rewards', 'error');
        } finally {
            setClaiming(false);
        }
    };

    const handleToggleAutoReinvest = async () => {
        if (!connected || !wallet) return;

        try {
            await vaultClient.setAutoReinvest(!autoReinvest);
            setAutoReinvest(!autoReinvest);
            showToast(`Auto-reinvestment ${!autoReinvest ? 'enabled' : 'disabled'}`, 'success');
        } catch (error) {
            console.error('Failed to toggle auto-reinvest:', error);
            showToast('Failed to update auto-reinvest setting', 'error');
        }
    };

    const formatCurrency = (amount: string, currency: 'BTC' | 'USDC' = 'BTC') => {
        const num = parseFloat(amount);
        if (currency === 'BTC') {
            return `${num.toFixed(8)} BTC`;
        }
        return `$${num.toFixed(2)}`;
    };

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'completed': return 'text-green-600 bg-green-100';
            case 'pending': return 'text-yellow-600 bg-yellow-100';
            case 'failed': return 'text-red-600 bg-red-100';
            default: return 'text-gray-600 bg-gray-100';
        }
    };

    if (!connected) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
                <div className="text-center">
                    <div className="w-16 h-16 mx-auto mb-4 bg-blue-100 rounded-full flex items-center justify-center">
                        <svg className="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                        </svg>
                    </div>
                    <h2 className="text-xl font-semibold text-gray-900 mb-2">Connect Your Wallet</h2>
                    <p className="text-gray-600">Please connect your wallet to manage your rewards</p>
                </div>
            </div>
        );
    }

    if (loading) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
                <div className="text-center">
                    <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                    <p className="text-gray-600">Loading reward data...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
            <div className="container mx-auto px-4 py-8">
                {/* Header */}
                <div className="flex justify-between items-center mb-8">
                    <div>
                        <h1 className="text-3xl font-bold text-gray-900">Reward Management</h1>
                        <p className="text-gray-600 mt-1">
                            Manage your rewards, configure payments, and track your earnings.
                        </p>
                    </div>
                    <button
                        onClick={fetchRewardData}
                        className="flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                    >
                        <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                        </svg>
                        Refresh
                    </button>
                </div>

                {/* Reward Summary Cards */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Pending Rewards</p>
                                <p className="text-2xl font-bold text-gray-900">{formatCurrency(rewards?.pending_rewards || '0')}</p>
                                <p className="text-xs text-gray-500 mt-1">Ready to claim</p>
                            </div>
                            <div className="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Total Earned</p>
                                <p className="text-2xl font-bold text-gray-900">{formatCurrency(rewards?.total_rewards || '0')}</p>
                                <p className="text-xs text-gray-500 mt-1">All-time earnings</p>
                            </div>
                            <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Claimed Rewards</p>
                                <p className="text-2xl font-bold text-gray-900">{formatCurrency(rewards?.claimed_rewards || '0')}</p>
                                <p className="text-xs text-gray-500 mt-1">Successfully claimed</p>
                            </div>
                            <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Current APY</p>
                                <p className="text-2xl font-bold text-gray-900">{rewards?.apy || '0'}%</p>
                                <p className="text-xs text-gray-500 mt-1">Annual percentage yield</p>
                            </div>
                            <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z" />
                                </svg>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Reward Actions */}
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
                    {/* Claim Rewards */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <h2 className="text-xl font-semibold text-gray-900 mb-4">Claim Rewards</h2>
                        <div className="space-y-4">
                            <div>
                                <p className="text-sm text-gray-600 mb-2">Available to claim:</p>
                                <p className="text-lg font-semibold text-gray-900">{formatCurrency(rewards?.pending_rewards || '0')}</p>
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">Payment Method</label>
                                <div className="flex space-x-4">
                                    <label className="flex items-center">
                                        <input
                                            type="radio"
                                            name="paymentMethod"
                                            value="btc"
                                            checked={selectedPaymentMethod === 'btc'}
                                            onChange={(e) => setSelectedPaymentMethod(e.target.value as 'btc' | 'usdc')}
                                            className="mr-2"
                                        />
                                        <span className="text-sm">BTC (Lightning)</span>
                                    </label>
                                    <label className="flex items-center">
                                        <input
                                            type="radio"
                                            name="paymentMethod"
                                            value="usdc"
                                            checked={selectedPaymentMethod === 'usdc'}
                                            onChange={(e) => setSelectedPaymentMethod(e.target.value as 'btc' | 'usdc')}
                                            className="mr-2"
                                        />
                                        <span className="text-sm">USDC</span>
                                    </label>
                                </div>
                            </div>

                            <button
                                onClick={() => setShowClaimModal(true)}
                                disabled={!rewards?.pending_rewards || parseFloat(rewards.pending_rewards) === 0 || claiming}
                                className="w-full bg-green-600 text-white py-3 px-4 rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                            >
                                {claiming ? 'Claiming...' : 'Claim Rewards'}
                            </button>
                        </div>
                    </div>

                    {/* Auto-Reinvestment - auto-reinvestment toggle and configuration options */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <h2 className="text-xl font-semibold text-gray-900 mb-4">Auto-Reinvestment</h2>
                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="font-medium text-gray-900">Enable Auto-Reinvestment</p>
                                    <p className="text-sm text-gray-600">Automatically reinvest rewards to compound earnings</p>
                                </div>
                                <button
                                    onClick={handleToggleAutoReinvest}
                                    className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${autoReinvest ? 'bg-green-600' : 'bg-gray-200'
                                        }`}
                                >
                                    <span
                                        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${autoReinvest ? 'translate-x-6' : 'translate-x-1'
                                            }`}
                                    />
                                </button>
                            </div>

                            {autoReinvest && (
                                <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                                    <div className="flex items-center">
                                        <svg className="w-5 h-5 text-green-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <p className="text-sm text-green-800">Auto-reinvestment is enabled. Rewards will be automatically reinvested to maximize your earnings.</p>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* Payment History - payment history and transaction tracking */}
                <div className="bg-white rounded-xl shadow-sm border border-gray-100">
                    <div className="p-6 border-b border-gray-100">
                        <h2 className="text-xl font-semibold text-gray-900">Payment History</h2>
                        <p className="text-gray-600 text-sm mt-1">Track your reward claims and payments - transaction tracking</p>
                    </div>

                    <div className="overflow-x-auto">
                        <table className="w-full">
                            <thead className="bg-gray-50">
                                <tr>
                                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Date</th>
                                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Amount</th>
                                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Method</th>
                                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
                                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Transaction</th>
                                </tr>
                            </thead>
                            <tbody className="bg-white divide-y divide-gray-200">
                                {paymentHistory.length === 0 ? (
                                    <tr>
                                        <td colSpan={5} className="px-6 py-8 text-center text-gray-500">
                                            No payment history yet. Claim your first rewards to see transactions here.
                                        </td>
                                    </tr>
                                ) : (
                                    paymentHistory.map((payment, index) => (
                                        <tr key={index} className="hover:bg-gray-50">
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {new Date(payment.timestamp).toLocaleDateString()}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {formatCurrency(payment.amount.toString(), payment.currency as 'BTC' | 'USDC')}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {payment.currency === 'BTC' ? 'Lightning' : 'USDC'}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(payment.status)}`}>
                                                    {payment.status}
                                                </span>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {payment.id ? `${payment.id.slice(0, 8)}...` : 'N/A'}
                                            </td>
                                        </tr>
                                    ))
                                )}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

            {/* Claim Modal */}
            {showClaimModal && (
                <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div className="bg-white rounded-xl p-6 max-w-md w-full mx-4">
                        <h3 className="text-lg font-semibold text-gray-900 mb-4">Confirm Reward Claim</h3>

                        <div className="space-y-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">Amount to Claim</label>
                                <input
                                    type="text"
                                    value={claimAmount}
                                    onChange={(e) => setClaimAmount(e.target.value)}
                                    placeholder={rewards?.pending_rewards || '0'}
                                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                />
                                <p className="text-xs text-gray-500 mt-1">Leave empty to claim all pending rewards</p>
                            </div>

                            <div>
                                <p className="text-sm font-medium text-gray-700">Payment Method: {selectedPaymentMethod.toUpperCase()}</p>
                                <p className="text-xs text-gray-500">
                                    {selectedPaymentMethod === 'btc' ? 'Lightning Network payment' : 'USDC payment to your wallet'}
                                </p>
                            </div>

                            <div className="flex space-x-4 pt-4">
                                <button
                                    onClick={() => setShowClaimModal(false)}
                                    className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
                                >
                                    Cancel
                                </button>
                                <button
                                    onClick={handleClaimRewards}
                                    disabled={claiming}
                                    className="flex-1 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50 transition-colors"
                                >
                                    {claiming ? 'Claiming...' : 'Confirm Claim'}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}