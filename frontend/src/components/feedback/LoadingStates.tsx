/**
 * Loading States and Progress Indicators
 * Professional loading feedback for all user operations
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';

// Loading state types
export interface LoadingState {
    isLoading: boolean;
    message?: string;
    progress?: number;
    stage?: string;
    estimatedTime?: number;
}

// Transaction stages for multi-step operations
export enum TransactionStage {
    PREPARING = 'preparing',
    SIGNING = 'signing',
    BROADCASTING = 'broadcasting',
    CONFIRMING = 'confirming',
    FINALIZING = 'finalizing',
    COMPLETED = 'completed',
}

// Loading component props
interface LoadingSpinnerProps {
    size?: 'sm' | 'md' | 'lg';
    color?: 'primary' | 'secondary' | 'white';
    className?: string;
}

interface ProgressBarProps {
    progress: number;
    showPercentage?: boolean;
    color?: 'primary' | 'secondary' | 'success' | 'warning';
    className?: string;
}

interface TransactionProgressProps {
    stage: TransactionStage;
    message?: string;
    estimatedTime?: number;
    onCancel?: () => void;
}

interface SkeletonLoaderProps {
    lines?: number;
    height?: string;
    className?: string;
}

// Loading Spinner Component
export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
    size = 'md',
    color = 'primary',
    className = '',
}) => {
    const sizeClasses = {
        sm: 'w-4 h-4',
        md: 'w-6 h-6',
        lg: 'w-8 h-8',
    };

    const colorClasses = {
        primary: 'text-blue-600',
        secondary: 'text-gray-600',
        white: 'text-white',
    };

    return (
        <motion.div
            className={`inline-block ${sizeClasses[size]} ${colorClasses[color]} ${className}`}
            animate={{ rotate: 360 }}
            transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
        >
            <svg
                className="w-full h-full"
                fill="none"
                viewBox="0 0 24 24"
            >
                <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                />
                <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
            </svg>
        </motion.div>
    );
};

// Progress Bar Component
export const ProgressBar: React.FC<ProgressBarProps> = ({
    progress,
    showPercentage = true,
    color = 'primary',
    className = '',
}) => {
    const colorClasses = {
        primary: 'bg-blue-600',
        secondary: 'bg-gray-600',
        success: 'bg-green-600',
        warning: 'bg-yellow-600',
    };

    const clampedProgress = Math.max(0, Math.min(100, progress));

    return (
        <div className={`w-full ${className}`}>
            <div className="flex justify-between items-center mb-2">
                <span className="text-sm font-medium text-gray-700">Progress</span>
                {showPercentage && (
                    <span className="text-sm text-gray-500">{Math.round(clampedProgress)}%</span>
                )}
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
                <motion.div
                    className={`h-2 rounded-full ${colorClasses[color]}`}
                    initial={{ width: 0 }}
                    animate={{ width: `${clampedProgress}%` }}
                    transition={{ duration: 0.5, ease: 'easeOut' }}
                />
            </div>
        </div>
    );
};

// Transaction Progress Component
export const TransactionProgress: React.FC<TransactionProgressProps> = ({
    stage,
    message,
    estimatedTime,
    onCancel,
}) => {
    const stageInfo = {
        [TransactionStage.PREPARING]: {
            label: 'Preparing Transaction',
            progress: 20,
            icon: '⚙️',
        },
        [TransactionStage.SIGNING]: {
            label: 'Signing Transaction',
            progress: 40,
            icon: '✍️',
        },
        [TransactionStage.BROADCASTING]: {
            label: 'Broadcasting to Network',
            progress: 60,
            icon: '📡',
        },
        [TransactionStage.CONFIRMING]: {
            label: 'Confirming Transaction',
            progress: 80,
            icon: '⏳',
        },
        [TransactionStage.FINALIZING]: {
            label: 'Finalizing',
            progress: 95,
            icon: '🔄',
        },
        [TransactionStage.COMPLETED]: {
            label: 'Completed',
            progress: 100,
            icon: '✅',
        },
    };

    const currentStage = stageInfo[stage];

    return (
        <motion.div
            className="bg-white rounded-lg shadow-lg p-6 max-w-md mx-auto"
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.9 }}
        >
            <div className="text-center mb-4">
                <div className="text-4xl mb-2">{currentStage.icon}</div>
                <h3 className="text-lg font-semibold text-gray-900">
                    {currentStage.label}
                </h3>
                {message && (
                    <p className="text-sm text-gray-600 mt-1">{message}</p>
                )}
            </div>

            <ProgressBar
                progress={currentStage.progress}
                color={stage === TransactionStage.COMPLETED ? 'success' : 'primary'}
                className="mb-4"
            />

            {estimatedTime && stage !== TransactionStage.COMPLETED && (
                <div className="text-center text-sm text-gray-500 mb-4">
                    Estimated time: {estimatedTime}s
                </div>
            )}

            <div className="flex justify-center space-x-2">
                <LoadingSpinner size="sm" />
                <span className="text-sm text-gray-600">
                    {stage === TransactionStage.COMPLETED ? 'Transaction completed!' : 'Processing...'}
                </span>
            </div>

            {onCancel && stage !== TransactionStage.COMPLETED && (
                <button
                    onClick={onCancel}
                    className="w-full mt-4 px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors"
                >
                    Cancel Transaction
                </button>
            )}
        </motion.div>
    );
};

// Skeleton Loader Component
export const SkeletonLoader: React.FC<SkeletonLoaderProps> = ({
    lines = 3,
    height = '1rem',
    className = '',
}) => {
    return (
        <div className={`animate-pulse ${className}`}>
            {Array.from({ length: lines }).map((_, index) => (
                <div
                    key={index}
                    className="bg-gray-200 rounded mb-2 last:mb-0"
                    style={{ height }}
                />
            ))}
        </div>
    );
};

// Button Loading State Component
interface ButtonLoadingProps {
    isLoading: boolean;
    children: React.ReactNode;
    loadingText?: string;
    disabled?: boolean;
    className?: string;
    onClick?: () => void;
}

export const ButtonLoading: React.FC<ButtonLoadingProps> = ({
    isLoading,
    children,
    loadingText = 'Loading...',
    disabled = false,
    className = '',
    onClick,
}) => {
    return (
        <button
            onClick={onClick}
            disabled={disabled || isLoading}
            className={`
        relative inline-flex items-center justify-center px-4 py-2 
        font-medium rounded-md transition-all duration-200
        ${isLoading || disabled
                    ? 'opacity-50 cursor-not-allowed'
                    : 'hover:opacity-90 active:scale-95'
                }
        ${className}
      `}
        >
            <AnimatePresence mode="wait">
                {isLoading ? (
                    <motion.div
                        key="loading"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        className="flex items-center space-x-2"
                    >
                        <LoadingSpinner size="sm" color="white" />
                        <span>{loadingText}</span>
                    </motion.div>
                ) : (
                    <motion.div
                        key="content"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                    >
                        {children}
                    </motion.div>
                )}
            </AnimatePresence>
        </button>
    );
};

// Full Page Loading Component
interface FullPageLoadingProps {
    message?: string;
    showProgress?: boolean;
    progress?: number;
}

export const FullPageLoading: React.FC<FullPageLoadingProps> = ({
    message = 'Loading...',
    showProgress = false,
    progress = 0,
}) => {
    return (
        <motion.div
            className="fixed inset-0 bg-white bg-opacity-90 backdrop-blur-sm z-50 flex items-center justify-center"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
        >
            <div className="text-center">
                <LoadingSpinner size="lg" className="mb-4" />
                <h2 className="text-xl font-semibold text-gray-900 mb-2">
                    {message}
                </h2>
                {showProgress && (
                    <div className="w-64">
                        <ProgressBar progress={progress} />
                    </div>
                )}
            </div>
        </motion.div>
    );
};

// Card Loading State Component
interface CardLoadingProps {
    title?: string;
    lines?: number;
    className?: string;
}

export const CardLoading: React.FC<CardLoadingProps> = ({
    title,
    lines = 3,
    className = '',
}) => {
    return (
        <div className={`bg-white rounded-lg shadow p-6 ${className}`}>
            {title && (
                <div className="mb-4">
                    <div className="h-6 bg-gray-200 rounded w-1/3 animate-pulse" />
                </div>
            )}
            <SkeletonLoader lines={lines} />
        </div>
    );
};

// Table Loading State Component
interface TableLoadingProps {
    rows?: number;
    columns?: number;
    className?: string;
}

export const TableLoading: React.FC<TableLoadingProps> = ({
    rows = 5,
    columns = 4,
    className = '',
}) => {
    return (
        <div className={`animate-pulse ${className}`}>
            {/* Table Header */}
            <div className="grid grid-cols-4 gap-4 mb-4">
                {Array.from({ length: columns }).map((_, index) => (
                    <div key={index} className="h-4 bg-gray-300 rounded" />
                ))}
            </div>

            {/* Table Rows */}
            {Array.from({ length: rows }).map((_, rowIndex) => (
                <div key={rowIndex} className="grid grid-cols-4 gap-4 mb-2">
                    {Array.from({ length: columns }).map((_, colIndex) => (
                        <div key={colIndex} className="h-4 bg-gray-200 rounded" />
                    ))}
                </div>
            ))}
        </div>
    );
};

// Hook for managing loading states
export const useLoadingState = (initialState: boolean = false) => {
    const [isLoading, setIsLoading] = React.useState(initialState);
    const [message, setMessage] = React.useState<string>('');
    const [progress, setProgress] = React.useState<number>(0);

    const startLoading = (loadingMessage?: string) => {
        setIsLoading(true);
        setMessage(loadingMessage || '');
        setProgress(0);
    };

    const updateProgress = (newProgress: number, newMessage?: string) => {
        setProgress(newProgress);
        if (newMessage) setMessage(newMessage);
    };

    const stopLoading = () => {
        setIsLoading(false);
        setMessage('');
        setProgress(0);
    };

    return {
        isLoading,
        message,
        progress,
        startLoading,
        updateProgress,
        stopLoading,
    };
};

export default {
    LoadingSpinner,
    ProgressBar,
    TransactionProgress,
    SkeletonLoader,
    ButtonLoading,
    FullPageLoading,
    CardLoading,
    TableLoading,
    useLoadingState,
};