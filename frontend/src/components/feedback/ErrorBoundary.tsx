/**
 * Error Boundary Component
 * Catches and handles React component errors gracefully
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { Component, ErrorInfo, ReactNode } from 'react';
import { errorHandler, ErrorCategory, ErrorSeverity } from '../../lib/errors/error-handler';

interface Props {
    children: ReactNode;
    fallback?: ReactNode;
    onError?: (error: Error, errorInfo: ErrorInfo) => void;
    level?: 'page' | 'component' | 'critical';
}

interface State {
    hasError: boolean;
    error: Error | null;
    errorInfo: ErrorInfo | null;
    errorId: string | null;
}

export class ErrorBoundary extends Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {
            hasError: false,
            error: null,
            errorInfo: null,
            errorId: null,
        };
    }

    static getDerivedStateFromError(error: Error): Partial<State> {
        // Update state so the next render will show the fallback UI
        return {
            hasError: true,
            error,
            errorId: `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        };
    }

    componentDidCatch(error: Error, errorInfo: ErrorInfo) {
        // Log the error
        console.error('ErrorBoundary caught an error:', error, errorInfo);

        // Update state with error info
        this.setState({
            errorInfo,
        });

        // Handle error through error handler
        const vaultError = errorHandler.handleError(error, 'React Error Boundary');

        // Call custom error handler if provided
        if (this.props.onError) {
            this.props.onError(error, errorInfo);
        }

        // Report to error monitoring service
        this.reportError(error, errorInfo);
    }

    private reportError(error: Error, errorInfo: ErrorInfo) {
        // In production, this would send to error reporting service
        const errorReport = {
            message: error.message,
            stack: error.stack,
            componentStack: errorInfo.componentStack,
            timestamp: new Date().toISOString(),
            userAgent: navigator.userAgent,
            url: window.location.href,
            level: this.props.level || 'component',
        };

        console.warn('Error reported:', errorReport);
        // Example: errorReportingService.report(errorReport);
    }

    private handleRetry = () => {
        this.setState({
            hasError: false,
            error: null,
            errorInfo: null,
            errorId: null,
        });
    };

    private handleReload = () => {
        window.location.reload();
    };

    render() {
        if (this.state.hasError) {
            // Custom fallback UI
            if (this.props.fallback) {
                return this.props.fallback;
            }

            // Default fallback UI based on level
            return this.renderDefaultFallback();
        }

        return this.props.children;
    }

    private renderDefaultFallback() {
        const { level = 'component' } = this.props;
        const { error, errorId } = this.state;

        switch (level) {
            case 'critical':
                return (
                    <div className="min-h-screen bg-red-50 flex items-center justify-center p-4">
                        <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6 text-center">
                            <div className="text-red-500 text-6xl mb-4">⚠️</div>
                            <h1 className="text-2xl font-bold text-gray-900 mb-2">
                                Critical System Error
                            </h1>
                            <p className="text-gray-600 mb-6">
                                A critical error has occurred. Please reload the application.
                            </p>
                            <div className="space-y-3">
                                <button
                                    onClick={this.handleReload}
                                    className="w-full bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700 transition-colors"
                                >
                                    Reload Application
                                </button>
                                <p className="text-xs text-gray-500">
                                    Error ID: {errorId}
                                </p>
                            </div>
                        </div>
                    </div>
                );

            case 'page':
                return (
                    <div className="min-h-96 bg-gray-50 flex items-center justify-center p-4">
                        <div className="max-w-lg w-full bg-white rounded-lg shadow p-6 text-center">
                            <div className="text-yellow-500 text-5xl mb-4">🚧</div>
                            <h2 className="text-xl font-semibold text-gray-900 mb-2">
                                Page Error
                            </h2>
                            <p className="text-gray-600 mb-4">
                                This page encountered an error and couldn't load properly.
                            </p>
                            <div className="space-y-2">
                                <button
                                    onClick={this.handleRetry}
                                    className="w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition-colors"
                                >
                                    Try Again
                                </button>
                                <button
                                    onClick={this.handleReload}
                                    className="w-full bg-gray-600 text-white px-4 py-2 rounded-md hover:bg-gray-700 transition-colors"
                                >
                                    Reload Page
                                </button>
                            </div>
                            {process.env.NODE_ENV === 'development' && (
                                <details className="mt-4 text-left">
                                    <summary className="cursor-pointer text-sm text-gray-500">
                                        Error Details (Development)
                                    </summary>
                                    <pre className="mt-2 text-xs bg-gray-100 p-2 rounded overflow-auto">
                                        {error?.stack}
                                    </pre>
                                </details>
                            )}
                        </div>
                    </div>
                );

            case 'component':
            default:
                return (
                    <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                        <div className="flex items-start">
                            <div className="text-yellow-400 text-xl mr-3">⚠️</div>
                            <div className="flex-1">
                                <h3 className="text-sm font-medium text-yellow-800">
                                    Component Error
                                </h3>
                                <p className="text-sm text-yellow-700 mt-1">
                                    This component encountered an error and couldn't render properly.
                                </p>
                                <div className="mt-3">
                                    <button
                                        onClick={this.handleRetry}
                                        className="text-sm bg-yellow-100 text-yellow-800 px-3 py-1 rounded hover:bg-yellow-200 transition-colors"
                                    >
                                        Retry
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                );
        }
    }
}

// Higher-order component for wrapping components with error boundary
export function withErrorBoundary<P extends object>(
    Component: React.ComponentType<P>,
    errorBoundaryProps?: Omit<Props, 'children'>
) {
    const WrappedComponent = (props: P) => (
        <ErrorBoundary {...errorBoundaryProps}>
            <Component {...props} />
        </ErrorBoundary>
    );

    WrappedComponent.displayName = `withErrorBoundary(${Component.displayName || Component.name})`;

    return WrappedComponent;
}

// Hook for error boundary context
export function useErrorHandler() {
    const throwError = React.useCallback((error: Error) => {
        throw error;
    }, []);

    const handleError = React.useCallback((error: any, context?: string) => {
        return errorHandler.handleError(error, context);
    }, []);

    return { throwError, handleError };
}

// Specialized error boundaries for different contexts
export const CriticalErrorBoundary: React.FC<{ children: ReactNode }> = ({ children }) => (
    <ErrorBoundary level="critical">
        {children}
    </ErrorBoundary>
);

export const PageErrorBoundary: React.FC<{ children: ReactNode }> = ({ children }) => (
    <ErrorBoundary level="page">
        {children}
    </ErrorBoundary>
);

export const ComponentErrorBoundary: React.FC<{ children: ReactNode }> = ({ children }) => (
    <ErrorBoundary level="component">
        {children}
    </ErrorBoundary>
);

export default ErrorBoundary;