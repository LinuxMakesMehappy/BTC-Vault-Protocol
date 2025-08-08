/**
 * Error Handling Hook
 * Comprehensive error handling with recovery actions and user feedback
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import { useState, useCallback, useEffect } from 'react';
import { errorHandler, VaultError, ErrorCategory, ErrorSeverity } from '../lib/errors/error-handler';
import { executeAutomatedRecovery, getRecoveryActions } from '../lib/errors/recovery-actions';
import { useNotifications } from '../components/feedback/NotificationSystem';

export interface ErrorHandlingOptions {
  autoRecover?: boolean;
  showNotifications?: boolean;
  logErrors?: boolean;
  retryAttempts?: number;
  retryDelay?: number;
}

export interface ErrorState {
  error: VaultError | null;
  isRecovering: boolean;
  recoveryAttempts: number;
  lastRecoveryTime: number | null;
  hasRecoveryActions: boolean;
}

export const useErrorHandling = (options: ErrorHandlingOptions = {}) => {
  const {
    autoRecover = true,
    showNotifications = true,
    logErrors = true,
    retryAttempts = 3,
    retryDelay = 1000,
  } = options;

  const [errorState, setErrorState] = useState<ErrorState>({
    error: null,
    isRecovering: false,
    recoveryAttempts: 0,
    lastRecoveryTime: null,
    hasRecoveryActions: false,
  });

  const { showToast, addNotification } = useNotifications();

  // Handle error with comprehensive processing
  const handleError = useCallback(async (
    error: any,
    context?: string,
    customOptions?: Partial<ErrorHandlingOptions>
  ): Promise<VaultError> => {
    const processedError = errorHandler.handleError(error, context);
    const recoveryActions = getRecoveryActions(processedError);

    setErrorState(prev => ({
      ...prev,
      error: processedError,
      hasRecoveryActions: recoveryActions.length > 0,
    }));

    // Show notifications if enabled
    if (showNotifications || customOptions?.showNotifications) {
      showErrorNotification(processedError);
    }

    // Attempt automated recovery if enabled
    if ((autoRecover || customOptions?.autoRecover) && recoveryActions.length > 0) {
      await attemptRecovery(processedError);
    }

    return processedError;
  }, [autoRecover, showNotifications, showToast, addNotification]);

  // Show error notification based on severity
  const showErrorNotification = useCallback((error: VaultError) => {
    const notificationOptions = {
      persistent: error.severity === ErrorSeverity.CRITICAL,
      actions: error.recoveryActions ? error.recoveryActions.slice(0, 2).map(action => ({
        label: action,
        action: () => {
          // Trigger recovery action
          const event = new CustomEvent('recovery-action', { detail: { action, error } });
          window.dispatchEvent(event);
        },
      })) : undefined,
    };

    switch (error.severity) {
      case ErrorSeverity.CRITICAL:
        addNotification({
          type: 'error',
          title: 'Critical Error',
          message: error.userMessage,
          persistent: true,
          actions: notificationOptions.actions,
        });
        break;

      case ErrorSeverity.HIGH:
        addNotification({
          type: 'error',
          title: 'Error',
          message: error.userMessage,
          duration: 8000,
          actions: notificationOptions.actions,
        });
        break;

      case ErrorSeverity.MEDIUM:
        showToast('warning', error.userMessage);
        break;

      case ErrorSeverity.LOW:
        showToast('info', error.userMessage);
        break;
    }
  }, [addNotification, showToast]);

  // Attempt automated recovery
  const attemptRecovery = useCallback(async (error: VaultError) => {
    if (errorState.recoveryAttempts >= retryAttempts) {
      console.warn('Maximum recovery attempts reached for error:', error.code);
      return false;
    }

    setErrorState(prev => ({
      ...prev,
      isRecovering: true,
      recoveryAttempts: prev.recoveryAttempts + 1,
    }));

    try {
      // Add delay between recovery attempts
      if (errorState.recoveryAttempts > 0) {
        await new Promise(resolve => setTimeout(resolve, retryDelay * errorState.recoveryAttempts));
      }

      const success = await executeAutomatedRecovery(error);

      setErrorState(prev => ({
        ...prev,
        isRecovering: false,
        lastRecoveryTime: Date.now(),
      }));

      if (success) {
        showToast('success', 'Issue resolved automatically');
        clearError();
        return true;
      } else {
        showToast('warning', 'Automated recovery failed. Manual intervention may be required.');
        return false;
      }
    } catch (recoveryError) {
      console.error('Recovery attempt failed:', recoveryError);
      
      setErrorState(prev => ({
        ...prev,
        isRecovering: false,
        lastRecoveryTime: Date.now(),
      }));

      showToast('error', 'Recovery attempt failed');
      return false;
    }
  }, [errorState.recoveryAttempts, retryAttempts, retryDelay, showToast]);

  // Clear current error
  const clearError = useCallback(() => {
    setErrorState({
      error: null,
      isRecovering: false,
      recoveryAttempts: 0,
      lastRecoveryTime: null,
      hasRecoveryActions: false,
    });
  }, []);

  // Retry the last failed operation
  const retryOperation = useCallback(async (operation?: () => Promise<any>) => {
    if (!errorState.error) return;

    try {
      setErrorState(prev => ({ ...prev, isRecovering: true }));

      if (operation) {
        await operation();
      } else {
        // Attempt recovery again
        await attemptRecovery(errorState.error);
      }

      clearError();
      showToast('success', 'Operation completed successfully');
    } catch (error) {
      await handleError(error, 'Retry operation');
    } finally {
      setErrorState(prev => ({ ...prev, isRecovering: false }));
    }
  }, [errorState.error, attemptRecovery, clearError, handleError, showToast]);

  // Get error statistics
  const getErrorStats = useCallback(() => {
    return errorHandler.getErrorStats();
  }, []);

  // Check if error is recoverable
  const isRecoverable = useCallback((error?: VaultError) => {
    const targetError = error || errorState.error;
    if (!targetError) return false;

    const recoveryActions = getRecoveryActions(targetError);
    return recoveryActions.length > 0;
  }, [errorState.error]);

  // Get user-friendly error message
  const getErrorMessage = useCallback((error?: VaultError) => {
    const targetError = error || errorState.error;
    return targetError?.userMessage || 'An unexpected error occurred';
  }, [errorState.error]);

  // Get recovery suggestions
  const getRecoverySuggestions = useCallback((error?: VaultError) => {
    const targetError = error || errorState.error;
    return targetError?.recoveryActions || [];
  }, [errorState.error]);

  // Handle specific error categories
  const handleNetworkError = useCallback(async (error: any, context?: string) => {
    return handleError(error, context || 'Network operation');
  }, [handleError]);

  const handleTransactionError = useCallback(async (error: any, transactionType?: string) => {
    return handleError(error, `Transaction: ${transactionType || 'Unknown'}`);
  }, [handleError]);

  const handleValidationError = useCallback(async (field: string, value: any, message?: string) => {
    const error = {
      code: 'VALIDATION_ERROR',
      message: message || `Invalid value for ${field}`,
      field,
      value,
    };
    return handleError(error, `Validation: ${field}`);
  }, [handleError]);

  const handleAuthError = useCallback(async (error: any, authType?: string) => {
    return handleError(error, `Authentication: ${authType || 'Unknown'}`);
  }, [handleError]);

  // Effect to handle recovery action events
  useEffect(() => {
    const handleRecoveryAction = (event: CustomEvent) => {
      const { action, error } = event.detail;
      console.log('Recovery action triggered:', action, error);
      // Handle specific recovery actions here
    };

    window.addEventListener('recovery-action', handleRecoveryAction as EventListener);
    return () => {
      window.removeEventListener('recovery-action', handleRecoveryAction as EventListener);
    };
  }, []);

  return {
    // State
    error: errorState.error,
    isRecovering: errorState.isRecovering,
    recoveryAttempts: errorState.recoveryAttempts,
    hasRecoveryActions: errorState.hasRecoveryActions,
    lastRecoveryTime: errorState.lastRecoveryTime,

    // Actions
    handleError,
    clearError,
    retryOperation,
    attemptRecovery: () => errorState.error ? attemptRecovery(errorState.error) : Promise.resolve(false),

    // Specific error handlers
    handleNetworkError,
    handleTransactionError,
    handleValidationError,
    handleAuthError,

    // Utilities
    isRecoverable,
    getErrorMessage,
    getRecoverySuggestions,
    getErrorStats,

    // Convenience methods
    hasError: !!errorState.error,
    canRetry: errorState.recoveryAttempts < retryAttempts,
    errorSeverity: errorState.error?.severity,
    errorCategory: errorState.error?.category,
  };
};

// Hook for handling async operations with error handling
export const useAsyncOperation = <T = any>(
  operation: () => Promise<T>,
  options: ErrorHandlingOptions & {
    onSuccess?: (result: T) => void;
    onError?: (error: VaultError) => void;
    dependencies?: any[];
  } = {}
) => {
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<T | null>(null);
  const errorHandling = useErrorHandling(options);

  const execute = useCallback(async (...args: any[]) => {
    setIsLoading(true);
    setResult(null);
    errorHandling.clearError();

    try {
      const operationResult = await operation();
      setResult(operationResult);
      options.onSuccess?.(operationResult);
      return operationResult;
    } catch (error) {
      const processedError = await errorHandling.handleError(error, 'Async operation');
      options.onError?.(processedError);
      throw processedError;
    } finally {
      setIsLoading(false);
    }
  }, [operation, errorHandling, options, ...(options.dependencies || [])]);

  const retry = useCallback(() => {
    return execute();
  }, [execute]);

  return {
    execute,
    retry,
    isLoading,
    result,
    ...errorHandling,
  };
};

export default useErrorHandling;