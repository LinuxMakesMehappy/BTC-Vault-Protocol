'use client';

import React, { useState, useEffect } from 'react';
import { useWallet } from './WalletProvider';
import { useToast } from './ToastProvider';
import { VaultClient } from '../lib/vault-client';
import { BTCCommitment, RewardSummary, TreasuryData } from '../types/vault';

interface DashboardStats {
    btcBalance: string;
    btcCommitted: string;
    totalRewards: string;
    pendingRewards: string;
    claimedRewards: string;
    treasuryValue: string;
    lastUpdate: Date;
    timestamp: string;
}

export default function Dashboard() {
    const { wallet, connected } = useWallet();
    const { showToast } = useToast();
    const [stats, setStats] = useState<DashboardStats | null>(null);
    const [loading, setLoading] = useState(true);
    const [refreshing, setRefreshing] = useState(false);
    const [lastRefresh, setLastRefresh] = useState<Date>(new Date());

    const vaultClient = new VaultClient();

    const fetchDashboardData = async (showRefreshIndicator = false) => {
        if (!connected || !wallet) return;

        try {
            if (showRefreshIndicator) {
                setRefreshing(true);
            } else {
                setLoading(true);
            }

            // Fetch BTC balance from Chainlink oracle
            const btcBalance = await vaultClient.getBTCBalance(wallet.publicKey);

            // Fetch user's BTC commitments
            const commitments = await vaultClient.getUserCommitments(wallet.publicKey);
            const totalCommitted = commitments.reduce((sum, c) => sum + parseFloat(c.btc_amount), 0);

            // Fetch reward summary
            const rewards = await vaultClient.getRewardSummary(wallet.publicKey);

            // Fetch treasury data (public information)
            const treasury = await vaultClient.getTreasuryData();

            setStats({
                btcBalance: btcBalance.toFixed(8),
                btcCommitted: totalCommitted.toFixed(8),
                totalRewards: rewards.total_rewards,
                pendingRewards: rewards.pending_rewards,
                claimedRewards: rewards.claimed_rewards,
                treasuryValue: treasury.total_value_usd,
                lastUpdate: new Date(),
                timestamp: new Date().toISOString()
            });

            setLastRefresh(new Date());
        } catch (error) {
            console.error('Failed to fetch dashboard data:', error);
            showToast('Failed to load dashboard data', 'error');
        } finally {
            setLoading(false);
            setRefreshing(false);
        }
    };

    useEffect(() => {
        fetchDashboardData();
    }, [connected, wallet]);

    // Auto-refresh every 30 seconds
    useEffect(() => {
        if (!connected) return;

        const interval = setInterval(() => {
            fetchDashboardData(true);
        }, 30000);

        return () => clearInterval(interval);
    }, [connected]);

    const handleRefresh = () => {
        fetchDashboardData(true);
    };

    if (!connected) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
                <div className="text-center">
                    <div className="w-16 h-16 mx-auto mb-4 bg-blue-100 rounded-full flex items-center justify-center">
                        <svg className="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                        </svg>
                    </div>
                    <h2 className="text-xl font-semibold text-gray-900 mb-2">Connect Your Wallet</h2>
                    <p className="text-gray-600">Please connect your wallet to view your dashboard</p>
                </div>
            </div>
        );
    }

    if (loading) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
                <div className="text-center">
                    <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                    <p className="text-gray-600">Loading your dashboard...</p>
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
                        <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
                        <p className="text-gray-600 mt-1">
                            Welcome back! Here's your VaultBTC overview.
                        </p>
                    </div>
                    <div className="flex items-center space-x-4">
                        <div className="text-sm text-gray-500">
                            Last updated: {lastRefresh.toLocaleTimeString()}
                        </div>
                        <button
                            onClick={handleRefresh}
                            disabled={refreshing}
                            className="flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                        >
                            <svg
                                className={`w-4 h-4 mr-2 ${refreshing ? 'animate-spin' : ''}`}
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                            </svg>
                            {refreshing ? 'Refreshing...' : 'Refresh'}
                        </button>
                    </div>
                </div>

                {/* Stats Grid */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    {/* BTC Balance */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">BTC Balance</p>
                                <p className="text-2xl font-bold text-gray-900">{stats?.btcBalance || '0.00000000'}</p>
                                <p className="text-xs text-gray-500 mt-1">Live from Chainlink Oracle</p>
                            </div>
                            <div className="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-orange-600" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M23.638 14.904c-1.602 6.43-8.113 10.34-14.542 8.736C2.67 22.05-1.244 15.525.362 9.105 1.962 2.67 8.475-1.243 14.9.358c6.43 1.605 10.342 8.115 8.738 14.546z" />
                                    <path fill="#fff" d="M17.415 11.85c.18-1.2-.735-1.848-1.985-2.277l.405-1.626-1.002-.25-.395 1.585c-.264-.066-.535-.128-.805-.19l.398-1.595-1.002-.25-.405 1.626c-.219-.05-.433-.099-.642-.152l.001-.007-1.382-.345-.266 1.07s.735.169.72.179c.402.1.475.365.463.575l-.463 1.86c.028.007.064.018.104.035l-.107-.027-.65 2.607c-.049.121-.174.303-.454.234.01.015-.72-.179-.72-.179l-.497 1.147 1.305.325c.242.061.479.125.712.185l-.41 1.647 1.002.25.405-1.626c.274.074.54.142.799.207l-.404 1.621 1.002.25.41-1.647c1.69.32 2.963.191 3.499-1.338.432-1.234-.021-1.946-.913-2.407.649-.15 1.138-.578 1.269-1.462zm-2.27 3.184c-.307 1.23-2.379.565-3.05.398l.544-2.181c.671.167 2.829.498 2.506 1.783zm.307-3.201c-.279 1.119-2.006.55-2.565.411l.493-1.978c.559.139 2.36.398 2.072 1.567z" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    {/* BTC Committed */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">BTC Committed</p>
                                <p className="text-2xl font-bold text-gray-900">{stats?.btcCommitted || '0.00000000'}</p>
                                <p className="text-xs text-gray-500 mt-1">Earning rewards</p>
                            </div>
                            <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    {/* Pending Rewards */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Pending Rewards</p>
                                <p className="text-2xl font-bold text-gray-900">{stats?.pendingRewards || '0.00000000'}</p>
                                <p className="text-xs text-gray-500 mt-1">Ready to claim</p>
                            </div>
                            <div className="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                                </svg>
                            </div>
                        </div>
                    </div>

                    {/* Total Rewards */}
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-gray-600">Total Rewards</p>
                                <p className="text-2xl font-bold text-gray-900">{stats?.totalRewards || '0.00000000'}</p>
                                <p className="text-xs text-gray-500 mt-1">All-time earnings</p>
                            </div>
                            <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                                <svg className="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                                </svg>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Treasury Overview */}
                <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100 mb-8">
                    <div className="flex items-center justify-between mb-6">
                        <div>
                            <h2 className="text-xl font-semibold text-gray-900">Treasury Overview</h2>
                            <p className="text-gray-600 text-sm">Protocol treasury performance and USD rewards</p>
                        </div>
                        <div className="text-right">
                            <p className="text-sm text-gray-600">Total Treasury Value</p>
                            <p className="text-2xl font-bold text-gray-900">${stats?.treasuryValue || '0.00'}</p>
                        </div>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div className="text-center p-4 bg-gray-50 rounded-lg">
                            <div className="w-8 h-8 mx-auto mb-2 bg-blue-100 rounded-full flex items-center justify-center">
                                <span className="text-blue-600 font-semibold text-sm">SOL</span>
                            </div>
                            <p className="text-sm text-gray-600">Solana Staking</p>
                            <p className="text-lg font-semibold text-gray-900">40% Allocation</p>
                        </div>
                        <div className="text-center p-4 bg-gray-50 rounded-lg">
                            <div className="w-8 h-8 mx-auto mb-2 bg-purple-100 rounded-full flex items-center justify-center">
                                <span className="text-purple-600 font-semibold text-sm">ETH</span>
                            </div>
                            <p className="text-sm text-gray-600">Ethereum L2</p>
                            <p className="text-lg font-semibold text-gray-900">30% Allocation</p>
                        </div>
                        <div className="text-center p-4 bg-gray-50 rounded-lg">
                            <div className="w-8 h-8 mx-auto mb-2 bg-red-100 rounded-full flex items-center justify-center">
                                <span className="text-red-600 font-semibold text-sm">ATOM</span>
                            </div>
                            <p className="text-sm text-gray-600">Cosmos Staking</p>
                            <p className="text-lg font-semibold text-gray-900">30% Allocation</p>
                        </div>
                    </div>
                </div>

                {/* Quick Actions */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <h3 className="text-lg font-semibold text-gray-900 mb-4">Commit More BTC</h3>
                        <p className="text-gray-600 text-sm mb-4">
                            Increase your BTC commitment to earn more rewards from protocol staking.
                        </p>
                        <button className="w-full bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 transition-colors">
                            Commit BTC
                        </button>
                    </div>

                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <h3 className="text-lg font-semibold text-gray-900 mb-4">Claim Rewards</h3>
                        <p className="text-gray-600 text-sm mb-4">
                            Claim your pending rewards in BTC or USDC, or enable auto-reinvestment.
                        </p>
                        <button
                            className="w-full bg-green-600 text-white py-2 px-4 rounded-lg hover:bg-green-700 transition-colors"
                            disabled={!stats?.pendingRewards || parseFloat(stats.pendingRewards) === 0}
                        >
                            Claim Rewards
                        </button>
                    </div>

                    <div className="bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                        <h3 className="text-lg font-semibold text-gray-900 mb-4">Security Settings</h3>
                        <p className="text-gray-600 text-sm mb-4">
                            Manage your 2FA, passkeys, and security preferences for your account.
                        </p>
                        <button className="w-full bg-gray-600 text-white py-2 px-4 rounded-lg hover:bg-gray-700 transition-colors">
                            Security Settings
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}