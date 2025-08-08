'use client';

import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';

interface SecurityEvent {
    event_id: number;
    event_type: string;
    user?: string;
    timestamp: number;
    ip_address?: string;
    user_agent?: string;
    device_id?: string;
    session_id?: string;
    transaction_id?: string;
    amount?: number;
    details: string;
    metadata: Record<string, string>;
    security_level: 'Low' | 'Medium' | 'High' | 'Critical';
    requires_investigation: boolean;
}

interface SecurityAlert {
    alert_id: number;
    alert_type: string;
    user?: string;
    created_at: number;
    updated_at: number;
    status: 'Active' | 'Investigating' | 'Resolved' | 'FalsePositive';
    security_level: 'Low' | 'Medium' | 'High' | 'Critical';
    description: string;
    related_events: number[];
    investigation_notes: string[];
    assigned_to?: string;
    auto_resolved: boolean;
    resolution_time?: number;
    false_positive: boolean;
}

interface UserBehaviorProfile {
    user: string;
    created_at: number;
    last_updated: number;
    typical_login_hours: number[];
    typical_login_days: number[];
    common_locations: string[];
    common_devices: string[];
    common_user_agents: string[];
    average_transaction_amount: number;
    max_transaction_amount: number;
    transaction_frequency: number;
    preferred_payment_methods: string[];
    failed_login_attempts: number;
    suspicious_activity_count: number;
    last_suspicious_activity?: number;
    risk_score: number;
    is_high_risk: boolean;
    kyc_tier: number;
    compliance_alerts: number;
    last_compliance_review?: number;
}

interface SecurityMetrics {
    total_events: number;
    active_alerts: number;
    resolved_alerts: number;
    false_positives: number;
    high_risk_users: number;
    total_users: number;
    total_audit_trails: number;
    compliance_trails: number;
}

interface AuditTrail {
    trail_id: number;
    user?: string;
    action: string;
    resource: string;
    timestamp: number;
    ip_address?: string;
    user_agent?: string;
    session_id?: string;
    before_state?: string;
    after_state?: string;
    success: boolean;
    error_message?: string;
    compliance_relevant: boolean;
    retention_period: number;
}

const SecurityMonitoringDashboard: React.FC = () => {
    const { t } = useTranslation();
    const [activeTab, setActiveTab] = useState<'overview' | 'events' | 'alerts' | 'users' | 'audit'>('overview');
    const [metrics, setMetrics] = useState<SecurityMetrics | null>(null);
    const [events, setEvents] = useState<SecurityEvent[]>([]);
    const [alerts, setAlerts] = useState<SecurityAlert[]>([]);
    const [userProfiles, setUserProfiles] = useState<UserBehaviorProfile[]>([]);
    const [auditTrails, setAuditTrails] = useState<AuditTrail[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // Mock data for demonstration
    useEffect(() => {
        const loadMockData = () => {
            setMetrics({
                total_events: 1247,
                active_alerts: 3,
                resolved_alerts: 45,
                false_positives: 12,
                high_risk_users: 2,
                total_users: 156,
                total_audit_trails: 2341,
                compliance_trails: 234
            });

            setEvents([
                {
                    event_id: 1,
                    event_type: 'LoginSuccess',
                    user: 'user123',
                    timestamp: Date.now() - 3600000,
                    ip_address: '192.168.1.100',
                    details: 'User logged in successfully',
                    metadata: {},
                    security_level: 'Low',
                    requires_investigation: false
                },
                {
                    event_id: 2,
                    event_type: 'BTCCommitment',
                    user: 'user456',
                    timestamp: Date.now() - 7200000,
                    amount: 150000,
                    details: 'High value BTC commitment',
                    metadata: { btc_address: 'bc1q...' },
                    security_level: 'High',
                    requires_investigation: true
                }
            ]);

            setAlerts([
                {
                    alert_id: 1,
                    alert_type: 'HighValueTransaction',
                    user: 'user456',
                    created_at: Date.now() - 7200000,
                    updated_at: Date.now() - 3600000,
                    status: 'Active',
                    security_level: 'High',
                    description: 'High value transaction detected: $150,000',
                    related_events: [2],
                    investigation_notes: [],
                    auto_resolved: false,
                    false_positive: false
                }
            ]);

            setUserProfiles([
                {
                    user: 'user123',
                    created_at: Date.now() - 86400000 * 30,
                    last_updated: Date.now() - 3600000,
                    typical_login_hours: [9, 10, 14, 15, 16],
                    typical_login_days: [1, 2, 3, 4, 5],
                    common_locations: ['192.168.1.0/24', 'US-CA'],
                    common_devices: ['device_abc123'],
                    common_user_agents: ['Mozilla/5.0...'],
                    average_transaction_amount: 5000,
                    max_transaction_amount: 25000,
                    transaction_frequency: 2.5,
                    preferred_payment_methods: ['BTC', 'USDC'],
                    failed_login_attempts: 0,
                    suspicious_activity_count: 0,
                    risk_score: 15,
                    is_high_risk: false,
                    kyc_tier: 1,
                    compliance_alerts: 0
                }
            ]);

            setAuditTrails([
                {
                    trail_id: 1,
                    user: 'user123',
                    action: 'UPDATE_PROFILE',
                    resource: 'user_profile',
                    timestamp: Date.now() - 3600000,
                    ip_address: '192.168.1.100',
                    success: true,
                    compliance_relevant: false,
                    retention_period: 86400 * 365 * 7
                }
            ]);

            setLoading(false);
        };

        loadMockData();
    }, []);

    const formatTimestamp = (timestamp: number) => {
        return new Date(timestamp).toLocaleString();
    };

    const getSecurityLevelColor = (level: string) => {
        switch (level) {
            case 'Critical': return 'text-red-600 bg-red-100';
            case 'High': return 'text-orange-600 bg-orange-100';
            case 'Medium': return 'text-yellow-600 bg-yellow-100';
            case 'Low': return 'text-green-600 bg-green-100';
            default: return 'text-gray-600 bg-gray-100';
        }
    };

    const getAlertStatusColor = (status: string) => {
        switch (status) {
            case 'Active': return 'text-red-600 bg-red-100';
            case 'Investigating': return 'text-yellow-600 bg-yellow-100';
            case 'Resolved': return 'text-green-600 bg-green-100';
            case 'FalsePositive': return 'text-gray-600 bg-gray-100';
            default: return 'text-gray-600 bg-gray-100';
        }
    };

    if (loading) {
        return (
            <div className="flex items-center justify-center h-64">
                <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                <strong className="font-bold">{t('security.error')}: </strong>
                <span className="block sm:inline">{error}</span>
            </div>
        );
    }

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div className="mb-8">
                <h1 className="text-3xl font-bold text-gray-900 mb-2">
                    {t('security.monitoring.title')}
                </h1>
                <p className="text-gray-600">
                    {t('security.monitoring.description')}
                </p>
            </div>

            {/* Tab Navigation */}
            <div className="border-b border-gray-200 mb-6">
                <nav className="-mb-px flex space-x-8">
                    {[
                        { key: 'overview', label: t('security.tabs.overview') },
                        { key: 'events', label: t('security.tabs.events') },
                        { key: 'alerts', label: t('security.tabs.alerts') },
                        { key: 'users', label: t('security.tabs.users') },
                        { key: 'audit', label: t('security.tabs.audit') }
                    ].map((tab) => (
                        <button
                            key={tab.key}
                            onClick={() => setActiveTab(tab.key as any)}
                            className={`py-2 px-1 border-b-2 font-medium text-sm ${activeTab === tab.key
                                    ? 'border-blue-500 text-blue-600'
                                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                                }`}
                        >
                            {tab.label}
                        </button>
                    ))}
                </nav>
            </div>

            {/* Overview Tab */}
            {activeTab === 'overview' && metrics && (
                <div className="space-y-6">
                    {/* Metrics Cards */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        <div className="bg-white overflow-hidden shadow rounded-lg">
                            <div className="p-5">
                                <div className="flex items-center">
                                    <div className="flex-shrink-0">
                                        <div className="w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center">
                                            <span className="text-white text-sm font-medium">E</span>
                                        </div>
                                    </div>
                                    <div className="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt className="text-sm font-medium text-gray-500 truncate">
                                                {t('security.metrics.totalEvents')}
                                            </dt>
                                            <dd className="text-lg font-medium text-gray-900">
                                                {metrics.total_events.toLocaleString()}
                                            </dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="bg-white overflow-hidden shadow rounded-lg">
                            <div className="p-5">
                                <div className="flex items-center">
                                    <div className="flex-shrink-0">
                                        <div className="w-8 h-8 bg-red-500 rounded-md flex items-center justify-center">
                                            <span className="text-white text-sm font-medium">A</span>
                                        </div>
                                    </div>
                                    <div className="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt className="text-sm font-medium text-gray-500 truncate">
                                                {t('security.metrics.activeAlerts')}
                                            </dt>
                                            <dd className="text-lg font-medium text-gray-900">
                                                {metrics.active_alerts}
                                            </dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="bg-white overflow-hidden shadow rounded-lg">
                            <div className="p-5">
                                <div className="flex items-center">
                                    <div className="flex-shrink-0">
                                        <div className="w-8 h-8 bg-yellow-500 rounded-md flex items-center justify-center">
                                            <span className="text-white text-sm font-medium">U</span>
                                        </div>
                                    </div>
                                    <div className="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt className="text-sm font-medium text-gray-500 truncate">
                                                {t('security.metrics.highRiskUsers')}
                                            </dt>
                                            <dd className="text-lg font-medium text-gray-900">
                                                {metrics.high_risk_users}
                                            </dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="bg-white overflow-hidden shadow rounded-lg">
                            <div className="p-5">
                                <div className="flex items-center">
                                    <div className="flex-shrink-0">
                                        <div className="w-8 h-8 bg-green-500 rounded-md flex items-center justify-center">
                                            <span className="text-white text-sm font-medium">C</span>
                                        </div>
                                    </div>
                                    <div className="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt className="text-sm font-medium text-gray-500 truncate">
                                                {t('security.metrics.complianceTrails')}
                                            </dt>
                                            <dd className="text-lg font-medium text-gray-900">
                                                {metrics.compliance_trails}
                                            </dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    {/* Recent Alerts */}
                    <div className="bg-white shadow rounded-lg">
                        <div className="px-4 py-5 sm:p-6">
                            <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                                {t('security.recentAlerts')}
                            </h3>
                            <div className="space-y-3">
                                {alerts.slice(0, 5).map((alert) => (
                                    <div key={alert.alert_id} className="flex items-center justify-between p-3 bg-gray-50 rounded-md">
                                        <div className="flex items-center space-x-3">
                                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${getSecurityLevelColor(alert.security_level)}`}>
                                                {alert.security_level}
                                            </span>
                                            <span className="text-sm text-gray-900">{alert.description}</span>
                                        </div>
                                        <div className="flex items-center space-x-2">
                                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${getAlertStatusColor(alert.status)}`}>
                                                {alert.status}
                                            </span>
                                            <span className="text-xs text-gray-500">
                                                {formatTimestamp(alert.created_at)}
                                            </span>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {/* Events Tab */}
            {activeTab === 'events' && (
                <div className="bg-white shadow rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                            {t('security.events.title')}
                        </h3>
                        <div className="overflow-x-auto">
                            <table className="min-w-full divide-y divide-gray-200">
                                <thead className="bg-gray-50">
                                    <tr>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.id')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.type')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.user')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.level')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.timestamp')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.events.details')}
                                        </th>
                                    </tr>
                                </thead>
                                <tbody className="bg-white divide-y divide-gray-200">
                                    {events.map((event) => (
                                        <tr key={event.event_id}>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {event.event_id}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {event.event_type}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {event.user || '-'}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <span className={`px-2 py-1 text-xs font-medium rounded-full ${getSecurityLevelColor(event.security_level)}`}>
                                                    {event.security_level}
                                                </span>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {formatTimestamp(event.timestamp)}
                                            </td>
                                            <td className="px-6 py-4 text-sm text-gray-900">
                                                {event.details}
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            )}

            {/* Alerts Tab */}
            {activeTab === 'alerts' && (
                <div className="bg-white shadow rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                            {t('security.alerts.title')}
                        </h3>
                        <div className="space-y-4">
                            {alerts.map((alert) => (
                                <div key={alert.alert_id} className="border border-gray-200 rounded-lg p-4">
                                    <div className="flex items-center justify-between mb-2">
                                        <div className="flex items-center space-x-3">
                                            <span className="text-sm font-medium text-gray-900">
                                                Alert #{alert.alert_id}
                                            </span>
                                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${getSecurityLevelColor(alert.security_level)}`}>
                                                {alert.security_level}
                                            </span>
                                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${getAlertStatusColor(alert.status)}`}>
                                                {alert.status}
                                            </span>
                                        </div>
                                        <span className="text-xs text-gray-500">
                                            {formatTimestamp(alert.created_at)}
                                        </span>
                                    </div>
                                    <p className="text-sm text-gray-700 mb-2">{alert.description}</p>
                                    {alert.user && (
                                        <p className="text-xs text-gray-500">User: {alert.user}</p>
                                    )}
                                    {alert.related_events.length > 0 && (
                                        <p className="text-xs text-gray-500">
                                            Related Events: {alert.related_events.join(', ')}
                                        </p>
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            )}

            {/* Users Tab */}
            {activeTab === 'users' && (
                <div className="bg-white shadow rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                            {t('security.users.title')}
                        </h3>
                        <div className="space-y-4">
                            {userProfiles.map((profile) => (
                                <div key={profile.user} className="border border-gray-200 rounded-lg p-4">
                                    <div className="flex items-center justify-between mb-2">
                                        <span className="text-sm font-medium text-gray-900">
                                            {profile.user}
                                        </span>
                                        <div className="flex items-center space-x-2">
                                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${profile.is_high_risk ? 'text-red-600 bg-red-100' : 'text-green-600 bg-green-100'
                                                }`}>
                                                Risk Score: {profile.risk_score}
                                            </span>
                                            <span className="text-xs text-gray-500">
                                                KYC Tier {profile.kyc_tier}
                                            </span>
                                        </div>
                                    </div>
                                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-xs text-gray-600">
                                        <div>
                                            <span className="font-medium">Avg Transaction:</span> ${profile.average_transaction_amount.toLocaleString()}
                                        </div>
                                        <div>
                                            <span className="font-medium">Max Transaction:</span> ${profile.max_transaction_amount.toLocaleString()}
                                        </div>
                                        <div>
                                            <span className="font-medium">Failed Logins:</span> {profile.failed_login_attempts}
                                        </div>
                                        <div>
                                            <span className="font-medium">Compliance Alerts:</span> {profile.compliance_alerts}
                                        </div>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            )}

            {/* Audit Tab */}
            {activeTab === 'audit' && (
                <div className="bg-white shadow rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">
                            {t('security.audit.title')}
                        </h3>
                        <div className="overflow-x-auto">
                            <table className="min-w-full divide-y divide-gray-200">
                                <thead className="bg-gray-50">
                                    <tr>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.id')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.user')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.action')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.resource')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.status')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.timestamp')}
                                        </th>
                                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            {t('security.audit.compliance')}
                                        </th>
                                    </tr>
                                </thead>
                                <tbody className="bg-white divide-y divide-gray-200">
                                    {auditTrails.map((trail) => (
                                        <tr key={trail.trail_id}>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {trail.trail_id}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {trail.user || '-'}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {trail.action}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {trail.resource}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <span className={`px-2 py-1 text-xs font-medium rounded-full ${trail.success ? 'text-green-600 bg-green-100' : 'text-red-600 bg-red-100'
                                                    }`}>
                                                    {trail.success ? 'Success' : 'Failed'}
                                                </span>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {formatTimestamp(trail.timestamp)}
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <span className={`px-2 py-1 text-xs font-medium rounded-full ${trail.compliance_relevant ? 'text-blue-600 bg-blue-100' : 'text-gray-600 bg-gray-100'
                                                    }`}>
                                                    {trail.compliance_relevant ? 'Yes' : 'No'}
                                                </span>
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default SecurityMonitoringDashboard;