/**
 * Error Handling and User Feedback Components
 * Complete export of all error handling and feedback components
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

// Core Error Handling
export { errorHandler, ErrorCategory, ErrorSeverity } from '../../lib/errors/error-handler';
export type { VaultError } from '../../lib/errors/error-handler';

// Recovery System
export { 
  recoveryActionManager,
  getRecoveryActions,
  executeRecoveryAction,
  executeAutomatedRecovery,
  getRecoverySuggestions 
} from '../../lib/errors/recovery-actions';
export type { RecoveryAction, RecoveryPlan } from '../../lib/errors/recovery-actions';

// Hooks
export { default as useErrorHandling, useAsyncOperation } from '../../hooks/useErrorHandling';

// Error Boundary
export { 
  ErrorBoundary,
  withErrorBoundary,
  useErrorHandler,
  CriticalErrorBoundary,
  PageErrorBoundary,
  ComponentErrorBoundary 
} from './ErrorBoundary';

// Loading States
export {
  LoadingSpinner,
  ProgressBar,
  TransactionProgress,
  SkeletonLoader,
  ButtonLoading,
  FullPageLoading,
  CardLoading,
  TableLoading,
  useLoadingState,
  TransactionStage
} from './LoadingStates';
export type { LoadingState } from './LoadingStates';

// Notification System
export {
  NotificationProvider,
  useNotifications,
  NotificationPanel,
  NotificationBell,
  useCommonNotifications,
  NotificationType
} from './NotificationSystem';
export type { Notification, NotificationAction } from './NotificationSystem';

// User Feedback Components
export {
  FeedbackMessage,
  ErrorDisplay,
  SuccessFeedback,
  InlineValidation,
  ValidationSummary
} from './UserFeedback';

// Recovery Interface
export { default as RecoveryInterface } from './RecoveryInterface';

// Progress Tracker
export { 
  default as ProgressTracker,
  useProgressTracker 
} from './ProgressTracker';
export type { ProgressStep } from './ProgressTracker';

// Comprehensive Error Handler
export {
  default as ComprehensiveErrorHandler,
  useErrorHandlerContext,
  withErrorHandling,
  CriticalErrorHandler,
  TransactionErrorHandler,
  NetworkErrorHandler
} from './ComprehensiveErrorHandler';

// Utility functions for common error scenarios
export const handleNetworkError = (error: any, context?: string) => {
  return errorHandler.handleError(error, context);
};

export const handleValidationError = (field: string, value: any, message?: string) => {
  const error = { 
    code: 'VALIDATION_ERROR', 
    message: message || `Invalid value for ${field}`,
    field, 
    value 
  };
  return errorHandler.handleError(error, `Validation: ${field}`);
};

export const handleTransactionError = (error: any, transactionType: string) => {
  return errorHandler.handleError(error, `Transaction: ${transactionType}`);
};

export const handleAuthError = (error: any, authType: string) => {
  return errorHandler.handleError(error, `Authentication: ${authType}`);
};

// Common error patterns
export const ERROR_PATTERNS = {
  NETWORK_TIMEOUT: /timeout|network|connection/i,
  INSUFFICIENT_FUNDS: /insufficient|balance|funds/i,
  INVALID_ADDRESS: /invalid.*address|address.*invalid/i,
  AUTHENTICATION_FAILED: /auth|permission|unauthorized/i,
  VALIDATION_ERROR: /validation|invalid|format/i,
  TRANSACTION_FAILED: /transaction.*failed|failed.*transaction/i,
} as const;

// Error severity helpers
export const isHighSeverityError = (error: VaultError): boolean => {
  return error.severity === ErrorSeverity.HIGH || error.severity === ErrorSeverity.CRITICAL;
};

export const isCriticalError = (error: VaultError): boolean => {
  return error.severity === ErrorSeverity.CRITICAL;
};

export const isRecoverableError = (error: VaultError): boolean => {
  return error.recoveryActions && error.recoveryActions.length > 0;
};

// Default export for convenience
export default {
  // Core components
  ComprehensiveErrorHandler,
  ErrorBoundary,
  NotificationProvider,
  
  // Hooks
  useErrorHandling,
  useNotifications,
  useErrorHandlerContext,
  
  // Utilities
  errorHandler,
  recoveryActionManager,
  handleNetworkError,
  handleValidationError,
  handleTransactionError,
  handleAuthError,
};