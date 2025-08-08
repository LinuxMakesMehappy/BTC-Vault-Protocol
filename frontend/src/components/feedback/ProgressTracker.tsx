/**
 * Progress Tracker Component
 * Advanced progress tracking for multi-step operations
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

export interface ProgressStep {
    id: string;
    label: string;
    description?: string;
    status: 'pending' | 'active' | 'completed' | 'failed' | 'skipped';
    estimatedTime?: number;
    actualTime?: number;
    error?: string;
    substeps?: ProgressStep[];
}

export interface ProgressTrackerProps {
    steps: ProgressStep[];
    currentStepId?: string;
    onStepComplete?: (stepId: string) => void;
    onStepFailed?: (stepId: string, error: string) => void;
    onAllComplete?: () => void;
    showEstimatedTime?: boolean;
    showSubsteps?: boolean;
    allowRetry?: boolean;
    className?: string;
}

export const ProgressTracker: React.FC<ProgressTrackerProps> = ({
    steps,
    currentStepId,
    onStepComplete,
    onStepFailed,
    onAllComplete,
    showEstimatedTime = true,
    showSubsteps = false,
    allowRetry = true,
    className = '',
}) => {
    const [internalSteps, setInternalSteps] = useState<ProgressStep[]>(steps);
    const [startTime, setStartTime] = useState<number>(Date.now());
    const [elapsedTime, setElapsedTime] = useState<number>(0);

    useEffect(() => {
        setInternalSteps(steps);
    }, [steps]);

    useEffect(() => {
        const timer = setInterval(() => {
            setElapsedTime(Date.now() - startTime);
        }, 1000);

        return () => clearInterval(timer);
    }, [startTime]);

    useEffect(() => {
        const allCompleted = internalSteps.every(step =>
            step.status === 'completed' || step.status === 'skipped'
        );

        if (allCompleted && internalSteps.length > 0) {
            onAllComplete?.();
        }
    }, [internalSteps, onAllComplete]);

    const getStepIcon = (step: ProgressStep) => {
        switch (step.status) {
            case 'completed': return '✅';
            case 'active': return '⏳';
            case 'failed': return '❌';
            case 'skipped': return '⏭️';
            default: return '⚪';
        }
    };

    const getStepColor = (step: ProgressStep) => {
        switch (step.status) {
            case 'completed': return 'text-green-600 border-green-200 bg-green-50';
            case 'active': return 'text-blue-600 border-blue-200 bg-blue-50';
            case 'failed': return 'text-red-600 border-red-200 bg-red-50';
            case 'skipped': return 'text-gray-400 border-gray-200 bg-gray-50';
            default: return 'text-gray-500 border-gray-200 bg-gray-50';
        }
    };

    const formatTime = (milliseconds: number) => {
        const seconds = Math.floor(milliseconds / 1000);
        const minutes = Math.floor(seconds / 60);

        if (minutes > 0) {
            return `${minutes}m ${seconds % 60}s`;
        }
        return `${seconds}s`;
    };

    const getTotalEstimatedTime = () => {
        return internalSteps.reduce((total, step) => {
            return total + (step.estimatedTime || 0);
        }, 0);
    };

    const getCompletedSteps = () => {
        return internalSteps.filter(step => step.status === 'completed').length;
    };

    const getProgressPercentage = () => {
        const completed = getCompletedSteps();
        return Math.round((completed / internalSteps.length) * 100);
    };

    const handleRetryStep = (stepId: string) => {
        setInternalSteps(prev => prev.map(step =>
            step.id === stepId
                ? { ...step, status: 'pending', error: undefined }
                : step
        ));
    };

    const renderSubsteps = (substeps: ProgressStep[]) => {
        if (!showSubsteps || !substeps || substeps.length === 0) return null;

        return (
            <div className="ml-8 mt-2 space-y-1">
                {substeps.map((substep, index) => (
                    <div key={substep.id} className="flex items-center space-x-2 text-xs">
                        <span>{getStepIcon(substep)}</span>
                        <span className={substep.status === 'failed' ? 'text-red-600' : 'text-gray-600'}>
                            {substep.label}
                        </span>
                        {substep.status === 'active' && (
                            <div className="animate-spin w-3 h-3 border border-blue-600 border-t-transparent rounded-full"></div>
                        )}
                    </div>
                ))}
            </div>
        );
    };

    return (
        <div className={`bg-white rounded-lg border border-gray-200 shadow-sm ${className}`}>
            {/* Header */}
            <div className="p-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                    <h3 className="font-semibold text-gray-900">Progress</h3>
                    <span className="text-sm text-gray-500">
                        {getCompletedSteps()} of {internalSteps.length} completed
                    </span>
                </div>

                {/* Progress Bar */}
                <div className="mt-3">
                    <div className="flex justify-between items-center mb-1">
                        <span className="text-sm text-gray-600">Overall Progress</span>
                        <span className="text-sm font-medium text-gray-900">
                            {getProgressPercentage()}%
                        </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                        <motion.div
                            className="bg-blue-600 h-2 rounded-full"
                            initial={{ width: 0 }}
                            animate={{ width: `${getProgressPercentage()}%` }}
                            transition={{ duration: 0.5, ease: 'easeOut' }}
                        />
                    </div>
                </div>

                {/* Time Information */}
                {showEstimatedTime && (
                    <div className="mt-2 flex justify-between text-xs text-gray-500">
                        <span>Elapsed: {formatTime(elapsedTime)}</span>
                        <span>
                            Estimated total: {formatTime(getTotalEstimatedTime() * 1000)}
                        </span>
                    </div>
                )}
            </div>

            {/* Steps */}
            <div className="p-4">
                <div className="space-y-3">
                    {internalSteps.map((step, index) => (
                        <motion.div
                            key={step.id}
                            initial={{ opacity: 0, x: -20 }}
                            animate={{ opacity: 1, x: 0 }}
                            transition={{ delay: index * 0.1 }}
                            className={`
                border rounded-lg p-3 transition-all duration-200
                ${getStepColor(step)}
                ${step.status === 'active' ? 'ring-2 ring-blue-200' : ''}
              `}
                        >
                            <div className="flex items-start justify-between">
                                <div className="flex items-start space-x-3 flex-1">
                                    <div className="text-xl">{getStepIcon(step)}</div>

                                    <div className="flex-1">
                                        <div className="flex items-center space-x-2">
                                            <h4 className="font-medium text-sm">
                                                {step.label}
                                            </h4>
                                            {step.status === 'active' && (
                                                <div className="animate-spin w-4 h-4 border-2 border-current border-t-transparent rounded-full"></div>
                                            )}
                                        </div>

                                        {step.description && (
                                            <p className="text-xs mt-1 opacity-75">
                                                {step.description}
                                            </p>
                                        )}

                                        {step.error && (
                                            <p className="text-xs text-red-600 mt-1">
                                                Error: {step.error}
                                            </p>
                                        )}

                                        {showEstimatedTime && step.estimatedTime && (
                                            <p className="text-xs mt-1 opacity-75">
                                                Estimated time: {formatTime(step.estimatedTime * 1000)}
                                            </p>
                                        )}

                                        {step.actualTime && step.status === 'completed' && (
                                            <p className="text-xs mt-1 opacity-75">
                                                Completed in: {formatTime(step.actualTime * 1000)}
                                            </p>
                                        )}

                                        {renderSubsteps(step.substeps || [])}
                                    </div>
                                </div>

                                {/* Action Buttons */}
                                <div className="ml-3 flex space-x-2">
                                    {step.status === 'failed' && allowRetry && (
                                        <button
                                            onClick={() => handleRetryStep(step.id)}
                                            className="text-xs bg-red-100 text-red-700 px-2 py-1 rounded hover:bg-red-200 transition-colors"
                                        >
                                            Retry
                                        </button>
                                    )}

                                    {step.status === 'pending' && index > 0 && (
                                        <button
                                            onClick={() => {
                                                setInternalSteps(prev => prev.map(s =>
                                                    s.id === step.id
                                                        ? { ...s, status: 'skipped' }
                                                        : s
                                                ));
                                            }}
                                            className="text-xs bg-gray-100 text-gray-700 px-2 py-1 rounded hover:bg-gray-200 transition-colors"
                                        >
                                            Skip
                                        </button>
                                    )}
                                </div>
                            </div>
                        </motion.div>
                    ))}
                </div>

                {/* Summary */}
                <div className="mt-4 pt-4 border-t border-gray-200">
                    <div className="grid grid-cols-2 gap-4 text-sm">
                        <div>
                            <span className="text-gray-500">Status:</span>
                            <span className="ml-2 font-medium">
                                {getProgressPercentage() === 100 ? 'Completed' : 'In Progress'}
                            </span>
                        </div>
                        <div>
                            <span className="text-gray-500">Time:</span>
                            <span className="ml-2 font-medium">
                                {formatTime(elapsedTime)}
                            </span>
                        </div>
                    </div>

                    {getProgressPercentage() === 100 && (
                        <motion.div
                            initial={{ opacity: 0, y: 10 }}
                            animate={{ opacity: 1, y: 0 }}
                            className="mt-3 p-3 bg-green-50 border border-green-200 rounded-lg text-center"
                        >
                            <span className="text-green-800 font-medium">
                                🎉 All steps completed successfully!
                            </span>
                        </motion.div>
                    )}
                </div>
            </div>
        </div>
    );
};

// Hook for managing progress state
export const useProgressTracker = (initialSteps: ProgressStep[]) => {
    const [steps, setSteps] = useState<ProgressStep[]>(initialSteps);
    const [currentStepIndex, setCurrentStepIndex] = useState(0);

    const updateStepStatus = (stepId: string, status: ProgressStep['status'], error?: string) => {
        setSteps(prev => prev.map(step =>
            step.id === stepId
                ? {
                    ...step,
                    status,
                    error,
                    actualTime: status === 'completed' ? Date.now() : step.actualTime
                }
                : step
        ));
    };

    const nextStep = () => {
        if (currentStepIndex < steps.length - 1) {
            const currentStep = steps[currentStepIndex];
            const nextStep = steps[currentStepIndex + 1];

            updateStepStatus(currentStep.id, 'completed');
            updateStepStatus(nextStep.id, 'active');
            setCurrentStepIndex(currentStepIndex + 1);
        }
    };

    const failStep = (error: string) => {
        const currentStep = steps[currentStepIndex];
        updateStepStatus(currentStep.id, 'failed', error);
    };

    const skipStep = () => {
        const currentStep = steps[currentStepIndex];
        updateStepStatus(currentStep.id, 'skipped');
        nextStep();
    };

    const resetProgress = () => {
        setSteps(initialSteps.map(step => ({ ...step, status: 'pending' })));
        setCurrentStepIndex(0);
        if (initialSteps.length > 0) {
            updateStepStatus(initialSteps[0].id, 'active');
        }
    };

    const getCurrentStep = () => {
        return steps[currentStepIndex];
    };

    const isComplete = () => {
        return steps.every(step => step.status === 'completed' || step.status === 'skipped');
    };

    return {
        steps,
        currentStep: getCurrentStep(),
        currentStepIndex,
        updateStepStatus,
        nextStep,
        failStep,
        skipStep,
        resetProgress,
        isComplete,
    };
};

export default ProgressTracker;