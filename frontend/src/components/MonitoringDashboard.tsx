'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Alert, AlertDescription } from '@/components/ui/alert';
import {
    Activity,
    AlertTriangle,
    CheckCircle,
    XCircle,
    Clock,
    TrendingUp,
    TrendingDown,
    Server,
    Database,
    Wifi,
    Shield,
    DollarSign,
    Users,
    RefreshCw
} from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';

interface ComponentHealth {
    component_name: string;
    status: 'healthy' | 'warning' | 'critical' | 'offline';
    last_check: number;
    response_time_ms: number;
    error_count: number;
    uptime_percentage: number;
    metrics: Record<string, number>;
}

interface AlertEvent {
    alert_id: string;
    component: string;
    severity: 'low' | 'medium' | 'high' | 'critical';
    message: string;
    timestamp: number;
    metadata: Record<string, string>;
}

interface SystemMetrics {
    cpu_usage_percent: number;
    memory_usage_percent: number;
    disk_usage_percent: number;
    active_connections: number;
    timestamp: number;
}

interface MonitoringStatus {
    enabled: boolean;
    last_health_check: number;
    last_performance_check: number;
    system_health: 'healthy' | 'warning' | 'critical' | 'offline';
    total_components: number;
    active_alerts: number;
    alert_channels: number;
    delivery_stats: {
        total_alerts: number;
        delivered: number;
        failed: number;
        success_rate: number;
    };
}

const MonitoringDashboard: React.FC = () => {
    const [monitoringStatus, setMonitoringStatus] = useState<MonitoringStatus | null>(null);
    const [componentHealth, setComponentHealth] = useState<ComponentHealth[]>([]);
    const [recentAlerts, setRecentAlerts] = useState<AlertEvent[]>([]);
    const [systemMetrics, setSystemMetrics] = useState<SystemMetrics[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [autoRefresh, setAutoRefresh] = useState(true);

    // Mock data for demonstration
    const mockMonitoringStatus: MonitoringStatus = {
        enabled: true,
        last_health_check: Date.now() - 30000,
        last_performance_check: Date.now() - 60000,
        system_health: 'healthy',
        total_components: 6,
        active_alerts: 2,
        alert_channels: 3,
        delivery_stats: {
            total_alerts: 45,
            delivered: 43,
            failed: 2,
            success_rate: 95.6
        }
    };

    const mockComponentHealth: ComponentHealth[] = [
        {
            component_name: 'oracle',
            status: 'healthy',
            last_check: Date.now() - 15000,
            response_time_ms: 250,
            error_count: 0,
            uptime_percentage: 99.9,
            metrics: { requests_per_second: 25.5, cache_hit_rate: 95.2 }
        },
        {
            component_name: 'staking',
            status: 'warning',
            last_check: Date.now() - 20000,
            response_time_ms: 450,
            error_count: 2,
            uptime_percentage: 98.5,
            metrics: { active_validators: 12, total_staked: 5000000 }
        },
        {
            component_name: 'treasury',
            status: 'healthy',
            last_check: Date.now() - 10000,
            response_time_ms: 180,
            error_count: 0,
            uptime_percentage: 99.8,
            metrics: { balance_usd: 25000, last_deposit_hours: 2 }
        },
        {
            component_name: 'security',
            status: 'critical',
            last_check: Date.now() - 5000,
            response_time_ms: 800,
            error_count: 5,
            uptime_percentage: 95.0,
            metrics: { failed_auth_attempts: 15, active_sessions: 150 }
        },
        {
            component_name: 'frontend',
            status: 'healthy',
            last_check: Date.now() - 25000,
            response_time_ms: 320,
            error_count: 1,
            uptime_percentage: 99.5,
            metrics: { active_users: 245, page_load_time: 1200 }
        },
        {
            component_name: 'backend',
            status: 'healthy',
            last_check: Date.now() - 18000,
            response_time_ms: 150,
            error_count: 0,
            uptime_percentage: 99.7,
            metrics: { api_requests_per_minute: 180, database_connections: 25 }
        }
    ];

    const mockRecentAlerts: AlertEvent[] = [
        {
            alert_id: 'alert_001',
            component: 'security',
            severity: 'critical',
            message: 'Excessive failed authentication attempts detected',
            timestamp: Date.now() - 300000,
            metadata: { failed_attempts: '15', threshold: '10' }
        },
        {
            alert_id: 'alert_002',
            component: 'staking',
            severity: 'medium',
            message: 'Validator response time exceeds threshold',
            timestamp: Date.now() - 600000,
            metadata: { response_time: '450ms', threshold: '400ms' }
        },
        {
            alert_id: 'alert_003',
            component: 'oracle',
            severity: 'low',
            message: 'Oracle data refresh completed successfully',
            timestamp: Date.now() - 900000,
            metadata: { refresh_time: '2.5s', data_points: '1250' }
        }
    ];

    const mockSystemMetrics: SystemMetrics[] = Array.from({ length: 24 }, (_, i) => ({
        cpu_usage_percent: 45 + Math.sin(i * 0.5) * 15 + Math.random() * 10,
        memory_usage_percent: 65 + Math.cos(i * 0.3) * 10 + Math.random() * 5,
        disk_usage_percent: 40 + Math.random() * 5,
        active_connections: 150 + Math.sin(i * 0.2) * 30 + Math.random() * 20,
        timestamp: Date.now() - (23 - i) * 60000
    }));

    const fetchMonitoringData = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);

            // In a real implementation, these would be API calls
            await new Promise(resolve => setTimeout(resolve, 500)); // Simulate API delay

            setMonitoringStatus(mockMonitoringStatus);
            setComponentHealth(mockComponentHealth);
            setRecentAlerts(mockRecentAlerts);
            setSystemMetrics(mockSystemMetrics);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to fetch monitoring data');
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        fetchMonitoringData();
    }, [fetchMonitoringData]);

    useEffect(() => {
        if (!autoRefresh) return;

        const interval = setInterval(() => {
            fetchMonitoringData();
        }, 30000); // Refresh every 30 seconds

        return () => clearInterval(interval);
    }, [autoRefresh, fetchMonitoringData]);

    const getStatusIcon = (status: string) => {
        switch (status) {
            case 'healthy':
                return <CheckCircle className="h-4 w-4 text-green-500" />;
            case 'warning':
                return <AlertTriangle className="h-4 w-4 text-yellow-500" />;
            case 'critical':
                return <XCircle className="h-4 w-4 text-red-500" />;
            case 'offline':
                return <XCircle className="h-4 w-4 text-gray-500" />;
            default:
                return <Clock className="h-4 w-4 text-gray-400" />;
        }
    };

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'healthy':
                return 'bg-green-100 text-green-800 border-green-200';
            case 'warning':
                return 'bg-yellow-100 text-yellow-800 border-yellow-200';
            case 'critical':
                return 'bg-red-100 text-red-800 border-red-200';
            case 'offline':
                return 'bg-gray-100 text-gray-800 border-gray-200';
            default:
                return 'bg-gray-100 text-gray-600 border-gray-200';
        }
    };

    const getSeverityColor = (severity: string) => {
        switch (severity) {
            case 'critical':
                return 'bg-red-100 text-red-800 border-red-200';
            case 'high':
                return 'bg-orange-100 text-orange-800 border-orange-200';
            case 'medium':
                return 'bg-yellow-100 text-yellow-800 border-yellow-200';
            case 'low':
                return 'bg-blue-100 text-blue-800 border-blue-200';
            default:
                return 'bg-gray-100 text-gray-600 border-gray-200';
        }
    };

    const formatTimestamp = (timestamp: number) => {
        const date = new Date(timestamp);
        const now = new Date();
        const diffMs = now.getTime() - date.getTime();
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMins / 60);

        if (diffMins < 1) return 'Just now';
        if (diffMins < 60) return `${diffMins}m ago`;
        if (diffHours < 24) return `${diffHours}h ago`;
        return date.toLocaleDateString();
    };

    const getComponentIcon = (component: string) => {
        switch (component) {
            case 'oracle':
                return <Database className="h-4 w-4" />;
            case 'staking':
                return <TrendingUp className="h-4 w-4" />;
            case 'treasury':
                return <DollarSign className="h-4 w-4" />;
            case 'security':
                return <Shield className="h-4 w-4" />;
            case 'frontend':
                return <Users className="h-4 w-4" />;
            case 'backend':
                return <Server className="h-4 w-4" />;
            default:
                return <Activity className="h-4 w-4" />;
        }
    };

    if (loading && !monitoringStatus) {
        return (
            <div className="flex items-center justify-center h-64">
                <RefreshCw className="h-8 w-8 animate-spin text-blue-500" />
                <span className="ml-2 text-lg">Loading monitoring data...</span>
            </div>
        );
    }

    if (error) {
        return (
            <Alert className="m-4">
                <AlertTriangle className="h-4 w-4" />
                <AlertDescription>
                    Error loading monitoring data: {error}
                    <Button onClick={fetchMonitoringData} className="ml-2" size="sm">
                        Retry
                    </Button>
                </AlertDescription>
            </Alert>
        );
    }

    return (
        <div className="p-6 space-y-6">
            {/* Header */}
            <div className="flex items-center justify-between">
                <div>
                    <h1 className="text-3xl font-bold">System Monitoring</h1>
                    <p className="text-gray-600 mt-1">Real-time health and performance monitoring</p>
                </div>
                <div className="flex items-center space-x-2">
                    <Button
                        variant={autoRefresh ? "default" : "outline"}
                        size="sm"
                        onClick={() => setAutoRefresh(!autoRefresh)}
                    >
                        <RefreshCw className={`h-4 w-4 mr-2 ${autoRefresh ? 'animate-spin' : ''}`} />
                        Auto Refresh
                    </Button>
                    <Button onClick={fetchMonitoringData} size="sm" variant="outline">
                        <RefreshCw className="h-4 w-4 mr-2" />
                        Refresh Now
                    </Button>
                </div>
            </div>

            {/* System Overview */}
            {monitoringStatus && (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    <Card>
                        <CardContent className="p-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="text-sm text-gray-600">System Health</p>
                                    <div className="flex items-center mt-1">
                                        {getStatusIcon(monitoringStatus.system_health)}
                                        <span className="ml-2 font-semibold capitalize">
                                            {monitoringStatus.system_health}
                                        </span>
                                    </div>
                                </div>
                                <Activity className="h-8 w-8 text-blue-500" />
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardContent className="p-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="text-sm text-gray-600">Active Alerts</p>
                                    <p className="text-2xl font-bold text-red-600">
                                        {monitoringStatus.active_alerts}
                                    </p>
                                </div>
                                <AlertTriangle className="h-8 w-8 text-red-500" />
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardContent className="p-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="text-sm text-gray-600">Components</p>
                                    <p className="text-2xl font-bold">
                                        {monitoringStatus.total_components}
                                    </p>
                                </div>
                                <Server className="h-8 w-8 text-green-500" />
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardContent className="p-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="text-sm text-gray-600">Alert Success Rate</p>
                                    <p className="text-2xl font-bold text-green-600">
                                        {monitoringStatus.delivery_stats.success_rate.toFixed(1)}%
                                    </p>
                                </div>
                                <TrendingUp className="h-8 w-8 text-green-500" />
                            </div>
                        </CardContent>
                    </Card>
                </div>
            )}

            {/* Main Content Tabs */}
            <Tabs defaultValue="components" className="space-y-4">
                <TabsList>
                    <TabsTrigger value="components">Component Health</TabsTrigger>
                    <TabsTrigger value="alerts">Recent Alerts</TabsTrigger>
                    <TabsTrigger value="metrics">System Metrics</TabsTrigger>
                    <TabsTrigger value="settings">Settings</TabsTrigger>
                </TabsList>

                {/* Component Health Tab */}
                <TabsContent value="components" className="space-y-4">
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                        {componentHealth.map((component) => (
                            <Card key={component.component_name}>
                                <CardHeader className="pb-3">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center space-x-2">
                                            {getComponentIcon(component.component_name)}
                                            <CardTitle className="text-lg capitalize">
                                                {component.component_name}
                                            </CardTitle>
                                        </div>
                                        <Badge className={getStatusColor(component.status)}>
                                            {component.status}
                                        </Badge>
                                    </div>
                                </CardHeader>
                                <CardContent>
                                    <div className="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <p className="text-gray-600">Response Time</p>
                                            <p className="font-semibold">{component.response_time_ms}ms</p>
                                        </div>
                                        <div>
                                            <p className="text-gray-600">Uptime</p>
                                            <p className="font-semibold">{component.uptime_percentage}%</p>
                                        </div>
                                        <div>
                                            <p className="text-gray-600">Errors</p>
                                            <p className="font-semibold text-red-600">{component.error_count}</p>
                                        </div>
                                        <div>
                                            <p className="text-gray-600">Last Check</p>
                                            <p className="font-semibold">{formatTimestamp(component.last_check)}</p>
                                        </div>
                                    </div>

                                    {/* Component-specific metrics */}
                                    {Object.keys(component.metrics).length > 0 && (
                                        <div className="mt-4 pt-4 border-t">
                                            <p className="text-sm font-medium text-gray-700 mb-2">Metrics</p>
                                            <div className="grid grid-cols-2 gap-2 text-xs">
                                                {Object.entries(component.metrics).map(([key, value]) => (
                                                    <div key={key}>
                                                        <p className="text-gray-600 capitalize">
                                                            {key.replace(/_/g, ' ')}
                                                        </p>
                                                        <p className="font-semibold">
                                                            {typeof value === 'number' ? value.toLocaleString() : value}
                                                        </p>
                                                    </div>
                                                ))}
                                            </div>
                                        </div>
                                    )}
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                </TabsContent>

                {/* Recent Alerts Tab */}
                <TabsContent value="alerts" className="space-y-4">
                    <Card>
                        <CardHeader>
                            <CardTitle>Recent Alerts</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div className="space-y-3">
                                {recentAlerts.map((alert) => (
                                    <div key={alert.alert_id} className="flex items-start space-x-3 p-3 border rounded-lg">
                                        <div className="flex-shrink-0 mt-1">
                                            {getComponentIcon(alert.component)}
                                        </div>
                                        <div className="flex-1 min-w-0">
                                            <div className="flex items-center justify-between mb-1">
                                                <p className="text-sm font-medium capitalize">
                                                    {alert.component}
                                                </p>
                                                <div className="flex items-center space-x-2">
                                                    <Badge className={getSeverityColor(alert.severity)}>
                                                        {alert.severity}
                                                    </Badge>
                                                    <span className="text-xs text-gray-500">
                                                        {formatTimestamp(alert.timestamp)}
                                                    </span>
                                                </div>
                                            </div>
                                            <p className="text-sm text-gray-700">{alert.message}</p>
                                            {Object.keys(alert.metadata).length > 0 && (
                                                <div className="mt-2 text-xs text-gray-500">
                                                    {Object.entries(alert.metadata).map(([key, value]) => (
                                                        <span key={key} className="mr-3">
                                                            {key}: {value}
                                                        </span>
                                                    ))}
                                                </div>
                                            )}
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </CardContent>
                    </Card>
                </TabsContent>

                {/* System Metrics Tab */}
                <TabsContent value="metrics" className="space-y-4">
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                        <Card>
                            <CardHeader>
                                <CardTitle>CPU Usage</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <ResponsiveContainer width="100%" height={200}>
                                    <AreaChart data={systemMetrics}>
                                        <CartesianGrid strokeDasharray="3 3" />
                                        <XAxis
                                            dataKey="timestamp"
                                            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
                                        />
                                        <YAxis domain={[0, 100]} />
                                        <Tooltip
                                            labelFormatter={(value) => new Date(value).toLocaleString()}
                                            formatter={(value: number) => [`${value.toFixed(1)}%`, 'CPU Usage']}
                                        />
                                        <Area
                                            type="monotone"
                                            dataKey="cpu_usage_percent"
                                            stroke="#3b82f6"
                                            fill="#3b82f6"
                                            fillOpacity={0.3}
                                        />
                                    </AreaChart>
                                </ResponsiveContainer>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>Memory Usage</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <ResponsiveContainer width="100%" height={200}>
                                    <AreaChart data={systemMetrics}>
                                        <CartesianGrid strokeDasharray="3 3" />
                                        <XAxis
                                            dataKey="timestamp"
                                            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
                                        />
                                        <YAxis domain={[0, 100]} />
                                        <Tooltip
                                            labelFormatter={(value) => new Date(value).toLocaleString()}
                                            formatter={(value: number) => [`${value.toFixed(1)}%`, 'Memory Usage']}
                                        />
                                        <Area
                                            type="monotone"
                                            dataKey="memory_usage_percent"
                                            stroke="#10b981"
                                            fill="#10b981"
                                            fillOpacity={0.3}
                                        />
                                    </AreaChart>
                                </ResponsiveContainer>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>Active Connections</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <ResponsiveContainer width="100%" height={200}>
                                    <LineChart data={systemMetrics}>
                                        <CartesianGrid strokeDasharray="3 3" />
                                        <XAxis
                                            dataKey="timestamp"
                                            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
                                        />
                                        <YAxis />
                                        <Tooltip
                                            labelFormatter={(value) => new Date(value).toLocaleString()}
                                            formatter={(value: number) => [value.toFixed(0), 'Connections']}
                                        />
                                        <Line
                                            type="monotone"
                                            dataKey="active_connections"
                                            stroke="#f59e0b"
                                            strokeWidth={2}
                                        />
                                    </LineChart>
                                </ResponsiveContainer>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>Disk Usage</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <ResponsiveContainer width="100%" height={200}>
                                    <AreaChart data={systemMetrics}>
                                        <CartesianGrid strokeDasharray="3 3" />
                                        <XAxis
                                            dataKey="timestamp"
                                            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
                                        />
                                        <YAxis domain={[0, 100]} />
                                        <Tooltip
                                            labelFormatter={(value) => new Date(value).toLocaleString()}
                                            formatter={(value: number) => [`${value.toFixed(1)}%`, 'Disk Usage']}
                                        />
                                        <Area
                                            type="monotone"
                                            dataKey="disk_usage_percent"
                                            stroke="#ef4444"
                                            fill="#ef4444"
                                            fillOpacity={0.3}
                                        />
                                    </AreaChart>
                                </ResponsiveContainer>
                            </CardContent>
                        </Card>
                    </div>
                </TabsContent>

                {/* Settings Tab */}
                <TabsContent value="settings" className="space-y-4">
                    <Card>
                        <CardHeader>
                            <CardTitle>Monitoring Configuration</CardTitle>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="font-medium">Monitoring Enabled</p>
                                    <p className="text-sm text-gray-600">Enable or disable system monitoring</p>
                                </div>
                                <Badge className={monitoringStatus?.enabled ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}>
                                    {monitoringStatus?.enabled ? 'Enabled' : 'Disabled'}
                                </Badge>
                            </div>

                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="font-medium">Alert Channels</p>
                                    <p className="text-sm text-gray-600">Number of configured alert channels</p>
                                </div>
                                <Badge variant="outline">
                                    {monitoringStatus?.alert_channels || 0} channels
                                </Badge>
                            </div>

                            <div className="flex items-center justify-between">
                                <div>
                                    <p className="font-medium">Auto Refresh</p>
                                    <p className="text-sm text-gray-600">Automatically refresh monitoring data</p>
                                </div>
                                <Badge className={autoRefresh ? 'bg-blue-100 text-blue-800' : 'bg-gray-100 text-gray-800'}>
                                    {autoRefresh ? 'On' : 'Off'}
                                </Badge>
                            </div>
                        </CardContent>
                    </Card>
                </TabsContent>
            </Tabs>
        </div>
    );
};

export default MonitoringDashboard;