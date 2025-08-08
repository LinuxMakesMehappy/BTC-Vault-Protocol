/**
 * Notification System
 * Professional notification management for user feedback
 * 
 * CLASSIFICATION: CONFIDENTIAL - AUTHORIZED PERSONNEL ONLY
 */

import React, { createContext, useContext, useReducer, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { toast, Toaster } from 'react-hot-toast';

// Notification types
export enum NotificationType {
    SUCCESS = 'success',
    ERROR = 'error',
    WARNING = 'warning',
    INFO = 'info',
    SECURITY = 'security',
}

// Notification interface
export interface Notification {
    id: string;
    type: NotificationType;
    title: string;
    message: string;
    duration?: number;
    persistent?: boolean;
    actions?: NotificationAction[];
    timestamp: number;
    read: boolean;
}

// Notification action interface
export interface NotificationAction {
    label: string;
    action: () => void;
    style?: 'primary' | 'secondary' | 'danger';
}

// Notification state
interface NotificationState {
    notifications: Notification[];
    unreadCount: number;
}

// Notification actions
type NotificationActionType =
    | { type: 'ADD_NOTIFICATION'; payload: Notification }
    | { type: 'REMOVE_NOTIFICATION'; payload: string }
    | { type: 'MARK_READ'; payload: string }
    | { type: 'MARK_ALL_READ' }
    | { type: 'CLEAR_ALL' };

// Notification reducer
const notificationReducer = (
    state: NotificationState,
    action: NotificationActionType
): NotificationState => {
    switch (action.type) {
        case 'ADD_NOTIFICATION':
            return {
                notifications: [action.payload, ...state.notifications],
                unreadCount: state.unreadCount + 1,
            };
        case 'REMOVE_NOTIFICATION':
            const filtered = state.notifications.filter(n => n.id !== action.payload);
            const removedNotification = state.notifications.find(n => n.id === action.payload);
            return {
                notifications: filtered,
                unreadCount: removedNotification && !removedNotification.read
                    ? state.unreadCount - 1
                    : state.unreadCount,
            };
        case 'MARK_READ':
            return {
                notifications: state.notifications.map(n =>
                    n.id === action.payload ? { ...n, read: true } : n
                ),
                unreadCount: Math.max(0, state.unreadCount - 1),
            };
        case 'MARK_ALL_READ':
            return {
                notifications: state.notifications.map(n => ({ ...n, read: true })),
                unreadCount: 0,
            };
        case 'CLEAR_ALL':
            return {
                notifications: [],
                unreadCount: 0,
            };
        default:
            return state;
    }
};

// Notification context
interface NotificationContextType {
    state: NotificationState;
    addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => void;
    removeNotification: (id: string) => void;
    markAsRead: (id: string) => void;
    markAllAsRead: () => void;
    clearAll: () => void;
    showToast: (type: NotificationType, message: string, options?: any) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

// Notification provider
export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [state, dispatch] = useReducer(notificationReducer, {
        notifications: [],
        unreadCount: 0,
    });

    const addNotification = useCallback((notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => {
        const newNotification: Notification = {
            ...notification,
            id: `notification_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            timestamp: Date.now(),
            read: false,
        };

        dispatch({ type: 'ADD_NOTIFICATION', payload: newNotification });

        // Auto-remove non-persistent notifications
        if (!notification.persistent) {
            setTimeout(() => {
                dispatch({ type: 'REMOVE_NOTIFICATION', payload: newNotification.id });
            }, notification.duration || 5000);
        }
    }, []);

    const removeNotification = useCallback((id: string) => {
        dispatch({ type: 'REMOVE_NOTIFICATION', payload: id });
    }, []);

    const markAsRead = useCallback((id: string) => {
        dispatch({ type: 'MARK_READ', payload: id });
    }, []);

    const markAllAsRead = useCallback(() => {
        dispatch({ type: 'MARK_ALL_READ' });
    }, []);

    const clearAll = useCallback(() => {
        dispatch({ type: 'CLEAR_ALL' });
    }, []);

    const showToast = useCallback((type: NotificationType, message: string, options: any = {}) => {
        const toastOptions = {
            duration: options.duration || 4000,
            position: options.position || 'top-right',
            style: {
                borderRadius: '8px',
                fontSize: '14px',
                fontWeight: '500',
                ...options.style,
            },
        };

        switch (type) {
            case NotificationType.SUCCESS:
                toast.success(message, {
                    ...toastOptions,
                    icon: '✅',
                    style: {
                        ...toastOptions.style,
                        background: '#10b981',
                        color: '#ffffff',
                    },
                });
                break;
            case NotificationType.ERROR:
                toast.error(message, {
                    ...toastOptions,
                    icon: '❌',
                    style: {
                        ...toastOptions.style,
                        background: '#ef4444',
                        color: '#ffffff',
                    },
                });
                break;
            case NotificationType.WARNING:
                toast(message, {
                    ...toastOptions,
                    icon: '⚠️',
                    style: {
                        ...toastOptions.style,
                        background: '#f59e0b',
                        color: '#ffffff',
                    },
                });
                break;
            case NotificationType.INFO:
                toast(message, {
                    ...toastOptions,
                    icon: 'ℹ️',
                    style: {
                        ...toastOptions.style,
                        background: '#3b82f6',
                        color: '#ffffff',
                    },
                });
                break;
            case NotificationType.SECURITY:
                toast(message, {
                    ...toastOptions,
                    icon: '🔒',
                    duration: 8000, // Longer duration for security messages
                    style: {
                        ...toastOptions.style,
                        background: '#dc2626',
                        color: '#ffffff',
                        border: '2px solid #b91c1c',
                    },
                });
                break;
        }
    }, []);

    const value: NotificationContextType = {
        state,
        addNotification,
        removeNotification,
        markAsRead,
        markAllAsRead,
        clearAll,
        showToast,
    };

    return (
        <NotificationContext.Provider value={value}>
            {children}
            <Toaster />
        </NotificationContext.Provider>
    );
};

// Hook to use notifications
export const useNotifications = () => {
    const context = useContext(NotificationContext);
    if (context === undefined) {
        throw new Error('useNotifications must be used within a NotificationProvider');
    }
    return context;
};

// Notification component
interface NotificationItemProps {
    notification: Notification;
    onRemove: (id: string) => void;
    onMarkRead: (id: string) => void;
}

const NotificationItem: React.FC<NotificationItemProps> = ({
    notification,
    onRemove,
    onMarkRead,
}) => {
    const getIcon = (type: NotificationType) => {
        switch (type) {
            case NotificationType.SUCCESS: return '✅';
            case NotificationType.ERROR: return '❌';
            case NotificationType.WARNING: return '⚠️';
            case NotificationType.INFO: return 'ℹ️';
            case NotificationType.SECURITY: return '🔒';
            default: return 'ℹ️';
        }
    };

    const getColorClasses = (type: NotificationType) => {
        switch (type) {
            case NotificationType.SUCCESS:
                return 'bg-green-50 border-green-200 text-green-800';
            case NotificationType.ERROR:
                return 'bg-red-50 border-red-200 text-red-800';
            case NotificationType.WARNING:
                return 'bg-yellow-50 border-yellow-200 text-yellow-800';
            case NotificationType.INFO:
                return 'bg-blue-50 border-blue-200 text-blue-800';
            case NotificationType.SECURITY:
                return 'bg-red-50 border-red-300 text-red-900';
            default:
                return 'bg-gray-50 border-gray-200 text-gray-800';
        }
    };

    const handleClick = () => {
        if (!notification.read) {
            onMarkRead(notification.id);
        }
    };

    return (
        <motion.div
            initial={{ opacity: 0, x: 300 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: 300 }}
            className={`
        border rounded-lg p-4 mb-3 cursor-pointer transition-all duration-200
        ${getColorClasses(notification.type)}
        ${!notification.read ? 'shadow-md' : 'opacity-75'}
      `}
            onClick={handleClick}
        >
            <div className="flex items-start justify-between">
                <div className="flex items-start space-x-3">
                    <div className="text-xl">{getIcon(notification.type)}</div>
                    <div className="flex-1">
                        <h4 className="font-semibold text-sm">{notification.title}</h4>
                        <p className="text-sm mt-1">{notification.message}</p>
                        <p className="text-xs mt-2 opacity-75">
                            {new Date(notification.timestamp).toLocaleString()}
                        </p>
                    </div>
                </div>
                <div className="flex items-center space-x-2">
                    {!notification.read && (
                        <div className="w-2 h-2 bg-blue-500 rounded-full" />
                    )}
                    <button
                        onClick={(e) => {
                            e.stopPropagation();
                            onRemove(notification.id);
                        }}
                        className="text-gray-400 hover:text-gray-600 transition-colors"
                    >
                        ✕
                    </button>
                </div>
            </div>

            {notification.actions && notification.actions.length > 0 && (
                <div className="mt-3 flex space-x-2">
                    {notification.actions.map((action, index) => (
                        <button
                            key={index}
                            onClick={(e) => {
                                e.stopPropagation();
                                action.action();
                            }}
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
        </motion.div>
    );
};

// Notification panel component
interface NotificationPanelProps {
    isOpen: boolean;
    onClose: () => void;
}

export const NotificationPanel: React.FC<NotificationPanelProps> = ({
    isOpen,
    onClose,
}) => {
    const { state, removeNotification, markAsRead, markAllAsRead, clearAll } = useNotifications();

    return (
        <AnimatePresence>
            {isOpen && (
                <>
                    {/* Backdrop */}
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        className="fixed inset-0 bg-black bg-opacity-25 z-40"
                        onClick={onClose}
                    />

                    {/* Panel */}
                    <motion.div
                        initial={{ opacity: 0, x: 300 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: 300 }}
                        className="fixed right-0 top-0 h-full w-96 bg-white shadow-xl z-50 overflow-hidden flex flex-col"
                    >
                        {/* Header */}
                        <div className="p-4 border-b bg-gray-50">
                            <div className="flex items-center justify-between">
                                <h2 className="text-lg font-semibold text-gray-900">
                                    Notifications
                                    {state.unreadCount > 0 && (
                                        <span className="ml-2 bg-blue-500 text-white text-xs px-2 py-1 rounded-full">
                                            {state.unreadCount}
                                        </span>
                                    )}
                                </h2>
                                <button
                                    onClick={onClose}
                                    className="text-gray-400 hover:text-gray-600 transition-colors"
                                >
                                    ✕
                                </button>
                            </div>

                            {state.notifications.length > 0 && (
                                <div className="flex space-x-2 mt-2">
                                    <button
                                        onClick={markAllAsRead}
                                        className="text-xs text-blue-600 hover:text-blue-800 transition-colors"
                                    >
                                        Mark all read
                                    </button>
                                    <button
                                        onClick={clearAll}
                                        className="text-xs text-red-600 hover:text-red-800 transition-colors"
                                    >
                                        Clear all
                                    </button>
                                </div>
                            )}
                        </div>

                        {/* Content */}
                        <div className="flex-1 overflow-y-auto p-4">
                            {state.notifications.length === 0 ? (
                                <div className="text-center text-gray-500 mt-8">
                                    <div className="text-4xl mb-2">🔔</div>
                                    <p>No notifications</p>
                                </div>
                            ) : (
                                <AnimatePresence>
                                    {state.notifications.map((notification) => (
                                        <NotificationItem
                                            key={notification.id}
                                            notification={notification}
                                            onRemove={removeNotification}
                                            onMarkRead={markAsRead}
                                        />
                                    ))}
                                </AnimatePresence>
                            )}
                        </div>
                    </motion.div>
                </>
            )}
        </AnimatePresence>
    );
};

// Notification bell component
interface NotificationBellProps {
    onClick: () => void;
    className?: string;
}

export const NotificationBell: React.FC<NotificationBellProps> = ({
    onClick,
    className = '',
}) => {
    const { state } = useNotifications();

    return (
        <button
            onClick={onClick}
            className={`relative p-2 text-gray-600 hover:text-gray-900 transition-colors ${className}`}
        >
            <div className="text-xl">🔔</div>
            {state.unreadCount > 0 && (
                <motion.div
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    className="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center"
                >
                    {state.unreadCount > 99 ? '99+' : state.unreadCount}
                </motion.div>
            )}
        </button>
    );
};

export default NotificationSystem;
// Uti
lity functions for common notifications
export const useCommonNotifications = () => {
        const { addNotification, showToast } = useNotifications();

        const notifySuccess = useCallback((title: string, message: string) => {
            showToast(NotificationType.SUCCESS, message);
            addNotification({
                type: NotificationType.SUCCESS,
                title,
                message,
                duration: 5000,
            });
        }, [addNotification, showToast]);

        const notifyError = useCallback((title: string, message: string, persistent = false) => {
            showToast(NotificationType.ERROR, message);
            addNotification({
                type: NotificationType.ERROR,
                title,
                message,
                persistent,
                duration: persistent ? undefined : 8000,
            });
        }, [addNotification, showToast]);

        const notifyWarning = useCallback((title: string, message: string) => {
            showToast(NotificationType.WARNING, message);
            addNotification({
                type: NotificationType.WARNING,
                title,
                message,
                duration: 6000,
            });
        }, [addNotification, showToast]);

        const notifyInfo = useCallback((title: string, message: string) => {
            showToast(NotificationType.INFO, message);
            addNotification({
                type: NotificationType.INFO,
                title,
                message,
                duration: 4000,
            });
        }, [addNotification, showToast]);

        const notifySecurity = useCallback((title: string, message: string, actions?: NotificationAction[]) => {
            showToast(NotificationType.SECURITY, message);
            addNotification({
                type: NotificationType.SECURITY,
                title,
                message,
                persistent: true,
                actions,
            });
        }, [addNotification, showToast]);

        const notifyTransaction = useCallback((stage: string, hash?: string) => {
            const messages = {
                preparing: 'Preparing transaction...',
                signing: 'Please sign the transaction in your wallet',
                broadcasting: 'Broadcasting transaction to network...',
                confirming: 'Waiting for transaction confirmation...',
                confirmed: hash ? `Transaction confirmed: ${hash.slice(0, 8)}...` : 'Transaction confirmed!',
                failed: 'Transaction failed. Please try again.',
            };

            const message = messages[stage as keyof typeof messages] || stage;

            if (stage === 'confirmed') {
                notifySuccess('Transaction Successful', message);
            } else if (stage === 'failed') {
                notifyError('Transaction Failed', message);
            } else {
                notifyInfo('Transaction Update', message);
            }
        }, [notifySuccess, notifyError, notifyInfo]);

        return {
            notifySuccess,
            notifyError,
            notifyWarning,
            notifyInfo,
            notifySecurity,
            notifyTransaction,
        };
    };