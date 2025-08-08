/**
 * Recovery Interface Component
 * Interactive recovery suggestions and automated recovery actions
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { VaultError } from '../../lib/errors/error-handler';
import {
    RecoveryAction,
    RecoveryPlan,
    recoveryActionManager,
    executeRecoveryAction,
    executeAutomatedRecovery
} from '../../lib/errors/recovery-actions';
import { useNotifications } from './NotificationSystem';

interface RecoveryInterfaceProps {
    error: VaultError;
    onRecoverySuccess?: () => void;
    onRecoveryFailed?: (error: Error) => void;
    onDismiss?: () => void;
    autoExecute?: boolean;
}

export const RecoveryInterface: React.FC<RecoveryInterfaceProps> = ({
    error,
    onRecoverySuccess,
    onRecoveryFailed,
    onDismiss,
    autoExecute = true,
}) => {
    const [recoveryPlan, setRecoveryPlan] = useState<RecoveryPlan | null>(null);
    const [executingAction, setExecutingAction] = useState<string | null>(null);
    const [completedActions, setCompletedActions] = useState<Set<string>>(new Set());
    const [failedActions, setFailedActions] = useState<Set<string>>(new Set());
    const [autoRecoveryAttempted, setAutoRecoveryAttempted] = useState(false);
    const { showToast } = useNotifications();

    useEffect(() => {
        const plan = recoveryActionManager.getRecoveryPlan(error.code);
        setRecoveryPlan(plan);

        // Attempt automated recovery if enabled
        if (autoExecute && !autoRecoveryAttempted) {
            setAutoRecoveryAttempted(true);
            attemptAutomatedRecovery();
        }
    }, [error.code, autoExecute, autoRecoveryAttempted]);

    const attemptAutomatedRecovery = async () => {
        try {
            const success = await executeAutomatedRecovery(error);
            if (success) {
                showToast('success', 'Issue resolved automatically');
                onRecoverySuccess?.();
            }
        } catch (recoveryError) {
            console.warn('Automated recovery failed:', recoveryError);
        }
    };

    const handleExecuteAction = async (action: RecoveryAction) => {
        setExecutingAction(action.id);

        try {
            const success = await executeRecoveryAction(action.id, error.code);

            if (success) {
                setCompletedActions(prev => new Set([...prev, action.id]));
                showToast('success', `${action.label} completed successfully`);

                // Check if this was a critical action that might resolve the issue
                if (action.priority === 'high') {
                    setTimeout(() => {
                        onRecoverySuccess?.();
                    }, 1000);
                }
            } else {
                setFailedActions(prev => new Set([...prev, action.id]));
                showToast('error', `${action.label} failed to complete`);
            }
        } catch (actionError) {
            setFailedActions(prev => new Set([...prev, action.id]));
            showToast('error', `Error executing ${action.label}: ${actionError}`);
            onRecoveryFailed?.(actionError as Error);
        } finally {
            setExecutingAction(null);
        }
    };

    const getPriorityColor = (priority: RecoveryAction['priority']) => {
        switch (priority) {
            case 'high': return 'border-red-200 bg-red-50';
            case 'medium': return 'border-yellow-200 bg-yellow-50';
            case 'low': return 'border-blue-200 bg-blue-50';
            default: return 'border-gray-200 bg-gray-50';
        }
    };

    const getPriorityIcon = (priority: RecoveryAction['priority']) => {
        switch (priority) {
            case 'high': return '🔴';
            case 'medium': return '🟡';
            case 'low': return '🔵';
            default: return '⚪';
        }
    };

    const getActionStatus = (actionId: string) => {
        if (executingAction === actionId) return 'executing';
        if (completedActions.has(actionId)) return 'completed';
        if (failedActions.has(actionId)) return 'failed';
        return 'pending';
    };

    const getStatusIcon = (status: string) => {
        switch (status) {
            case 'executing': return '⏳';
            case 'completed': return '✅';
            case 'failed': return '❌';
            default: return '⚪';
        }
    };

    if (!recoveryPlan) {
        return (
            <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="bg-gray-50 border border-gray-200 rounded-lg p-4"
            >
                <div className="text-center">
                    <div className="text-gray-400 text-2xl mb-2">🔧</div>
                    <p className="text-gray-600 text-sm">
                        No specific recovery actions available for this error.
                    </p>
                    <p className="text-gray-500 text-xs mt-1">
                        Try refreshing the page or contact support if the issue persists.
                    </p>
                </div>
            </motion.div>
        );
    }

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-white border border-gray-200 rounded-lg shadow-sm"
        >
            {/* Header */}
            <div className="p-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                    <div>
                        <h3 className="font-semibold text-gray-900 flex items-center space-x-2">
                            <span>🛠️</span>
                            <span>Recovery Options</span>
                        </h3>
                        <p className="text-sm text-gray-600 mt-1">
                            Try these actions to resolve the issue
                        </p>
                    </div>
                    {onDismiss && (
                        <button
                            onClick={onDismiss}
                            className="text-gray-400 hover:text-gray-600 transition-colors"
                        >
                            ✕
                        </button>
                    )}
                </div>

                {recoveryPlan.estimatedTime && (
                    <div className="mt-3 flex items-center space-x-4 text-xs text-gray-500">
                        <span>⏱️ Estimated time: {recoveryPlan.estimatedTime}</span>
                        {recoveryPlan.successRate && (
                            <span>📊 Success rate: {recoveryPlan.successRate}%</span>
                        )}
                    </div>
                )}
            </div>

            {/* Recovery Actions */}
            <div className="p-4">
                <div className="space-y-3">
                    {recoveryPlan.actions.map((action, index) => {
                        const status = getActionStatus(action.id);
                        const isDisabled = executingAction !== null && executingAction !== action.id;

                        return (
                            <motion.div
                                key={action.id}
                                initial={{ opacity: 0, x: -20 }}
                                animate={{ opacity: 1, x: 0 }}
                                transition={{ delay: index * 0.1 }}
                                className={`
                  border rounded-lg p-3 transition-all duration-200
                  ${getPriorityColor(action.priority)}
                  ${isDisabled ? 'opacity-50' : 'hover:shadow-sm'}
                `}
                            >
                                <div className="flex items-start justify-between">
                                    <div className="flex items-start space-x-3 flex-1">
                                        <div className="flex flex-col items-center space-y-1">
                                            <span className="text-sm">{getPriorityIcon(action.priority)}</span>
                                            <span className="text-lg">{getStatusIcon(status)}</span>
                                        </div>

                                        <div className="flex-1">
                                            <div className="flex items-center space-x-2">
                                                <h4 className="font-medium text-sm text-gray-900">
                                                    {action.label}
                                                </h4>
                                                {action.automated && (
                                                    <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded">
                                                        Auto
                                                    </span>
                                                )}
                                                {action.requiresConfirmation && (
                                                    <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded">
                                                        Confirm
                                                    </span>
                                                )}
                                            </div>

                                            <p className="text-xs text-gray-600 mt-1">
                                                {action.description}
                                            </p>

                                            {status === 'failed' && (
                                                <p className="text-xs text-red-600 mt-1">
                                                    Action failed. You may try again or contact support.
                                                </p>
                                            )}

                                            {status === 'completed' && (
                                                <p className="text-xs text-green-600 mt-1">
                                                    Action completed successfully.
                                                </p>
                                            )}
                                        </div>
                                    </div>

                                    <div className="ml-3">
                                        {status === 'pending' && (
                                            <button
                                                onClick={() => handleExecuteAction(action)}
                                                disabled={isDisabled}
                                                className={`
                          px-3 py-1 text-xs rounded transition-colors
                          ${action.priority === 'high'
                                                        ? 'bg-red-600 text-white hover:bg-red-700'
                                                        : action.priority === 'medium'
                                                            ? 'bg-yellow-600 text-white hover:bg-yellow-700'
                                                            : 'bg-blue-600 text-white hover:bg-blue-700'
                                                    }
                          ${isDisabled ? 'opacity-50 cursor-not-allowed' : ''}
                        `}
                                            >
                                                Execute
                                            </button>
                                        )}

                                        {status === 'executing' && (
                                            <div className="flex items-center space-x-2">
                                                <div className="animate-spin w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full"></div>
                                                <span className="text-xs text-gray-600">Running...</span>
                                            </div>
                                        )}

                                        {status === 'failed' && (
                                            <button
                                                onClick={() => handleExecuteAction(action)}
                                                disabled={isDisabled}
                                                className="px-3 py-1 text-xs bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                                            >
                                                Retry
                                            </button>
                                        )}

                                        {status === 'completed' && (
                                            <span className="text-xs text-green-600 font-medium">
                                                Done
                                            </span>
                                        )}
                                    </div>
                                </div>
                            </motion.div>
                        );
                    })}
                </div>

                {/* Progress Summary */}
                <div className="mt-4 pt-4 border-t border-gray-200">
                    <div className="flex items-center justify-between text-xs text-gray-500">
                        <span>
                            Progress: {completedActions.size} of {recoveryPlan.actions.length} actions completed
                        </span>
                        {failedActions.size > 0 && (
                            <span className="text-red-600">
                                {failedActions.size} failed
                            </span>
                        )}
                    </div>

                    {completedActions.size === recoveryPlan.actions.length && (
                        <motion.div
                            initial={{ opacity: 0, y: 10 }}
                            animate={{ opacity: 1, y: 0 }}
                            className="mt-2 p-2 bg-green-50 border border-green-200 rounded text-center"
                        >
                            <span className="text-sm text-green-800">
                                ✅ All recovery actions completed!
                            </span>
                        </motion.div>
                    )}
                </div>

                {/* Additional Help */}
                <div className="mt-4 pt-4 border-t border-gray-200">
                    <p className="text-xs text-gray-500 mb-2">
                        Still having issues? Here are some additional suggestions:
                    </p>
                    <ul className="text-xs text-gray-600 space-y-1">
                        {error.recoveryActions?.map((suggestion, index) => (
                            <li key={index} className="flex items-start space-x-1">
                                <span>•</span>
                                <span>{suggestion}</span>
                            </li>
                        ))}
                    </ul>

                    <div className="mt-3 flex space-x-2">
                        <button
                            onClick={() => window.location.reload()}
                            className="text-xs bg-gray-100 text-gray-700 px-3 py-1 rounded hover:bg-gray-200 transition-colors"
                        >
                            Refresh Page
                        </button>
                        <button
                            onClick={() => window.open('mailto:support@vaultprotocol.com?subject=Error%20Report&body=' + encodeURIComponent(`Error Code: ${error.code}\nMessage: ${error.message}\nTimestamp: ${new Date(error.timestamp).toISOString()}`))}
                            className="text-xs bg-blue-100 text-blue-700 px-3 py-1 rounded hover:bg-blue-200 transition-colors"
                        >
                            Contact Support
                        </button>
                    </div>
                </div>
            </div>
        </motion.div>
    );
};

export default RecoveryInterface;