'use client';

import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import {
    Settings, Shield, Smartphone, Key, AlertTriangle, CheckCircle,
    Monitor, MapPin, Clock, X, QrCode, Copy, Eye, EyeOff,
    Trash2, RefreshCw, Download, Upload, Lock, Unlock
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { useWallet } from '../components/WalletProvider';
import { VaultClient } from '../lib/vault-client';

interface AuthFactor {
    method: 'totp' | 'webauthn' | 'sms' | 'email' | 'passkey';
    identifier: string;
    enabled: boolean;
    verified: boolean;
    createdAt: Date;
    lastUsed: Date;
    failureCount: number;
    locked: boolean;
}

interface UserSession {
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

interface SecurityEvent {
    eventId: string;
    eventType: 'login_success' | 'login_failure' | 'two_factor_success' | 'two_factor_failure' | 'session_created' | 'session_expired' | 'suspicious_activity' | 'account_locked';
    timestamp: Date;
    details: string;
    riskLevel: number;
    resolved: boolean;
    ipAddress?: string;
    deviceId?: string;
}

interface SecuritySettings {
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

interface AuthStatus {
    isAuthenticated: boolean;
    twoFactorEnabled: boolean;
    authMethods: AuthFactor[];
    activeSessions: UserSession[];
    accountLocked: boolean;
    lastLogin: Date;
    securityEvents: SecurityEvent[];
    securitySettings: SecuritySettings;
}

export default function SecurityPage() {
    const { t } = useTranslation();
    const { wallet, isConnected } = useWallet();
    const [authStatus, setAuthStatus] = useState<AuthStatus | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [showTOTPSetup, setShowTOTPSetup] = useState(false);
    const [showWebAuthnSetup, setShowWebAuthnSetup] = useState(false);
    const [showBackupCodes, setShowBackupCodes] = useState(false);
    const [showSecurityEvents, setShowSecurityEvents] = useState(false);
    const [totpSecret, setTotpSecret] = useState<string>('');
    const [totpQR, setTotpQR] = useState<string>('');
    const [totpCode, setTotpCode] = useState<string>('');
    const [backupCodes, setBackupCodes] = useState<string[]>([]);
    const [securitySettings, setSecuritySettings] = useState<SecuritySettings | null>(null);

    const vaultClient = new VaultClient({
        network: 'devnet',
        programId: process.env.NEXT_PUBLIC_PROGRAM_ID || '',
        rpcEndpoint: process.env.NEXT_PUBLIC_RPC_ENDPOINT || '',
        chainlinkOracles: {
            btcUsd: process.env.NEXT_PUBLIC_BTC_USD_ORACLE || '',
            utxoVerification: process.env.NEXT_PUBLIC_UTXO_ORACLE || ''
        }
    });

    useEffect(() => {
        if (isConnected && wallet) {
            vaultClient.connectWallet(wallet);
            loadAuthStatus();
        }
    }, [isConnected, wallet]);

    const loadAuthStatus = async () => {
        try {
            setLoading(true);

            // Simulate loading auth status
            const mockAuthStatus: AuthStatus = {
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
                lastLogin: new Date(),
                securityEvents: [
                    {
                        eventId: 'event_001',
                        eventType: 'login_success',
                        timestamp: new Date(),
                        details: 'Successful wallet connection',
                        riskLevel: 10,
                        resolved: true,
                        ipAddress: '192.168.1.1',
                        deviceId: 'device_456'
                    }
                ],
                securitySettings: {
                    require2FAForAll: false,
                    require2FAForPayments: true,
                    require2FAForHighValue: true,
                    sessionTimeout: 3600,
                    maxConcurrentSessions: 3,
                    enableEmailNotifications: true,
                    enableSMSNotifications: false,
                    autoLockOnSuspicious: true,
                    trustedDevices: [],
                    ipWhitelist: []
                }
            };

            setAuthStatus(mockAuthStatus);
            setSecuritySettings(mockAuthStatus.securitySettings);
        } catch (err) {
            setError('Failed to load security status');
        } finally {
            setLoading(false);
        }
    };

    const setupTOTP = async () => {
        try {
            setLoading(true);
            setError(null);

            // Generate TOTP secret
            const secret = generateTOTPSecret();
            const qrCode = generateQRCode(secret);

            setTotpSecret(secret);
            setTotpQR(qrCode);
            setShowTOTPSetup(true);
        } catch (err) {
            setError('Failed to setup TOTP');
        } finally {
            setLoading(false);
        }
    };

    const verifyTOTP = async () => {
        if (!totpCode || totpCode.length !== 6) {
            setError('Please enter a valid 6-digit code');
            return;
        }

        try {
            setLoading(true);
            setError(null);

            // Simulate TOTP verification
            await new Promise(resolve => setTimeout(resolve, 1000));

            // Add TOTP to auth methods
            const newAuthMethod: AuthFactor = {
                method: 'totp',
                identifier: 'TOTP Authenticator',
                enabled: true,
                verified: true,
                createdAt: new Date(),
                lastUsed: new Date(),
                failureCount: 0,
                locked: false
            };

            setAuthStatus(prev => prev ? {
                ...prev,
                twoFactorEnabled: true,
                authMethods: [...prev.authMethods, newAuthMethod]
            } : null);

            setShowTOTPSetup(false);
            setTotpCode('');
            setSuccess('TOTP authentication enabled successfully');

            // Generate backup codes
            const codes = generateBackupCodes();
            setBackupCodes(codes);
            setShowBackupCodes(true);

        } catch (err) {
            setError('Failed to verify TOTP code');
        } finally {
            setLoading(false);
        }
    };

    const setupWebAuthn = async () => {
        try {
            setLoading(true);
            setError(null);

            // Check WebAuthn support
            if (!window.navigator.credentials) {
                setError('WebAuthn is not supported in this browser');
                return;
            }

            // Create credential
            const credential = await navigator.credentials.create({
                publicKey: {
                    challenge: new Uint8Array(32),
                    rp: {
                        name: "Vault Protocol",
                        id: window.location.hostname,
                    },
                    user: {
                        id: new TextEncoder().encode(wallet?.publicKey?.toString() || ''),
                        name: wallet?.publicKey?.toString() || '',
                        displayName: "Vault User",
                    },
                    pubKeyCredParams: [{ alg: -7, type: "public-key" }],
                    authenticatorSelection: {
                        authenticatorAttachment: "platform",
                        userVerification: "required"
                    },
                    timeout: 60000,
                    attestation: "direct"
                }
            });

            if (credential) {
                const newAuthMethod: AuthFactor = {
                    method: 'webauthn',
                    identifier: 'WebAuthn Device',
                    enabled: true,
                    verified: true,
                    createdAt: new Date(),
                    lastUsed: new Date(),
                    failureCount: 0,
                    locked: false
                };

                setAuthStatus(prev => prev ? {
                    ...prev,
                    twoFactorEnabled: true,
                    authMethods: [...prev.authMethods, newAuthMethod]
                } : null);

                setSuccess('WebAuthn device registered successfully');
            }

        } catch (err) {
            setError('Failed to setup WebAuthn. Please try again.');
        } finally {
            setLoading(false);
        }
    };

    const removeAuthMethod = async (method: AuthFactor) => {
        try {
            setLoading(true);
            setError(null);

            // Remove auth method
            setAuthStatus(prev => prev ? {
                ...prev,
                authMethods: prev.authMethods.filter(m => m.identifier !== method.identifier),
                twoFactorEnabled: prev.authMethods.length > 1
            } : null);

            setSuccess(`${method.identifier} removed successfully`);
        } catch (err) {
            setError('Failed to remove authentication method');
        } finally {
            setLoading(false);
        }
    };

    const revokeSession = async (sessionId: string) => {
        try {
            setLoading(true);
            setError(null);

            // Revoke session
            setAuthStatus(prev => prev ? {
                ...prev,
                activeSessions: prev.activeSessions.map(session =>
                    session.sessionId === sessionId
                        ? { ...session, status: 'revoked' as const }
                        : session
                )
            } : null);

            setSuccess('Session revoked successfully');
        } catch (err) {
            setError('Failed to revoke session');
        } finally {
            setLoading(false);
        }
    };

    const updateSecuritySettings = async (settings: Partial<SecuritySettings>) => {
        try {
            setLoading(true);
            setError(null);

            setSecuritySettings(prev => prev ? { ...prev, ...settings } : null);
            setAuthStatus(prev => prev ? {
                ...prev,
                securitySettings: { ...prev.securitySettings, ...settings }
            } : null);

            setSuccess('Security settings updated successfully');
        } catch (err) {
            setError('Failed to update security settings');
        } finally {
            setLoading(false);
        }
    };

    const generateTOTPSecret = (): string => {
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
        let secret = '';
        for (let i = 0; i < 32; i++) {
            secret += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        return secret;
    };

    const generateQRCode = (secret: string): string => {
        const issuer = 'Vault Protocol';
        const accountName = wallet?.publicKey?.toString().slice(0, 8) || 'user';
        const otpauth = `otpauth://totp/${issuer}:${accountName}?secret=${secret}&issuer=${issuer}`;
        return `https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${encodeURIComponent(otpauth)}`;
    };

    const generateBackupCodes = (): string[] => {
        const codes = [];
        for (let i = 0; i < 10; i++) {
            const code = Math.random().toString(36).substring(2, 10).toUpperCase();
            codes.push(code);
        }
        return codes;
    };

    const copyToClipboard = (text: string) => {
        navigator.clipboard.writeText(text);
        setSuccess('Copied to clipboard');
        setTimeout(() => setSuccess(null), 2000);
    };

    const downloadBackupCodes = () => {
        const content = backupCodes.join('\n');
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'vault-backup-codes.txt';
        a.click();
        URL.revokeObjectURL(url);
    };

    const getSessionRiskColor = (riskScore: number) => {
        if (riskScore < 30) return 'text-green-400';
        if (riskScore < 70) return 'text-yellow-400';
        return 'text-red-400';
    };

    const getEventTypeIcon = (eventType: string) => {
        switch (eventType) {
            case 'login_success': return <CheckCircle className="w-4 h-4 text-green-400" />;
            case 'login_failure': return <X className="w-4 h-4 text-red-400" />;
            case 'suspicious_activity': return <AlertTriangle className="w-4 h-4 text-yellow-400" />;
            case 'account_locked': return <Lock className="w-4 h-4 text-red-400" />;
            default: return <Shield className="w-4 h-4 text-gray-400" />;
        }
    };

    if (!isConnected) {
        return (
            <div className="container mx-auto px-4 py-8">
                <div className="text-center">
                    <Shield className="w-16 h-16 text-gray-400 mx-auto mb-4" />
                    <h1 className="text-2xl font-bold text-white mb-2">Connect Wallet</h1>
                    <p className="text-gray-400">Please connect your wallet to access security settings</p>
                </div>
            </div>
        );
    }

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
                        <h1 className="text-3xl font-bold text-white">Security Settings</h1>
                        <p className="text-gray-400">Manage your account security and authentication</p>
                    </div>
                </div>

                {/* Alerts */}
                <AnimatePresence>
                    {error && (
                        <motion.div
                            initial={{ opacity: 0, y: -10 }}
                            animate={{ opacity: 1, y: 0 }}
                            exit={{ opacity: 0, y: -10 }}
                            className="bg-vault-error/10 border border-vault-error/20 rounded-lg p-4 mb-6"
                        >
                            <div className="flex items-center space-x-2">
                                <AlertTriangle className="w-5 h-5 text-vault-error" />
                                <p className="text-vault-error text-sm">{error}</p>
                                <button onClick={() => setError(null)} className="ml-auto">
                                    <X className="w-4 h-4 text-vault-error" />
                                </button>
                            </div>
                        </motion.div>
                    )}

                    {success && (
                        <motion.div
                            initial={{ opacity: 0, y: -10 }}
                            animate={{ opacity: 1, y: 0 }}
                            exit={{ opacity: 0, y: -10 }}
                            className="bg-vault-success/10 border border-vault-success/20 rounded-lg p-4 mb-6"
                        >
                            <div className="flex items-center space-x-2">
                                <CheckCircle className="w-5 h-5 text-vault-success" />
                                <p className="text-vault-success text-sm">{success}</p>
                                <button onClick={() => setSuccess(null)} className="ml-auto">
                                    <X className="w-4 h-4 text-vault-success" />
                                </button>
                            </div>
                        </motion.div>
                    )}
                </AnimatePresence>

                {/* Security Overview */}
                {authStatus && (
                    <div className="card mb-8">
                        <h2 className="text-xl font-semibold mb-4">Security Status</h2>
                        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <div className="flex items-center space-x-3">
                                <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${authStatus.twoFactorEnabled ? 'bg-vault-success/20' : 'bg-vault-error/20'
                                    }`}>
                                    {authStatus.twoFactorEnabled ? (
                                        <Shield className="w-5 h-5 text-vault-success" />
                                    ) : (
                                        <AlertTriangle className="w-5 h-5 text-vault-error" />
                                    )}
                                </div>
                                <div>
                                    <h3 className="font-medium">Two-Factor Authentication</h3>
                                    <p className={`text-sm ${authStatus.twoFactorEnabled ? 'text-vault-success' : 'text-vault-error'}`}>
                                        {authStatus.twoFactorEnabled ? 'Enabled' : 'Disabled'}
                                    </p>
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

                            <div className="flex items-center space-x-3">
                                <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${authStatus.accountLocked ? 'bg-vault-error/20' : 'bg-vault-success/20'
                                    }`}>
                                    {authStatus.accountLocked ? (
                                        <Lock className="w-5 h-5 text-vault-error" />
                                    ) : (
                                        <Unlock className="w-5 h-5 text-vault-success" />
                                    )}
                                </div>
                                <div>
                                    <h3 className="font-medium">Account Status</h3>
                                    <p className={`text-sm ${authStatus.accountLocked ? 'text-vault-error' : 'text-vault-success'}`}>
                                        {authStatus.accountLocked ? 'Locked' : 'Active'}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                )}

                {/* 2FA Setup */}
                <div className="card mb-8">
                    <div className="flex items-center justify-between mb-4">
                        <div className="flex items-center space-x-3">
                            <Smartphone className="w-6 h-6 text-vault-accent" />
                            <h2 className="text-xl font-semibold">Two-Factor Authentication</h2>
                        </div>
                        {authStatus?.twoFactorEnabled && (
                            <span className="px-2 py-1 bg-vault-success/20 text-vault-success text-xs rounded">
                                Enabled
                            </span>
                        )}
                    </div>

                    <p className="text-gray-400 mb-6">
                        Add an extra layer of security to your account with two-factor authentication.
                    </p>

                    {/* Current Auth Methods */}
                    {authStatus?.authMethods.length > 0 && (
                        <div className="mb-6">
                            <h3 className="font-medium mb-3">Active Authentication Methods</h3>
                            <div className="space-y-3">
                                {authStatus.authMethods.map((method, index) => (
                                    <div key={index} className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                                        <div className="flex items-center space-x-3">
                                            {method.method === 'totp' && <Smartphone className="w-5 h-5 text-vault-success" />}
                                            {method.method === 'webauthn' && <Key className="w-5 h-5 text-vault-success" />}
                                            {method.method === 'passkey' && <Key className="w-5 h-5 text-vault-success" />}
                                            <div>
                                                <h4 className="font-medium">{method.identifier}</h4>
                                                <p className="text-sm text-gray-400">
                                                    Added {method.createdAt.toLocaleDateString()} •
                                                    Last used {method.lastUsed.toLocaleDateString()}
                                                </p>
                                            </div>
                                        </div>
                                        <div className="flex items-center space-x-2">
                                            {method.verified && (
                                                <CheckCircle className="w-4 h-4 text-vault-success" />
                                            )}
                                            <button
                                                onClick={() => removeAuthMethod(method)}
                                                className="p-2 text-gray-400 hover:text-vault-error transition-colors"
                                                title="Remove method"
                                            >
                                                <Trash2 className="w-4 h-4" />
                                            </button>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}

                    {/* Add New Methods */}
                    <div className="space-y-4">
                        <div className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                            <div className="flex items-center space-x-3">
                                <Smartphone className="w-5 h-5 text-gray-400" />
                                <div>
                                    <h3 className="font-medium">Authenticator App</h3>
                                    <p className="text-sm text-gray-400">Use Google Authenticator, Authy, or similar</p>
                                </div>
                            </div>
                            <button
                                onClick={setupTOTP}
                                disabled={loading}
                                className="btn-primary text-sm py-2 px-4 disabled:opacity-50"
                            >
                                {loading ? 'Setting up...' : 'Enable'}
                            </button>
                        </div>

                        <div className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                            <div className="flex items-center space-x-3">
                                <Key className="w-5 h-5 text-gray-400" />
                                <div>
                                    <h3 className="font-medium">Hardware Key / Passkey</h3>
                                    <p className="text-sm text-gray-400">YubiKey, TouchID, FaceID, or Windows Hello</p>
                                </div>
                            </div>
                            <button
                                onClick={setupWebAuthn}
                                disabled={loading}
                                className="btn-secondary text-sm py-2 px-4 disabled:opacity-50"
                            >
                                {loading ? 'Setting up...' : 'Add Key'}
                            </button>
                        </div>
                    </div>
                </div>

                {/* Security Settings */}
                {securitySettings && (
                    <div className="card mb-8">
                        <h2 className="text-xl font-semibold mb-4">Security Preferences</h2>
                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="font-medium">Require 2FA for all operations</h3>
                                    <p className="text-sm text-gray-400">Require two-factor authentication for all account actions</p>
                                </div>
                                <label className="relative inline-flex items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        checked={securitySettings.require2FAForAll}
                                        onChange={(e) => updateSecuritySettings({ require2FAForAll: e.target.checked })}
                                        className="sr-only peer"
                                    />
                                    <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-vault-accent"></div>
                                </label>
                            </div>

                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="font-medium">Require 2FA for payments</h3>
                                    <p className="text-sm text-gray-400">Require two-factor authentication for reward payments</p>
                                </div>
                                <label className="relative inline-flex items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        checked={securitySettings.require2FAForPayments}
                                        onChange={(e) => updateSecuritySettings({ require2FAForPayments: e.target.checked })}
                                        className="sr-only peer"
                                    />
                                    <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-vault-accent"></div>
                                </label>
                            </div>

                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="font-medium">Auto-lock on suspicious activity</h3>
                                    <p className="text-sm text-gray-400">Automatically lock account when suspicious activity is detected</p>
                                </div>
                                <label className="relative inline-flex items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        checked={securitySettings.autoLockOnSuspicious}
                                        onChange={(e) => updateSecuritySettings({ autoLockOnSuspicious: e.target.checked })}
                                        className="sr-only peer"
                                    />
                                    <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-vault-accent"></div>
                                </label>
                            </div>

                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="font-medium">Session timeout</h3>
                                    <p className="text-sm text-gray-400">Automatically log out after period of inactivity</p>
                                </div>
                                <select
                                    value={securitySettings.sessionTimeout}
                                    onChange={(e) => updateSecuritySettings({ sessionTimeout: parseInt(e.target.value) })}
                                    className="bg-gray-800 border border-gray-700 rounded px-3 py-1 text-sm"
                                >
                                    <option value={1800}>30 minutes</option>
                                    <option value={3600}>1 hour</option>
                                    <option value={7200}>2 hours</option>
                                    <option value={14400}>4 hours</option>
                                    <option value={28800}>8 hours</option>
                                </select>
                            </div>
                        </div>
                    </div>
                )}

                {/* Active Sessions */}
                {authStatus && (
                    <div className="card mb-8">
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-xl font-semibold">Active Sessions</h2>
                            <button
                                onClick={loadAuthStatus}
                                className="p-2 text-gray-400 hover:text-white transition-colors"
                                title="Refresh sessions"
                            >
                                <RefreshCw className="w-4 h-4" />
                            </button>
                        </div>
                        <p className="text-gray-400 mb-6">
                            Monitor and manage your active sessions across different devices.
                        </p>

                        <div className="space-y-3">
                            {authStatus.activeSessions.map((session) => (
                                <div key={session.sessionId} className="flex items-center justify-between p-4 border border-gray-700 rounded-lg">
                                    <div className="flex items-center space-x-3">
                                        <Monitor className="w-5 h-5 text-gray-400" />
                                        <div>
                                            <div className="flex items-center space-x-2">
                                                <h3 className="font-medium">
                                                    {session.userAgent.includes('Chrome') ? 'Chrome' :
                                                        session.userAgent.includes('Firefox') ? 'Firefox' :
                                                            session.userAgent.includes('Safari') ? 'Safari' : 'Unknown Browser'}
                                                </h3>
                                                {session.isCurrent && (
                                                    <span className="text-xs bg-vault-success/20 text-vault-success px-2 py-1 rounded">
                                                        Current
                                                    </span>
                                                )}
                                            </div>
                                            <div className="flex items-center space-x-4 text-sm text-gray-400">
                                                <div className="flex items-center space-x-1">
                                                    <MapPin className="w-3 h-3" />
                                                    <span>{session.ipAddress}</span>
                                                </div>
                                                <div className="flex items-center space-x-1">
                                                    <Clock className="w-3 h-3" />
                                                    <span>Last active {session.lastActivity.toLocaleString()}</span>
                                                </div>
                                                <div className="flex items-center space-x-1">
                                                    <Shield className="w-3 h-3" />
                                                    <span className={getSessionRiskColor(session.riskScore)}>
                                                        Risk: {session.riskScore}/100
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    {!session.isCurrent && session.status === 'active' && (
                                        <button
                                            onClick={() => revokeSession(session.sessionId)}
                                            className="btn-secondary text-sm py-1 px-3"
                                        >
                                            Revoke
                                        </button>
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>
                )}

                {/* Security Events */}
                {authStatus && (
                    <div className="card">
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-xl font-semibold">Recent Security Events</h2>
                            <button
                                onClick={() => setShowSecurityEvents(!showSecurityEvents)}
                                className="btn-secondary text-sm py-2 px-4"
                            >
                                {showSecurityEvents ? 'Hide' : 'Show All'}
                            </button>
                        </div>

                        <div className="space-y-3">
                            {authStatus.securityEvents.slice(0, showSecurityEvents ? undefined : 5).map((event) => (
                                <div key={event.eventId} className="flex items-center space-x-3 p-3 border border-gray-700 rounded-lg">
                                    {getEventTypeIcon(event.eventType)}
                                    <div className="flex-1">
                                        <p className="font-medium">{event.details}</p>
                                        <div className="flex items-center space-x-4 text-sm text-gray-400">
                                            <span>{event.timestamp.toLocaleString()}</span>
                                            {event.ipAddress && (
                                                <span>IP: {event.ipAddress}</span>
                                            )}
                                            <span className={getSessionRiskColor(event.riskLevel)}>
                                                Risk: {event.riskLevel}/100
                                            </span>
                                        </div>
                                    </div>
                                    {event.resolved && (
                                        <CheckCircle className="w-4 h-4 text-vault-success" />
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>
                )}

                {/* TOTP Setup Modal */}
                <AnimatePresence>
                    {showTOTPSetup && (
                        <motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            exit={{ opacity: 0 }}
                            className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4"
                            onClick={() => setShowTOTPSetup(false)}
                        >
                            <motion.div
                                initial={{ scale: 0.9 }}
                                animate={{ scale: 1 }}
                                exit={{ scale: 0.9 }}
                                className="bg-gray-900 rounded-lg max-w-md w-full p-6"
                                onClick={(e) => e.stopPropagation()}
                            >
                                <div className="flex items-center justify-between mb-4">
                                    <h3 className="text-xl font-semibold">Setup Authenticator App</h3>
                                    <button
                                        onClick={() => setShowTOTPSetup(false)}
                                        className="p-2 text-gray-400 hover:text-white transition-colors"
                                    >
                                        <X className="w-5 h-5" />
                                    </button>
                                </div>

                                <div className="space-y-4">
                                    <div className="text-center">
                                        <p className="text-gray-400 mb-4">
                                            Scan this QR code with your authenticator app:
                                        </p>
                                        {totpQR && (
                                            <img src={totpQR} alt="TOTP QR Code" className="mx-auto mb-4" />
                                        )}
                                    </div>

                                    <div>
                                        <label className="block text-sm font-medium text-gray-300 mb-2">
                                            Or enter this secret manually:
                                        </label>
                                        <div className="flex items-center space-x-2">
                                            <input
                                                type="text"
                                                value={totpSecret}
                                                readOnly
                                                className="flex-1 bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm font-mono"
                                            />
                                            <button
                                                onClick={() => copyToClipboard(totpSecret)}
                                                className="p-2 text-gray-400 hover:text-white transition-colors"
                                                title="Copy secret"
                                            >
                                                <Copy className="w-4 h-4" />
                                            </button>
                                        </div>
                                    </div>

                                    <div>
                                        <label className="block text-sm font-medium text-gray-300 mb-2">
                                            Enter the 6-digit code from your app:
                                        </label>
                                        <input
                                            type="text"
                                            value={totpCode}
                                            onChange={(e) => setTotpCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                                            placeholder="000000"
                                            className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-center font-mono text-lg"
                                            maxLength={6}
                                        />
                                    </div>

                                    <div className="flex space-x-3">
                                        <button
                                            onClick={() => setShowTOTPSetup(false)}
                                            className="flex-1 btn-secondary"
                                        >
                                            Cancel
                                        </button>
                                        <button
                                            onClick={verifyTOTP}
                                            disabled={loading || totpCode.length !== 6}
                                            className="flex-1 btn-primary disabled:opacity-50"
                                        >
                                            {loading ? 'Verifying...' : 'Verify & Enable'}
                                        </button>
                                    </div>
                                </div>
                            </motion.div>
                        </motion.div>
                    )}
                </AnimatePresence>

                {/* Backup Codes Modal */}
                <AnimatePresence>
                    {showBackupCodes && (
                        <motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            exit={{ opacity: 0 }}
                            className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4"
                            onClick={() => setShowBackupCodes(false)}
                        >
                            <motion.div
                                initial={{ scale: 0.9 }}
                                animate={{ scale: 1 }}
                                exit={{ scale: 0.9 }}
                                className="bg-gray-900 rounded-lg max-w-md w-full p-6"
                                onClick={(e) => e.stopPropagation()}
                            >
                                <div className="flex items-center justify-between mb-4">
                                    <h3 className="text-xl font-semibold">Backup Codes</h3>
                                    <button
                                        onClick={() => setShowBackupCodes(false)}
                                        className="p-2 text-gray-400 hover:text-white transition-colors"
                                    >
                                        <X className="w-5 h-5" />
                                    </button>
                                </div>

                                <div className="space-y-4">
                                    <div className="bg-vault-warning/10 border border-vault-warning/20 rounded-lg p-4">
                                        <p className="text-vault-warning text-sm">
                                            Save these backup codes in a secure location. Each code can only be used once.
                                        </p>
                                    </div>

                                    <div className="grid grid-cols-2 gap-2">
                                        {backupCodes.map((code, index) => (
                                            <div key={index} className="bg-gray-800 rounded px-3 py-2 font-mono text-sm text-center">
                                                {code}
                                            </div>
                                        ))}
                                    </div>

                                    <div className="flex space-x-3">
                                        <button
                                            onClick={downloadBackupCodes}
                                            className="flex-1 btn-secondary flex items-center justify-center space-x-2"
                                        >
                                            <Download className="w-4 h-4" />
                                            <span>Download</span>
                                        </button>
                                        <button
                                            onClick={() => copyToClipboard(backupCodes.join('\n'))}
                                            className="flex-1 btn-secondary flex items-center justify-center space-x-2"
                                        >
                                            <Copy className="w-4 h-4" />
                                            <span>Copy</span>
                                        </button>
                                    </div>

                                    <button
                                        onClick={() => setShowBackupCodes(false)}
                                        className="w-full btn-primary"
                                    >
                                        I've Saved My Backup Codes
                                    </button>
                                </div>
                            </motion.div>
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>
        </div>
    );
}