/**
 * Comprehensive Error Handler Component
 * Complete error handling solution with recovery, notifications, and progress tracking
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { VaultError, ErrorSeverity } from '../../lib/errors/error-handler';
import { RecoveryAction } from '../../lib/errors/recovery-actions';
import useErrorHandling from '../../hooks/useErrorHandling';
import { useNotifications } from './NotificationSystem';
import { ErrorDisplay } from './UserFeedback';
import RecoveryInterface from './RecoveryInterface';
import ProgressTracker, { ProgressStep, useProgressTracker } from './ProgressTracker';
import { LoadingSpinner, TransactionProgress, TransactionStage } from './LoadingStates';

export interface ComprehensiveErrorHandlerProps {
    children: React.ReactNode;
    enableAutoRecovery?: boolean;
    showProgressTracking?: boolean;
    enableNotifications?: boolean;
    maxRetryAttempts?: number;
    onErrorResolved?: () => void;
    onCriticalError?: (error: VaultError) => void;
}

export const ComprehensiveErrorHandler: React.FC<ComprehensiveErrorHandlerProps> = ({
    children,
    enableAutoRecovery = true,
    showProgressTracking = true,
    enableNotifications = true,
    maxRetryAttempts = 3,
    onErrorResolved,
    onCriticalError,
}) => {
    const [showRecoveryInterface, setShowRecoveryInterface] = useState(false);
    const [currentOperation, setCurrentOperation] = useState<string | null>(null);
    const [operationProgress, setOperationProgress] = useState<ProgressStep[]>([]);

    const errorHandling = useErrorHandling({
        autoRecover: enableAutoRecovery,
        showNotifications: enableNotifications,
        retryAttempts: maxRetryAttempts,
    });

    const { showToast } = useNotifications();
    const progressTracker = useProgressTracker(operationProgress);

    // Handle critical errors
    useEffect(() => {
        if (errorHandling.error?.severity === ErrorSeverity.CRITICAL) {
            onCriticalError?.(errorHandling.error);
            setShowRecoveryInterface(true);
        }
    }, [errorHandling.error, onCriticalError]);

    // Handle error resolution
    useEffect(() => {
        if (!errorHandling.hasError && showRecoveryInterface) {
            setShowRecoveryInterface(false);
            onErrorResolved?.();
            showToast('success', 'Issue resolved successfully');
        }
    }, [errorHandling.hasError, showRecoveryInterface, onErrorResolved, showToast]);

    // Create progress steps for common operations
    const createProgressSteps = useCallback((operationType: string): ProgressStep[] => {
        const commonSteps: Record<string, ProgressStep[]> = {
            'btc_commitment': [
                {
                    id: 'validate_input',
                    label: 'Validating Input',
                    description: 'Checking Bitcoin address and amount',
                    status: 'pending',
                    estimatedTime: 2,
                },
                {
                    id: 'generate_proof',
                    label: 'Generating Proof',
                    description: 'Creating ECDSA proof for commitment',
                    status: 'pending',
                    estimatedTime: 5,
                },
                {
                    id: 'submit_transaction',
                    label: 'Submitting Transaction',
                    description: 'Broadcasting to Solana network',
                    status: 'pending',
                    estimatedTime: 10,
                },
                {
                    id: 'confirm_transaction',
                    label: 'Confirming Transaction',
                    description: 'Waiting for network confirmation',
                    status: 'pending',
                    estimatedTime: 15,
                },
            ],
            'reward_claim': [
                {
                    id: 'calculate_rewards',
                    label: 'Calculating Rewards',
                    description: 'Computing available rewards',
                    status: 'pending',
                    estimatedTime: 3,
                },
                {
                    id: 'prepare_payment',
                    label: 'Preparing Payment',
                    description: 'Setting up payment method',
                    status: 'pending',
                    estimatedTime: 5,
                },
                {
                    id: 'process_payment',
                    label: 'Processing Payment',
                    description: 'Executing reward payment',
                    status: 'pending',
                    estimatedTime: 12,
                },
            ],
            'wallet_connection': [
                {
                    id: 'detect_wallet',
                    label: 'Detecting Wallet',
                    description: 'Looking for wallet extension',
                    status: 'pending',
                    estimatedTime: 2,
                },
                {
                    id: 'request_connection',
                    label: 'Requesting Connection',
                    description: 'Asking for wallet permission',
                    status: 'pending',
                    estimatedTime: 5,
                },
                {
                    id: 'verify_connection',
                    label: 'Verifying Connection',
                    description: 'Confirming wallet access',
                    status: 'pending',
                    estimatedTime: 3,
                },
            ],
        };

        return commonSteps[operationType] || [
            {
                id: 'generic_operation',
                label: 'Processing',
                description: 'Executing operation',
                status: 'pending',
                estimatedTime: 10,
            },
        ];
    }, []);

    // Start operation with progress tracking
    const startOperation = useCallback((operationType: string) => {
        const steps = createProgressSteps(operationType);
        setCurrentOperation(operationType);
        setOperationProgress(steps);

        // Start first step
        if (steps.length > 0) {
            progressTracker.updateStepStatus(steps[0].id, 'active');
        }
    }, [createProgressSteps, progressTracker]);

    // Update operation progress
    const updateOperationProgress = useCallback((stepId: string, status: ProgressStep['status'], error?: string) => {
        progressTracker.updateStepStatus(stepId, status, error);

        if (status === 'completed') {
            progressTracker.nextStep();
        } else if (status === 'failed') {
            progressTracker.failStep(error || 'Step failed');
        }
    }, [progressTracker]);

    // Complete operation
    const completeOperation = useCallback(() => {
        setCurrentOperation(null);
        setOperationProgress([]);
        progressTracker.resetProgress();
    }, [progressTracker]);

    // Handle recovery success
    const handleRecoverySuccess = useCallback(() => {
        setShowRecoveryInterface(false);
        errorHandling.clearError();
        showToast('success', 'Issue resolved successfully');
        onErrorResolved?.();
    }, [errorHandling, showToast, onErrorResolved]);

    // Handle recovery failure
    const handleRecoveryFailed = useCallback((error: Error) => {
        showToast('error', `Recovery failed: ${error.message}`);
    }, [showToast]);

    // Render error overlay
    const renderErrorOverlay = () => {
        if (!errorHandling.hasError) return null;

        return (
            <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4"
            >
                <div className="max-w-2xl w-full max-h-[90vh] overflow-y-auto">
                    <div className="space-y-4">
                        {/* Error Display */}
                        <ErrorDisplay
                            error={errorHandling.error!}
                            onRetry={() => errorHandling.retryOperation()}
                            onDismiss={() => errorHandling.clearError()}
                            showDetails={process.env.NODE_ENV === 'development'}
                        />

                        {/* Recovery Interface */}
                        {errorHandling.hasRecoveryActions && (
                            <RecoveryInterface
                                error={errorHandling.error!}
                                onRecoverySuccess={handleRecoverySuccess}
                                onRecoveryFailed={handleRecoveryFailed}
                                onDismiss={() => setShowRecoveryInterface(false)}
                                autoExecute={enableAutoRecovery}
                            />
                        )}
                    </div>
                </div>
            </motion.div>
        );
    };

    // Render progress overlay
    const renderProgressOverlay = () => {
        if (!currentOperation || !showProgressTracking) return null;

        return (
            <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="fixed inset-0 bg-black bg-opacity-30 z-40 flex items-center justify-center p-4"
            >
                <div className="max-w-lg w-full">
                    <ProgressTracker
                        steps={progressTracker.steps}
                        currentStepId={progressTracker.currentStep?.id}
                        onStepComplete={(stepId) => updateOperationProgress(stepId, 'completed')}
                        onStepFailed={(stepId, error) => updateOperationProgress(stepId, 'failed', error)}
                        onAllComplete={completeOperation}
                        showEstimatedTime={true}
                        showSubsteps={true}
                        allowRetry={true}
                    />
                </div>
            </motion.div>
        );
    };

    // Provide context to children
    const contextValue = {
        // Error handling
        handleError: errorHandling.handleError,
        clearError: errorHandling.clearError,
        hasError: errorHandling.hasError,
        error: errorHandling.error,
        isRecovering: errorHandling.isRecovering,

        // Progress tracking
        startOperation,
        updateOperationProgress,
        completeOperation,
        currentOperation,

        // Specific error handlers
        handleNetworkError: errorHandling.handleNetworkError,
        handleTransactionError: errorHandling.handleTransactionError,
        handleValidationError: errorHandling.handleValidationError,
        handleAuthError: errorHandling.handleAuthError,
    };

    return (
        <ErrorHandlerContext.Provider value={contextValue}>
            {children}

            <AnimatePresence>
                {renderProgressOverlay()}
                {renderErrorOverlay()}
            </AnimatePresence>
        </ErrorHandlerContext.Provider>
    );
};

// Context for error handling
const ErrorHandlerContext = React.createContext<any>(null);

// Hook to use error handler context
export const useErrorHandlerContext = () => {
    const context = React.useContext(ErrorHandlerContext);
    if (!context) {
        throw new Error('useErrorHandlerContext must be used within ComprehensiveErrorHandler');
    }
    return context;
};

// Higher-order component for automatic error handling
export function withErrorHandling<P extends object>(
    Component: React.ComponentType<P>,
    options: Partial<ComprehensiveErrorHandlerProps> = {}
) {
    const WrappedComponent = (props: P) => (
        <ComprehensiveErrorHandler {...options}>
            <Component {...props} />
        </ComprehensiveErrorHandler>
    );

    WrappedComponent.displayName = `withErrorHandling(${Component.displayName || Component.name})`;

    return WrappedComponent;
}

// Specialized error handlers for different contexts
export const CriticalErrorHandler: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <ComprehensiveErrorHandler
        enableAutoRecovery={false}
        showProgressTracking={false}
        enableNotifications={true}
        maxRetryAttempts={1}
    >
        {children}
    </ComprehensiveErrorHandler>
);

export const TransactionErrorHandler: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <ComprehensiveErrorHandler
        enableAutoRecovery={true}
        showProgressTracking={true}
        enableNotifications={true}
        maxRetryAttempts={3}
    >
        {children}
    </ComprehensiveErrorHandler>
);

export const NetworkErrorHandler: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <ComprehensiveErrorHandler
        enableAutoRecovery={true}
        showProgressTracking={false}
        enableNotifications={true}
        maxRetryAttempts={5}
    >
        {children}
    </ComprehensiveErrorHandler>
);

export default ComprehensiveErrorHandler;