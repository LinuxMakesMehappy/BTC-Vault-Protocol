'use client';

import { useTranslation } from 'react-i18next';
import { Settings, Shield, Smartphone, Key, AlertTriangle } from 'lucide-react';
import { motion } from 'framer-motion';

export default function SecurityPage() {
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
                    <div className="w-12 h-12 bg-gradient-to-br from-red-500 to-red-400 rounded-xl flex items-center justify-center">
                        <Settings className="w-6 h-6 text-white" />
                    </div>
                    <div>
                        <h1 className="text-3xl font-bold text-white">{t('security.title')}</h1>
                        <p className="text-gray-400">Manage your account security and authentication</p>
                    </div>
                </div>

                {/* Security Overview */}
                <div className="card mb-8">
                    <h2 className="text-xl font-semibold mb-4">Security Status</h2>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div className="flex items-center space-x-3">
                            <div className="w-10 h-10 bg-vault-error/20 rounded-lg flex items-center justify-center">
                                <AlertTriangle className="w-5 h-5 text-vault-error" />
                            </div>
                            <div>
                                <h3 className="font-medium">Two-Factor Authentication</h3>
                                <p className="text-sm text-vault-error">{t('security.disabled')}</p>
                            </div>
                        </div>

                        <div className="flex items-center space-x-3">
                            <div className="w-10 h-10 bg-vault-success/20 rounded-lg flex items-center justify-center">
                                <Shield className="w-5 h-5 text-vault-success" />
                            </div>
                            <div>
                                <h3 className="font-medium">Wallet Connection</h3>
                                <p className="text-sm text-vault-success">Secure</p>
                            </div>
                        </div>
                    </div>
                </div>

                {/* 2FA Setup */}
                <div className="card mb-8">
                    <div className="flex items-center space-x-3 mb-4">
                        <Smartphone className="w-6 h-6 text-vault-accent" />
                        <h2 className="text-xl font-semibold">{t('security.twoFA')}</h2>
                    </div>

                    <p className="text-gray-400 mb-6">
                        Add an extra layer of security to your account with two-factor authentication.
                    </p>

                    <div className="space-y-4 mb-6">
                        <div className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                            <div className="flex items-center space-x-3">
                                <Smartphone className="w-5 h-5 text-gray-400" />
                                <div>
                                    <h3 className="font-medium">Authenticator App</h3>
                                    <p className="text-sm text-gray-400">Use Google Authenticator or similar</p>
                                </div>
                            </div>
                            <button className="btn-primary text-sm py-2 px-4">
                                {t('security.enable')}
                            </button>
                        </div>

                        <div className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                            <div className="flex items-center space-x-3">
                                <Key className="w-5 h-5 text-gray-400" />
                                <div>
                                    <h3 className="font-medium">Hardware Key</h3>
                                    <p className="text-sm text-gray-400">YubiKey or other FIDO2 device</p>
                                </div>
                            </div>
                            <button className="btn-secondary text-sm py-2 px-4">
                                Add Key
                            </button>
                        </div>
                    </div>
                </div>

                {/* Session Management */}
                <div className="card">
                    <h2 className="text-xl font-semibold mb-4">Active Sessions</h2>
                    <p className="text-gray-400 mb-6">
                        Monitor and manage your active sessions across different devices.
                    </p>

                    <div className="space-y-3">
                        <div className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                            <div>
                                <h3 className="font-medium">Current Session</h3>
                                <p className="text-sm text-gray-400">Chrome on Windows • Active now</p>
                            </div>
                            <span className="text-xs bg-vault-success/20 text-vault-success px-2 py-1 rounded">
                                Current
                            </span>
                        </div>
                    </div>
                </div>
            </motion.div>
        </div>
    );
}