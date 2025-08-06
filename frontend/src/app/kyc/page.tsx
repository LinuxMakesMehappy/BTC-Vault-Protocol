'use client';

import { useTranslation } from 'react-i18next';
import { Shield, CheckCircle, Clock, AlertCircle } from 'lucide-react';
import { motion } from 'framer-motion';

export default function KYCPage() {
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
                    <div className="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-400 rounded-xl flex items-center justify-center">
                        <Shield className="w-6 h-6 text-white" />
                    </div>
                    <div>
                        <h1 className="text-3xl font-bold text-white">{t('kyc.title')}</h1>
                        <p className="text-gray-400">Verify your identity to increase commitment limits</p>
                    </div>
                </div>

                {/* KYC Status */}
                <div className="card mb-8">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-xl font-semibold">Verification Status</h2>
                        <div className="flex items-center space-x-2">
                            <AlertCircle className="w-5 h-5 text-vault-warning" />
                            <span className="text-vault-warning font-medium">{t('kyc.none')}</span>
                        </div>
                    </div>

                    <div className="bg-vault-warning/10 border border-vault-warning/20 rounded-lg p-4 mb-6">
                        <p className="text-vault-warning text-sm">
                            {t('kyc.required')}
                        </p>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div className="text-center p-4 border border-gray-700 rounded-lg">
                            <h3 className="font-medium mb-2">No KYC</h3>
                            <p className="text-2xl font-bold text-vault-accent mb-2">1 BTC</p>
                            <p className="text-sm text-gray-400">Maximum commitment</p>
                        </div>

                        <div className="text-center p-4 border border-gray-700 rounded-lg opacity-50">
                            <h3 className="font-medium mb-2">Basic KYC</h3>
                            <p className="text-2xl font-bold text-gray-400 mb-2">10 BTC</p>
                            <p className="text-sm text-gray-400">Requires verification</p>
                        </div>

                        <div className="text-center p-4 border border-gray-700 rounded-lg opacity-50">
                            <h3 className="font-medium mb-2">Enhanced KYC</h3>
                            <p className="text-2xl font-bold text-gray-400 mb-2">Unlimited</p>
                            <p className="text-sm text-gray-400">Full verification</p>
                        </div>
                    </div>
                </div>

                {/* Document Upload */}
                <div className="card">
                    <h2 className="text-xl font-semibold mb-4">Start Verification</h2>
                    <p className="text-gray-400 mb-6">
                        Upload the required documents to begin your KYC verification process.
                    </p>

                    <div className="space-y-4 mb-6">
                        <div>
                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                Government-issued ID
                            </label>
                            <div className="border-2 border-dashed border-gray-600 rounded-lg p-6 text-center hover:border-vault-accent/50 transition-colors cursor-pointer">
                                <p className="text-gray-400">Click to upload or drag and drop</p>
                                <p className="text-xs text-gray-500 mt-1">PNG, JPG, PDF up to 10MB</p>
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                Proof of Address
                            </label>
                            <div className="border-2 border-dashed border-gray-600 rounded-lg p-6 text-center hover:border-vault-accent/50 transition-colors cursor-pointer">
                                <p className="text-gray-400">Click to upload or drag and drop</p>
                                <p className="text-xs text-gray-500 mt-1">PNG, JPG, PDF up to 10MB</p>
                            </div>
                        </div>
                    </div>

                    <button className="btn-primary" disabled>
                        {t('kyc.submit')}
                    </button>
                </div>
            </motion.div>
        </div>
    );
}