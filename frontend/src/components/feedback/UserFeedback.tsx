/**
 * User Feedback Components
 * Comprehensive user feedback system with recovery suggestions
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { VaultError, ErrorSeverity, ErrorCategory } from '../../lib/errors/error-handler';

// Feedback message component
interface FeedbackMessageProps {
    type: 'success' | 'error' | 'warning' | 'info';
    title: string;
    message: string;
    actions?: Array<{
        label: string;
        action: () => void;
        style?: 'primary' | 'secondary' | 'danger';
    }>;
    dismissible?: boolean;
    onDismiss?: () => void;
    className?: string;
}

export const FeedbackMessage: React.FC<FeedbackMessageProps> = ({
    type,
    title,
    message,
    actions = [],
    dismissible = true,
    onDismiss,
    className = '',
}) => {
    const [isVisible, setIsVisible] = useState(true);

    const getIcon = () => {
        switch (type) {
            case 'success': return '✅';
            case 'error': return '❌';
            case 'warning': return '⚠️';
            case 'info': return 'ℹ️';
            default: return 'ℹ️';
        }
    };

    const getColorClasses = () => {
        switch (type) {
            case 'success':
                return 'bg-green-50 border-green-200 text-green-800';
            case 'error':
                return 'bg-red-50 border-red-200 text-red-800';
            case 'warning':
                return 'bg-yellow-50 border-yellow-200 text-yellow-800';
            case 'info':
                return 'bg-blue-50 border-blue-200 text-blue-800';
            default:
                return 'bg-gray-50 border-gray-200 text-gray-800';
        }
    };

    const handleDismiss = () => {
        setIsVisible(false);
        setTimeout(() => onDismiss?.(), 300);
    };

    if (!isVisible) return null;

    return (
        <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            className={`border rounded-lg p-4 ${getColorClasses()} ${className}`}
        >
            <div className="flex items-start justify-between">
                <div className="flex items-start space-x-3">
                    <div className="text-xl">{getIcon()}</div>
                    <div className="flex-1">
                        <h3 className="font-semibold text-sm">{title}</h3>
                        <p className="text-sm mt-1">{message}</p>

                        {actions.length > 0 && (
                            <div className="mt-3 flex space-x-2">
                                {actions.map((action, index) => (
                                    <button
                                        key={index}
                                        onClick={action.action}
                                        className={`
                      px-3 py-1 text-xs rounded transition-colors
                      ${action.style === 'danger'
                                                ? 'bg-red-600 text-white hover:bg-red-700'
                                                : action.style === 'primary'
                                                    ? 'bg-blue-600 text-white hover:bg-blue-700'
                                                    : 'bg-gray-600 text-white hover:bg-gray-700'
                                            }
                    `}
                                    >
                                        {action.label}
                                    </button>
                                ))}
                            </div>
                        )}
                    </div>
                </div>

                {dismissible && (
                    <button
                        onClick={handleDismiss}
                        className="text-gray-400 hover:text-gray-600 transition-colors ml-2"
                    >
                        ✕
                    </button>
                )}
            </div>
        </motion.div>
    );
};

// Error display component with recovery suggestions
interface ErrorDisplayProps {
    error: VaultError;
    onRetry?: () => void;
    onDismiss?: () => void;
    showDetails?: boolean;
}

export const ErrorDisplay: React.FC<ErrorDisplayProps> = ({
    error,
    onRetry,
    onDismiss,
    showDetails = false,
}) => {
    const [showFullDetails, setShowFullDetails] = useState(false);

    const getSeverityColor = (severity: ErrorSeverity) => {
        switch (severity) {
            case ErrorSeverity.CRITICAL: return 'text-red-900 bg-red-50 border-red-300';
            case ErrorSeverity.HIGH: return 'text-red-800 bg-red-50 border-red-200';
            case ErrorSeverity.MEDIUM: return 'text-yellow-800 bg-yellow-50 border-yellow-200';
            case ErrorSeverity.LOW: return 'text-blue-800 bg-blue-50 border-blue-200';
            default: return 'text-gray-800 bg-gray-50 border-gray-200';
        }
    };

    const getCategoryIcon = (category: ErrorCategory) => {
        switch (category) {
            case ErrorCategory.NETWORK: return '🌐';
            case ErrorCategory.VALIDATION: return '📝';
            case ErrorCategory.AUTHENTICATION: return '🔐';
            case ErrorCategory.AUTHORIZATION: return '🚫';
            case ErrorCategory.TRANSACTION: return '💳';
            case ErrorCategory.SYSTEM: return '⚙️';
            case ErrorCategory.SECURITY: return '🔒';
            case ErrorCategory.COMPLIANCE: return '📋';
            default: return '❌';
        }
    };

    return (
        <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            className={`border rounded-lg p-4 ${getSeverityColor(error.severity)}`}
        >
            <div className="flex items-start justify-between">
                <div className="flex items-start space-x-3 flex-1">
                    <div className="text-2xl">{getCategoryIcon(error.category)}</div>
                    <div className="flex-1">
                        <div className="flex items-center space-x-2 mb-2">
                            <h3 className="font-semibold text-sm">
                                {error.userMessage}
                            </h3>
                            <span className="text-xs px-2 py-1 bg-white bg-opacity-50 rounded">
                                {error.severity.toUpperCase()}
                            </span>
                        </div>

                        {error.recoveryActions && error.recoveryActions.length > 0 && (
                            <div className="mb-3">
                                <p className="text-xs font-medium mb-1">Suggested actions:</p>
                                <ul className="text-xs space-y-1">
                                    {error.recoveryActions.map((action, index) => (
                                        <li key={index} className="flex items-start space-x-1">
                                            <span>•</span>
                                            <span>{action}</span>
                                        </li>
                                    ))}
                                </ul>
                            </div>
                        )}

                        <div className="flex items-center space-x-2">
                            {onRetry && (
                                <button
                                    onClick={onRetry}
                                    className="text-xs bg-white bg-opacity-75 px-3 py-1 rounded hover:bg-opacity-100 transition-all"
                                >
                                    Try Again
                                </button>
                            )}

                            {showDetails && (
                                <button
                                    onClick={() => setShowFullDetails(!showFullDetails)}
                                    className="text-xs text-opacity-75 hover:text-opacity-100 transition-opacity"
                                >
                                    {showFullDetails ? 'Hide Details' : 'Show Details'}
                                </button>
                            )}
                        </div>

                        {showFullDetails && (
                            <motion.div
                                initial={{ opacity: 0, height: 0 }}
                                animate={{ opacity: 1, height: 'auto' }}
                                className="mt-3 pt-3 border-t border-current border-opacity-20"
                            >
                                <div className="text-xs space-y-1">
                                    <p><strong>Error Code:</strong> {error.code}</p>
                                    <p><strong>Category:</strong> {error.category}</p>
                                    <p><strong>Timestamp:</strong> {new Date(error.timestamp).toLocaleString()}</p>
                                    {error.details && (
                                        <details className="mt-2">
                                            <summary className="cursor-pointer">Technical Details</summary>
                                            <pre className="mt-1 text-xs bg-black bg-opacity-10 p-2 rounded overflow-auto">
                                                {JSON.stringify(error.details, null, 2)}
                                            </pre>
                                        </details>
                                    )}
                                </div>
                            </motion.div>
                        )}
                    </div>
                </div>

                {onDismiss && (
                    <button
                        onClick={onDismiss}
                        className="text-current text-opacity-50 hover:text-opacity-100 transition-opacity ml-2"
                    >
                        ✕
                    </button>
                )}
            </div>
        </motion.div>
    );
};

// Success feedback component
interface SuccessFeedbackProps {
    title: string;
    message: string;
    transactionHash?: string;
    onViewTransaction?: () => void;
    onContinue?: () => void;
    onDismiss?: () => void;
}

export const SuccessFeedback: React.FC<SuccessFeedbackProps> = ({
    title,
    message,
    transactionHash,
    onViewTransaction,
    onContinue,
    onDismiss,
}) => {
    return (
        <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="bg-green-50 border border-green-200 rounded-lg p-6 text-center"
        >
            <div className="text-green-500 text-5xl mb-4">✅</div>
            <h2 className="text-xl font-semibold text-green-900 mb-2">{title}</h2>
            <p className="text-green-800 mb-4">{message}</p>

            {transactionHash && (
                <div className="bg-white bg-opacity-50 rounded p-3 mb-4">
                    <p className="text-sm text-green-700 mb-1">Transaction Hash:</p>
                    <code className="text-xs text-green-900 break-all">
                        {transactionHash}
                    </code>
                </div>
            )}

            <div className="flex justify-center space-x-3">
                {onViewTransaction && (
                    <button
                        onClick={onViewTransaction}
                        className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                    >
                        View Transaction
                    </button>
                )}

                {onContinue && (
                    <button
                        onClick={onContinue}
                        className="px-4 py-2 bg-white text-green-600 border border-green-600 rounded hover:bg-green-50 transition-colors"
                    >
                        Continue
                    </button>
                )}

                {onDismiss && (
                    <button
                        onClick={onDismiss}
                        className="px-4 py-2 text-green-600 hover:text-green-800 transition-colors"
                    >
                        Dismiss
                    </button>
                )}
            </div>
        </motion.div>
    );
};

// Inline validation feedback
interface InlineValidationProps {
    isValid: boolean;
    message: string;
    showSuccess?: boolean;
}

export const InlineValidation: React.FC<InlineValidationProps> = ({
    isValid,
    message,
    showSuccess = false,
}) => {
    if (isValid && !showSuccess) return null;

    return (
        <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            className={`flex items-center space-x-2 mt-1 text-sm ${isValid ? 'text-green-600' : 'text-red-600'
                }`}
        >
            <span>{isValid ? '✓' : '✗'}</span>
            <span>{message}</span>
        </motion.div>
    );
};

// Form validation summary
interface ValidationSummaryProps {
    errors: Array<{
        field: string;
        message: string;
    }>;
    onFieldFocus?: (field: string) => void;
}

export const ValidationSummary: React.FC<ValidationSummaryProps> = ({
    errors,
    onFieldFocus,
}) => {
    if (errors.length === 0) return null;

    return (
        <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-red-50 border border-red-200 rounded-lg p-4 mb-4"
        >
            <div className="flex items-start space-x-3">
                <div className="text-red-500 text-xl">❌</div>
                <div className="flex-1">
                    <h3 className="font-semibold text-red-800 text-sm mb-2">
                        Please correct the following errors:
                    </h3>
                    <ul className="space-y-1">
                        {errors.map((error, index) => (
                            <li key={index} className="text-sm text-red-700">
                                <button
                                    onClick={() => onFieldFocus?.(error.field)}
                                    className="hover:underline text-left"
                                >
                                    <strong>{error.field}:</strong> {error.message}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            </div>
        </motion.div>
    );
};

export default {
    FeedbackMessage,
    ErrorDisplay,
    SuccessFeedback,
    InlineValidation,
    ValidationSummary,
};