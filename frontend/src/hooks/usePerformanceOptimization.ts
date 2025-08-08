/**
 * React hook for performance optimization
 * Provides memory monitoring, caching, and resource management for low-resource systems
 */

import { useEffect, useRef, useCallback, useMemo } from 'react';
import PerformanceMonitor from '../lib/performance/monitor';
import OracleCache from '../lib/cache/oracle-cache';
import UIStateCache from '../lib/cache/ui-state-cache';
import UserManager from '../lib/data-structures/user-manager';
import MemoryPoolManager from '../lib/data-structures/memory-pool';

interface PerformanceOptions {
  enableMemoryMonitoring?: boolean;
  enableCaching?: boolean;
  enableDataOptimization?: boolean;
  componentId?: string;
  cacheKey?: string;
}

interface PerformanceMetrics {
  memoryUsage: number;
  renderTime: number;
  cacheHitRate: number;
  bundleSize: number;
  loadTime: number;
}

interface PerformanceActions {
  clearCache: () => void;
  optimizeMemory: () => void;
  getMetrics: () => PerformanceMetrics | null;
  cacheComponentState: (state: any) => void;
  getCachedComponentState: () => any | null;
}

export function usePerformanceOptimization(
  options: PerformanceOptions = {}
): PerformanceActions {
  const {
    enableMemoryMonitoring = true,
    enableCaching = true,
    enableDataOptimization = true,
    componentId,
    cacheKey,
  } = options;

  const performanceMonitor = useRef(PerformanceMonitor.getInstance());
  const oracleCache = useRef(OracleCache.getInstance());
  const uiStateCache = useRef(UIStateCache.getInstance());
  const userManager = useRef(UserManager.getInstance());
  const memoryPoolManager = useRef(MemoryPoolManager.getInstance());
  
  const renderStartTime = useRef<number>(0);
  const metricsRef = useRef<PerformanceMetrics | null>(null);

  // Start render time measurement
  useEffect(() => {
    if (enableMemoryMonitoring) {
      renderStartTime.current = performance.now();
    }
  }, [enableMemoryMonitoring]);

  // End render time measurement and update metrics
  useEffect(() => {
    if (enableMemoryMonitoring && renderStartTime.current > 0) {
      const renderTime = performance.now() - renderStartTime.current;
      
      // Update metrics asynchronously to avoid blocking render
      setTimeout(() => {
        updateMetrics(renderTime);
      }, 0);
    }
  });

  // Memory monitoring and cleanup
  useEffect(() => {
    if (!enableMemoryMonitoring) return;

    const monitoringInterval = setInterval(() => {
      const memoryUsage = performanceMonitor.current.monitorMemoryUsage();
      
      // Trigger cleanup if memory usage is high
      if (memoryUsage > 400) { // 400MB threshold
        optimizeMemory();
      }
    }, 30000); // Check every 30 seconds

    return () => clearInterval(monitoringInterval);
  }, [enableMemoryMonitoring]);

  // Cache cleanup
  useEffect(() => {
    if (!enableCaching) return;

    const cacheCleanupInterval = setInterval(() => {
      // Clean up expired cache entries
      const oracleStats = oracleCache.current.getStats();
      const uiStats = uiStateCache.current.getStats();
      
      // If cache hit rate is low, clear some entries
      if (oracleStats.hitRate < 50) {
        oracleCache.current.invalidate('old_');
      }
      
      // Clean up temporary UI state
      if (uiStats.memoryUsage > 20) { // 20MB threshold
        uiStateCache.current.clearTemporaryState();
      }
    }, 300000); // Every 5 minutes

    return () => clearInterval(cacheCleanupInterval);
  }, [enableCaching]);

  // Update performance metrics
  const updateMetrics = useCallback(async (renderTime: number) => {
    try {
      const bundleMetrics = await performanceMonitor.current.monitorBundlePerformance();
      const oracleStats = oracleCache.current.getStats();
      
      metricsRef.current = {
        memoryUsage: bundleMetrics.memoryUsage,
        renderTime,
        cacheHitRate: oracleStats.hitRate,
        bundleSize: bundleMetrics.bundleSize,
        loadTime: bundleMetrics.loadTime,
      };
    } catch (error) {
      console.warn('Failed to update performance metrics:', error);
    }
  }, []);

  // Clear all caches
  const clearCache = useCallback(() => {
    if (enableCaching) {
      oracleCache.current.clear();
      uiStateCache.current.clearTemporaryState();
      
      // Clear memory pools
      memoryPoolManager.current.cleanupAll();
      
      console.log('Performance caches cleared');
    }
  }, [enableCaching]);

  // Optimize memory usage
  const optimizeMemory = useCallback(() => {
    if (enableDataOptimization) {
      // Clear temporary caches
      uiStateCache.current.clearTemporaryState();
      
      // Clean up memory pools
      memoryPoolManager.current.cleanupAll();
      
      // Trigger garbage collection if available
      if (typeof window !== 'undefined' && (window as any).gc) {
        (window as any).gc();
      }
      
      console.log('Memory optimization completed');
    }
  }, [enableDataOptimization]);

  // Get current performance metrics
  const getMetrics = useCallback((): PerformanceMetrics | null => {
    return metricsRef.current;
  }, []);

  // Cache component state
  const cacheComponentState = useCallback((state: any) => {
    if (enableCaching && componentId) {
      uiStateCache.current.cacheComponentState(componentId, state);
    }
  }, [enableCaching, componentId]);

  // Get cached component state
  const getCachedComponentState = useCallback((): any | null => {
    if (enableCaching && componentId) {
      return uiStateCache.current.getComponentState(componentId);
    }
    return null;
  }, [enableCaching, componentId]);

  return {
    clearCache,
    optimizeMemory,
    getMetrics,
    cacheComponentState,
    getCachedComponentState,
  };
}

/**
 * Hook for optimized data fetching with caching
 */
export function useOptimizedDataFetching<T>(
  fetchFn: () => Promise<T>,
  cacheKey: string,
  options: {
    ttl?: number;
    enableCache?: boolean;
    dependencies?: any[];
  } = {}
) {
  const { ttl = 300000, enableCache = true, dependencies = [] } = options;
  const oracleCache = useRef(OracleCache.getInstance());
  const memoryPoolManager = useRef(MemoryPoolManager.getInstance());

  return useMemo(() => {
    const fetchWithCache = async (): Promise<T> => {
      // Try cache first
      if (enableCache) {
        const cached = oracleCache.current.get<T>(cacheKey);
        if (cached) {
          return cached;
        }
      }

      // Fetch data using memory pool for response object
      const response = memoryPoolManager.current.acquireAPIResponse();
      
      try {
        const data = await fetchFn();
        
        // Cache the result
        if (enableCache) {
          oracleCache.current.set(cacheKey, data, ttl);
        }
        
        return data;
      } finally {
        // Return response object to pool
        memoryPoolManager.current.releaseAPIResponse(response);
      }
    };

    return fetchWithCache;
  }, [fetchFn, cacheKey, ttl, enableCache, ...dependencies]);
}

/**
 * Hook for memory-efficient list rendering
 */
export function useVirtualizedList<T>(
  items: T[],
  itemHeight: number,
  containerHeight: number,
  options: {
    overscan?: number;
    enableCaching?: boolean;
  } = {}
) {
  const { overscan = 5, enableCaching = true } = options;
  const uiStateCache = useRef(UIStateCache.getInstance());

  return useMemo(() => {
    const visibleCount = Math.ceil(containerHeight / itemHeight);
    const totalCount = items.length;

    const getVisibleItems = (scrollTop: number) => {
      const startIndex = Math.floor(scrollTop / itemHeight);
      const endIndex = Math.min(startIndex + visibleCount + overscan, totalCount);
      const adjustedStartIndex = Math.max(0, startIndex - overscan);

      const visibleItems = items.slice(adjustedStartIndex, endIndex).map((item, index) => ({
        item,
        index: adjustedStartIndex + index,
        top: (adjustedStartIndex + index) * itemHeight,
      }));

      // Cache visible items if enabled
      if (enableCaching) {
        const cacheKey = `virtualized_list_${startIndex}_${endIndex}`;
        uiStateCache.current.setState(cacheKey, visibleItems, false);
      }

      return {
        visibleItems,
        totalHeight: totalCount * itemHeight,
        startIndex: adjustedStartIndex,
        endIndex,
      };
    };

    return { getVisibleItems, totalCount, itemHeight };
  }, [items, itemHeight, containerHeight, overscan, enableCaching]);
}

/**
 * Hook for debounced performance-optimized callbacks
 */
export function useOptimizedCallback<T extends (...args: any[]) => any>(
  callback: T,
  delay: number,
  dependencies: any[] = []
): T {
  const timeoutRef = useRef<NodeJS.Timeout>();
  const memoryPoolManager = useRef(MemoryPoolManager.getInstance());

  return useCallback(
    ((...args: Parameters<T>) => {
      // Clear previous timeout
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }

      // Use memory pool for argument storage if needed
      const pooledArgs = args.map(arg => {
        if (typeof arg === 'object' && arg !== null) {
          // For complex objects, consider using memory pool
          return arg;
        }
        return arg;
      });

      timeoutRef.current = setTimeout(() => {
        callback(...pooledArgs);
      }, delay);
    }) as T,
    [callback, delay, ...dependencies]
  );
}

export type { PerformanceOptions, PerformanceMetrics, PerformanceActions };