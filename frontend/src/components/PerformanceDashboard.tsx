/**
 * Performance Dashboard Component
 * Displays real-time performance metrics and optimization status
 */

'use client';

import React, { useState, useEffect } from 'react';
import { usePerformanceOptimization } from '../hooks/usePerformanceOptimization';
import PerformanceIntegration from '../lib/performance/integration';

interface PerformanceMetrics {
    memoryUsage: number;
    cacheHitRate: number;
    bundleSize: number;
    userCount: number;
    performanceScore: number;
    warnings: string[];
}

const PerformanceDashboard: React.FC = () => {
    const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null);
    const [isVisible, setIsVisible] = useState(false);
    const [recommendations, setRecommendations] = useState<string[]>([]);

    const { clearCache, optimizeMemory, getMetrics } = usePerformanceOptimization({
        enableMemoryMonitoring: true,
        enableCaching: true,
        enableDataOptimization: true,
    });

    const performanceIntegration = PerformanceIntegration.getInstance();

    useEffect(() => {
        const updateMetrics = async () => {
            try {
                const systemStatus = await performanceIntegration.getSystemStatus();
                setMetrics(systemStatus);

                const recs = performanceIntegration.getRecommendations();
                setRecommendations(recs);
            } catch (error) {
                console.error('Failed to update performance metrics:', error);
            }
        };

        // Update metrics every 30 seconds
        const interval = setInterval(updateMetrics, 30000);
        updateMetrics(); // Initial update

        return () => clearInterval(interval);
    }, []);

    const getScoreColor = (score: number): string => {
        if (score >= 80) return 'text-green-600';
        if (score >= 60) return 'text-yellow-600';
        return 'text-red-600';
    };

    const getMemoryColor = (usage: number): string => {
        if (usage < 300) return 'text-green-600';
        if (usage < 400) return 'text-yellow-600';
        return 'text-red-600';
    };

    const getCacheColor = (hitRate: number): string => {
        if (hitRate >= 70) return 'text-green-600';
        if (hitRate >= 50) return 'text-yellow-600';
        return 'text-red-600';
    };

    const handleOptimize = () => {
        optimizeMemory();
        clearCache();

        // Show success message
        setTimeout(() => {
            performanceIntegration.getSystemStatus().then(setMetrics);
        }, 1000);
    };

    const handleExportData = () => {
        const data = performanceIntegration.exportPerformanceData();
        const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `performance-data-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    };

    if (!isVisible) {
        return (
            <button
                onClick={() => setIsVisible(true)}
                className="fixed bottom-4 right-4 bg-blue-600 text-white px-4 py-2 rounded-lg shadow-lg hover:bg-blue-700 transition-colors z-50"
                title="Show Performance Dashboard"
            >
                📊 Performance
            </button>
        );
    }

    return (
        <div className="fixed bottom-4 right-4 bg-white border border-gray-200 rounded-lg shadow-xl p-4 w-80 z-50">
            <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-semibold text-gray-800">Performance</h3>
                <button
                    onClick={() => setIsVisible(false)}
                    className="text-gray-500 hover:text-gray-700"
                >
                    ✕
                </button>
            </div>

            {metrics && (
                <div className="space-y-3">
                    {/* Performance Score */}
                    <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">Overall Score:</span>
                        <span className={`font-semibold ${getScoreColor(metrics.performanceScore)}`}>
                            {metrics.performanceScore.toFixed(0)}/100
                        </span>
                    </div>

                    {/* Memory Usage */}
                    <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">Memory Usage:</span>
                        <span className={`font-semibold ${getMemoryColor(metrics.memoryUsage)}`}>
                            {metrics.memoryUsage.toFixed(1)}MB
                        </span>
                    </div>

                    {/* Cache Hit Rate */}
                    <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">Cache Hit Rate:</span>
                        <span className={`font-semibold ${getCacheColor(metrics.cacheHitRate)}`}>
                            {metrics.cacheHitRate.toFixed(1)}%
                        </span>
                    </div>

                    {/* Bundle Size */}
                    <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">Bundle Size:</span>
                        <span className="font-semibold text-gray-800">
                            {metrics.bundleSize.toFixed(2)}MB
                        </span>
                    </div>

                    {/* User Count */}
                    <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">Users Cached:</span>
                        <span className="font-semibold text-gray-800">
                            {metrics.userCount.toLocaleString()}
                        </span>
                    </div>

                    {/* Warnings */}
                    {metrics.warnings.length > 0 && (
                        <div className="mt-3 p-2 bg-yellow-50 border border-yellow-200 rounded">
                            <h4 className="text-sm font-semibold text-yellow-800 mb-1">Warnings:</h4>
                            <ul className="text-xs text-yellow-700 space-y-1">
                                {metrics.warnings.map((warning, index) => (
                                    <li key={index}>• {warning}</li>
                                ))}
                            </ul>
                        </div>
                    )}

                    {/* Recommendations */}
                    {recommendations.length > 0 && (
                        <div className="mt-3 p-2 bg-blue-50 border border-blue-200 rounded">
                            <h4 className="text-sm font-semibold text-blue-800 mb-1">Recommendations:</h4>
                            <ul className="text-xs text-blue-700 space-y-1">
                                {recommendations.slice(0, 2).map((rec, index) => (
                                    <li key={index}>• {rec}</li>
                                ))}
                            </ul>
                        </div>
                    )}

                    {/* Action Buttons */}
                    <div className="flex space-x-2 mt-4">
                        <button
                            onClick={handleOptimize}
                            className="flex-1 bg-green-600 text-white px-3 py-2 rounded text-sm hover:bg-green-700 transition-colors"
                        >
                            Optimize
                        </button>
                        <button
                            onClick={handleExportData}
                            className="flex-1 bg-gray-600 text-white px-3 py-2 rounded text-sm hover:bg-gray-700 transition-colors"
                        >
                            Export
                        </button>
                    </div>

                    {/* Performance Bars */}
                    <div className="mt-4 space-y-2">
                        <div>
                            <div className="flex justify-between text-xs text-gray-600 mb-1">
                                <span>Memory</span>
                                <span>{((metrics.memoryUsage / 512) * 100).toFixed(0)}%</span>
                            </div>
                            <div className="w-full bg-gray-200 rounded-full h-2">
                                <div
                                    className={`h-2 rounded-full ${metrics.memoryUsage < 300 ? 'bg-green-500' :
                                            metrics.memoryUsage < 400 ? 'bg-yellow-500' : 'bg-red-500'
                                        }`}
                                    style={{ width: `${Math.min(100, (metrics.memoryUsage / 512) * 100)}%` }}
                                />
                            </div>
                        </div>

                        <div>
                            <div className="flex justify-between text-xs text-gray-600 mb-1">
                                <span>Cache</span>
                                <span>{metrics.cacheHitRate.toFixed(0)}%</span>
                            </div>
                            <div className="w-full bg-gray-200 rounded-full h-2">
                                <div
                                    className={`h-2 rounded-full ${metrics.cacheHitRate >= 70 ? 'bg-green-500' :
                                            metrics.cacheHitRate >= 50 ? 'bg-yellow-500' : 'bg-red-500'
                                        }`}
                                    style={{ width: `${metrics.cacheHitRate}%` }}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {!metrics && (
                <div className="text-center text-gray-500 py-4">
                    Loading performance data...
                </div>
            )}
        </div>
    );
};

export default PerformanceDashboard;