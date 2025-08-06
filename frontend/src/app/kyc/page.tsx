'use client';

import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { Shield, CheckCircle, Clock, AlertCircle, Upload, FileText, User, MapPin, Building, X, Eye, Download } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { useWallet } from '../components/WalletProvider';
import { VaultClient } from '../lib/vault-client';

interface KYCDocument {
    type: 'passport' | 'drivers_license' | 'national_id' | 'proof_of_address' | 'bank_statement' | 'tax_document' | 'corporate_registration' | 'beneficial_ownership';
    file: File | null;
    hash?: string;
    uploaded: boolean;
    verified: boolean;
    uploadDate?: Date;
    verificationDate?: Date;
    expiryDate?: Date;
}

interface KYCProfile {
    user: string;
    tier: 'none' | 'basic' | 'enhanced' | 'institutional';
    status: 'not_started' | 'pending' | 'approved' | 'rejected' | 'expired' | 'suspended';
    documents: KYCDocument[];
    commitmentLimit: number;
    dailyLimit: number;
    monthlyVolume: number;
    lastScreeningDate: number;
    kycExpiryDate?: number;
    complianceScreening?: {
        riskLevel: 'low' | 'medium' | 'high' | 'prohibited';
        sanctionsMatch: boolean;
        pepMatch: boolean;
        adverseMedia: boolean;
        screeningDate: number;
        expiryDate: number;
    };
}

const DOCUMENT_TYPES = {
    passport: { label: 'Passport', icon: User, required: ['basic', 'enhanced'] },
    drivers_license: { label: 'Driver\'s License', icon: User, required: [] },
    national_id: { label: 'National ID', icon: User, required: [] },
    proof_of_address: { label: 'Proof of Address', icon: MapPin, required: ['basic', 'enhanced'] },
    bank_statement: { label: 'Bank Statement', icon: FileText, required: ['enhanced'] },
    tax_document: { label: 'Tax Document', icon: FileText, required: [] },
    corporate_registration: { label: 'Corporate Registration', icon: Building, required: ['institutional'] },
    beneficial_ownership: { label: 'Beneficial Ownership', icon: Building, required: ['institutional'] }
};

const KYC_TIERS = {
    none: { label: 'No KYC', limit: '1 BTC', color: 'text-gray-400', bgColor: 'bg-gray-500/20' },
    basic: { label: 'Basic KYC', limit: '10 BTC', color: 'text-blue-400', bgColor: 'bg-blue-500/20' },
    enhanced: { label: 'Enhanced KYC', limit: '100 BTC', color: 'text-green-400', bgColor: 'bg-green-500/20' },
    institutional: { label: 'Institutional KYC', limit: 'Unlimited', color: 'text-purple-400', bgColor: 'bg-purple-500/20' }
};

export default function KYCPage() {
    const { t } = useTranslation();
    const { wallet, isConnected } = useWallet();
    const [kycProfile, setKycProfile] = useState<KYCProfile | null>(null);
    const [selectedTier, setSelectedTier] = useState<'basic' | 'enhanced' | 'institutional'>('basic');
    const [documents, setDocuments] = useState<Record<string, KYCDocument>>({});
    const [loading, setLoading] = useState(false);
    const [uploading, setUploading] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [dragOver, setDragOver] = useState<string | null>(null);
    const [showDocumentViewer, setShowDocumentViewer] = useState<string | null>(null);

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
            loadKYCProfile();
        }
    }, [isConnected, wallet]);

    const loadKYCProfile = async () => {
        try {
            setLoading(true);
            // Simulate loading KYC profile
            const mockProfile: KYCProfile = {
                user: wallet?.publicKey?.toString() || '',
                tier: 'none',
                status: 'not_started',
                documents: [],
                commitmentLimit: 100_000_000, // 1 BTC in satoshis
                dailyLimit: 10_000_000, // 0.1 BTC
                monthlyVolume: 0,
                lastScreeningDate: 0,
            };
            setKycProfile(mockProfile);
        } catch (err) {
            setError('Failed to load KYC profile');
        } finally {
            setLoading(false);
        }
    };

    const handleFileUpload = useCallback(async (documentType: string, file: File) => {
        if (!file) return;

        // Validate file
        const maxSize = 10 * 1024 * 1024; // 10MB
        const allowedTypes = ['image/jpeg', 'image/png', 'application/pdf'];

        if (file.size > maxSize) {
            setError('File size must be less than 10MB');
            return;
        }

        if (!allowedTypes.includes(file.type)) {
            setError('Only JPG, PNG, and PDF files are allowed');
            return;
        }

        try {
            setUploading(documentType);
            setError(null);

            // Calculate file hash
            const arrayBuffer = await file.arrayBuffer();
            const hashBuffer = await crypto.subtle.digest('SHA-256', arrayBuffer);
            const hashArray = Array.from(new Uint8Array(hashBuffer));
            const hash = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

            // Create document entry
            const document: KYCDocument = {
                type: documentType as any,
                file,
                hash,
                uploaded: true,
                verified: false,
                uploadDate: new Date()
            };

            setDocuments(prev => ({
                ...prev,
                [documentType]: document
            }));

            setSuccess(`${DOCUMENT_TYPES[documentType as keyof typeof DOCUMENT_TYPES].label} uploaded successfully`);

            // Auto-clear success message
            setTimeout(() => setSuccess(null), 3000);

        } catch (err) {
            setError('Failed to upload document');
        } finally {
            setUploading(null);
        }
    }, []);

    const handleDrop = useCallback((e: React.DragEvent, documentType: string) => {
        e.preventDefault();
        setDragOver(null);

        const files = Array.from(e.dataTransfer.files);
        if (files.length > 0) {
            handleFileUpload(documentType, files[0]);
        }
    }, [handleFileUpload]);

    const handleDragOver = useCallback((e: React.DragEvent, documentType: string) => {
        e.preventDefault();
        setDragOver(documentType);
    }, []);

    const handleDragLeave = useCallback((e: React.DragEvent) => {
        e.preventDefault();
        setDragOver(null);
    }, []);

    const startKYCVerification = async () => {
        if (!kycProfile) return;

        try {
            setLoading(true);
            setError(null);

            // Check required documents
            const requiredDocs = Object.entries(DOCUMENT_TYPES)
                .filter(([_, config]) => config.required.includes(selectedTier))
                .map(([type]) => type);

            const missingDocs = requiredDocs.filter(type => !documents[type]?.uploaded);

            if (missingDocs.length > 0) {
                setError(`Please upload required documents: ${missingDocs.map(type => DOCUMENT_TYPES[type as keyof typeof DOCUMENT_TYPES].label).join(', ')}`);
                return;
            }

            // Submit KYC verification
            const documentHashes = Object.entries(documents)
                .filter(([_, doc]) => doc.uploaded)
                .map(([type, doc]) => ({ type, hash: doc.hash }));

            // Simulate API call
            await new Promise(resolve => setTimeout(resolve, 2000));

            // Update profile status
            setKycProfile(prev => prev ? {
                ...prev,
                status: 'pending',
                tier: selectedTier
            } : null);

            setSuccess('KYC verification submitted successfully. You will be notified once the review is complete.');

        } catch (err) {
            setError('Failed to submit KYC verification');
        } finally {
            setLoading(false);
        }
    };

    const removeDocument = (documentType: string) => {
        setDocuments(prev => {
            const updated = { ...prev };
            delete updated[documentType];
            return updated;
        });
    };

    const getStatusIcon = (status: string) => {
        switch (status) {
            case 'approved': return <CheckCircle className="w-5 h-5 text-vault-success" />;
            case 'pending': return <Clock className="w-5 h-5 text-vault-warning" />;
            case 'rejected': return <AlertCircle className="w-5 h-5 text-vault-error" />;
            default: return <AlertCircle className="w-5 h-5 text-gray-400" />;
        }
    };

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'approved': return 'text-vault-success';
            case 'pending': return 'text-vault-warning';
            case 'rejected': return 'text-vault-error';
            default: return 'text-gray-400';
        }
    };

    if (!isConnected) {
        return (
            <div className="container mx-auto px-4 py-8">
                <div className="text-center">
                    <Shield className="w-16 h-16 text-gray-400 mx-auto mb-4" />
                    <h1 className="text-2xl font-bold text-white mb-2">Connect Wallet</h1>
                    <p className="text-gray-400">Please connect your wallet to access KYC verification</p>
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
                    <div className="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-400 rounded-xl flex items-center justify-center">
                        <Shield className="w-6 h-6 text-white" />
                    </div>
                    <div>
                        <h1 className="text-3xl font-bold text-white">KYC Verification</h1>
                        <p className="text-gray-400">Verify your identity to increase commitment limits</p>
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
                                <AlertCircle className="w-5 h-5 text-vault-error" />
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

                {/* Current Status */}
                {kycProfile && (
                    <div className="card mb-8">
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-xl font-semibold">Current Status</h2>
                            <div className="flex items-center space-x-2">
                                {getStatusIcon(kycProfile.status)}
                                <span className={`font-medium ${getStatusColor(kycProfile.status)}`}>
                                    {kycProfile.status.replace('_', ' ').toUpperCase()}
                                </span>
                            </div>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                            <div>
                                <p className="text-sm text-gray-400 mb-1">Current Tier</p>
                                <p className="font-medium">{KYC_TIERS[kycProfile.tier].label}</p>
                            </div>
                            <div>
                                <p className="text-sm text-gray-400 mb-1">Commitment Limit</p>
                                <p className="font-medium">{KYC_TIERS[kycProfile.tier].limit}</p>
                            </div>
                            <div>
                                <p className="text-sm text-gray-400 mb-1">Daily Limit</p>
                                <p className="font-medium">{(kycProfile.dailyLimit / 100_000_000).toFixed(2)} BTC</p>
                            </div>
                            <div>
                                <p className="text-sm text-gray-400 mb-1">Monthly Volume</p>
                                <p className="font-medium">{(kycProfile.monthlyVolume / 100_000_000).toFixed(2)} BTC</p>
                            </div>
                        </div>

                        {kycProfile.complianceScreening && (
                            <div className="bg-gray-800/50 rounded-lg p-4">
                                <h3 className="font-medium mb-2">Compliance Screening</h3>
                                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                                    <div>
                                        <p className="text-gray-400">Risk Level</p>
                                        <p className={`font-medium ${kycProfile.complianceScreening.riskLevel === 'low' ? 'text-green-400' :
                                                kycProfile.complianceScreening.riskLevel === 'medium' ? 'text-yellow-400' :
                                                    kycProfile.complianceScreening.riskLevel === 'high' ? 'text-orange-400' :
                                                        'text-red-400'
                                            }`}>
                                            {kycProfile.complianceScreening.riskLevel.toUpperCase()}
                                        </p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400">Sanctions</p>
                                        <p className={`font-medium ${kycProfile.complianceScreening.sanctionsMatch ? 'text-red-400' : 'text-green-400'}`}>
                                            {kycProfile.complianceScreening.sanctionsMatch ? 'MATCH' : 'CLEAR'}
                                        </p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400">PEP</p>
                                        <p className={`font-medium ${kycProfile.complianceScreening.pepMatch ? 'text-yellow-400' : 'text-green-400'}`}>
                                            {kycProfile.complianceScreening.pepMatch ? 'MATCH' : 'CLEAR'}
                                        </p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400">Adverse Media</p>
                                        <p className={`font-medium ${kycProfile.complianceScreening.adverseMedia ? 'text-yellow-400' : 'text-green-400'}`}>
                                            {kycProfile.complianceScreening.adverseMedia ? 'FOUND' : 'CLEAR'}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                )}

                {/* Tier Selection */}
                {kycProfile?.status === 'not_started' && (
                    <div className="card mb-8">
                        <h2 className="text-xl font-semibold mb-4">Select Verification Tier</h2>
                        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                            {(['basic', 'enhanced', 'institutional'] as const).map((tier) => (
                                <button
                                    key={tier}
                                    onClick={() => setSelectedTier(tier)}
                                    className={`p-4 border rounded-lg text-left transition-all ${selectedTier === tier
                                            ? 'border-vault-accent bg-vault-accent/10'
                                            : 'border-gray-700 hover:border-gray-600'
                                        }`}
                                >
                                    <h3 className="font-medium mb-2">{KYC_TIERS[tier].label}</h3>
                                    <p className="text-2xl font-bold text-vault-accent mb-2">{KYC_TIERS[tier].limit}</p>
                                    <p className="text-sm text-gray-400">
                                        {tier === 'basic' && 'Identity verification required'}
                                        {tier === 'enhanced' && 'Enhanced due diligence'}
                                        {tier === 'institutional' && 'Corporate verification'}
                                    </p>
                                </button>
                            ))}
                        </div>

                        {/* Required Documents */}
                        <div className="bg-gray-800/50 rounded-lg p-4">
                            <h3 className="font-medium mb-2">Required Documents for {KYC_TIERS[selectedTier].label}</h3>
                            <div className="flex flex-wrap gap-2">
                                {Object.entries(DOCUMENT_TYPES)
                                    .filter(([_, config]) => config.required.includes(selectedTier))
                                    .map(([type, config]) => (
                                        <span key={type} className="px-2 py-1 bg-vault-accent/20 text-vault-accent text-xs rounded">
                                            {config.label}
                                        </span>
                                    ))}
                            </div>
                        </div>
                    </div>
                )}

                {/* Document Upload */}
                {(kycProfile?.status === 'not_started' || kycProfile?.status === 'rejected') && (
                    <div className="card mb-8">
                        <h2 className="text-xl font-semibold mb-4">Upload Documents</h2>
                        <p className="text-gray-400 mb-6">
                            Upload the required documents for {KYC_TIERS[selectedTier].label} verification.
                        </p>

                        <div className="space-y-6">
                            {Object.entries(DOCUMENT_TYPES)
                                .filter(([_, config]) => config.required.includes(selectedTier))
                                .map(([type, config]) => {
                                    const Icon = config.icon;
                                    const document = documents[type];
                                    const isUploading = uploading === type;
                                    const isDragOver = dragOver === type;

                                    return (
                                        <div key={type} className="space-y-2">
                                            <label className="block text-sm font-medium text-gray-300">
                                                {config.label}
                                                <span className="text-vault-error ml-1">*</span>
                                            </label>

                                            {document?.uploaded ? (
                                                <div className="flex items-center justify-between p-4 border border-vault-success/50 bg-vault-success/10 rounded-lg">
                                                    <div className="flex items-center space-x-3">
                                                        <Icon className="w-5 h-5 text-vault-success" />
                                                        <div>
                                                            <p className="font-medium text-vault-success">{document.file?.name}</p>
                                                            <p className="text-xs text-gray-400">
                                                                Uploaded {document.uploadDate?.toLocaleDateString()}
                                                                {document.verified && ' • Verified'}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div className="flex items-center space-x-2">
                                                        <button
                                                            onClick={() => setShowDocumentViewer(type)}
                                                            className="p-2 text-gray-400 hover:text-white transition-colors"
                                                            title="View document"
                                                        >
                                                            <Eye className="w-4 h-4" />
                                                        </button>
                                                        <button
                                                            onClick={() => removeDocument(type)}
                                                            className="p-2 text-gray-400 hover:text-vault-error transition-colors"
                                                            title="Remove document"
                                                        >
                                                            <X className="w-4 h-4" />
                                                        </button>
                                                    </div>
                                                </div>
                                            ) : (
                                                <div
                                                    onDrop={(e) => handleDrop(e, type)}
                                                    onDragOver={(e) => handleDragOver(e, type)}
                                                    onDragLeave={handleDragLeave}
                                                    className={`border-2 border-dashed rounded-lg p-6 text-center transition-all cursor-pointer ${isDragOver
                                                            ? 'border-vault-accent bg-vault-accent/10'
                                                            : 'border-gray-600 hover:border-vault-accent/50'
                                                        }`}
                                                >
                                                    <input
                                                        type="file"
                                                        accept=".jpg,.jpeg,.png,.pdf"
                                                        onChange={(e) => {
                                                            const file = e.target.files?.[0];
                                                            if (file) handleFileUpload(type, file);
                                                        }}
                                                        className="hidden"
                                                        id={`upload-${type}`}
                                                    />
                                                    <label htmlFor={`upload-${type}`} className="cursor-pointer">
                                                        {isUploading ? (
                                                            <div className="flex items-center justify-center space-x-2">
                                                                <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-vault-accent"></div>
                                                                <span className="text-vault-accent">Uploading...</span>
                                                            </div>
                                                        ) : (
                                                            <>
                                                                <Upload className="w-8 h-8 text-gray-400 mx-auto mb-2" />
                                                                <p className="text-gray-400">Click to upload or drag and drop</p>
                                                                <p className="text-xs text-gray-500 mt-1">PNG, JPG, PDF up to 10MB</p>
                                                            </>
                                                        )}
                                                    </label>
                                                </div>
                                            )}
                                        </div>
                                    );
                                })}
                        </div>

                        <div className="mt-8 flex justify-end">
                            <button
                                onClick={startKYCVerification}
                                disabled={loading || Object.entries(DOCUMENT_TYPES)
                                    .filter(([_, config]) => config.required.includes(selectedTier))
                                    .some(([type]) => !documents[type]?.uploaded)}
                                className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                {loading ? (
                                    <div className="flex items-center space-x-2">
                                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                                        <span>Submitting...</span>
                                    </div>
                                ) : (
                                    'Submit for Verification'
                                )}
                            </button>
                        </div>
                    </div>
                )}

                {/* Document Viewer Modal */}
                <AnimatePresence>
                    {showDocumentViewer && documents[showDocumentViewer] && (
                        <motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            exit={{ opacity: 0 }}
                            className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4"
                            onClick={() => setShowDocumentViewer(null)}
                        >
                            <motion.div
                                initial={{ scale: 0.9 }}
                                animate={{ scale: 1 }}
                                exit={{ scale: 0.9 }}
                                className="bg-gray-900 rounded-lg max-w-4xl max-h-[90vh] overflow-hidden"
                                onClick={(e) => e.stopPropagation()}
                            >
                                <div className="flex items-center justify-between p-4 border-b border-gray-700">
                                    <h3 className="font-medium">
                                        {DOCUMENT_TYPES[showDocumentViewer as keyof typeof DOCUMENT_TYPES].label}
                                    </h3>
                                    <button
                                        onClick={() => setShowDocumentViewer(null)}
                                        className="p-2 text-gray-400 hover:text-white transition-colors"
                                    >
                                        <X className="w-5 h-5" />
                                    </button>
                                </div>
                                <div className="p-4">
                                    <p className="text-gray-400 text-center">
                                        Document preview not available in demo mode
                                    </p>
                                </div>
                            </motion.div>
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>
        </div>
    );
}