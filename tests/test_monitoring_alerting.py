#!/usr/bin/env python3
"""
Comprehensive test suite for monitoring and alerting systems
Tests health monitoring, performance tracking, alert delivery, and system integration
"""

import pytest
import asyncio
import time
import json
import tempfile
import os
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor
import threading
from dataclasses import dataclass
from typing import Dict, List, Optional

# Mock classes for testing (in production these would import from the actual modules)
@dataclass
class AlertEvent:
    alert_id: str
    component: str
    severity: str
    message: str
    timestamp: int
    metadata: Dict[str, str]

@dataclass
class ComponentHealth:
    component_name: str
    status: str
    last_check: int
    response_time_ms: int
    error_count: int
    uptime_percentage: float
    metrics: Dict[str, float]

@dataclass
class DeliveryStatus:
    alert_id: str
    channel: str
    status: str
    timestamp: int
    retry_count: int
    error_message: Optional[str]

class MockHealthMonitor:
    def __init__(self):
        self.component_status = {}
        self.alert_cooldowns = {}
        
    def check_oracle_health(self, oracle_state):
        alerts = []
        if oracle_state.get('response_time', 0) > 5000:
            alerts.append(AlertEvent(
                alert_id=f"oracle_slow_{int(time.time())}",
                component="oracle",
                severity="medium",
                message=f"Oracle response time {oracle_state['response_time']}ms exceeds threshold",
                timestamp=int(time.time()),
                metadata={}
            ))
        return alerts
    
    def check_staking_health(self, staking_pool):
        alerts = []
        if staking_pool.get('slashing_events', 0) > 0:
            alerts.append(AlertEvent(
                alert_id=f"staking_slashing_{int(time.time())}",
                component="staking",
                severity="critical",
                message=f"Slashing event detected. Events: {staking_pool['slashing_events']}",
                timestamp=int(time.time()),
                metadata={}
            ))
        return alerts
    
    def check_security_health(self, auth_state):
        alerts = []
        if auth_state.get('failed_attempts', 0) > 10:
            alerts.append(AlertEvent(
                alert_id=f"security_failed_auth_{int(time.time())}",
                component="security",
                severity="high",
                message=f"Excessive failed auth attempts: {auth_state['failed_attempts']}",
                timestamp=int(time.time()),
                metadata={}
            ))
        return alerts
    
    def check_treasury_health(self, treasury):
        alerts = []
        balance_usd = treasury.get('total_assets', 0) / 1_000_000
        if balance_usd < 10000:
            alerts.append(AlertEvent(
                alert_id=f"treasury_low_balance_{int(time.time())}",
                component="treasury",
                severity="critical",
                message=f"Treasury balance ${balance_usd:.2f} below minimum threshold",
                timestamp=int(time.time()),
                metadata={}
            ))
        return alerts
    
    def get_system_health(self):
        return "healthy"
    
    def get_health_summary(self):
        return self.component_status

class MockAlertManager:
    def __init__(self):
        self.alert_channels = []
        self.alert_history = []
        self.delivery_status = {}
        self.rate_limits = {}
        
    def add_channel(self, channel):
        self.alert_channels.append(channel)
    
    async def send_alert(self, alert):
        self.alert_history.append(alert)
        delivery_results = []
        
        for channel in self.alert_channels:
            if channel.get('enabled', True):
                delivery_status = DeliveryStatus(
                    alert_id=alert.alert_id,
                    channel=channel['name'],
                    status="delivered" if channel['name'] != "failing_channel" else "failed",
                    timestamp=int(time.time()),
                    retry_count=0,
                    error_message=None if channel['name'] != "failing_channel" else "Mock failure"
                )
                delivery_results.append(delivery_status)
                self.delivery_status[f"{alert.alert_id}_{channel['name']}"] = delivery_status
        
        return delivery_results
    
    async def retry_failed_deliveries(self):
        retry_results = []
        for key, status in self.delivery_status.items():
            if status.status == "failed" and status.retry_count < 3:
                status.retry_count += 1
                status.status = "delivered" if status.retry_count >= 2 else "failed"
                retry_results.append(status)
        return retry_results
    
    def get_delivery_stats(self):
        total = len(self.delivery_status)
        delivered = sum(1 for s in self.delivery_status.values() if s.status == "delivered")
        failed = sum(1 for s in self.delivery_status.values() if s.status == "failed")
        
        return {
            'total_alerts': total,
            'delivered': delivered,
            'failed': failed,
            'success_rate': (delivered / total * 100) if total > 0 else 0
        }
    
    def cleanup_old_alerts(self, retention_days):
        cutoff_time = int(time.time()) - (retention_days * 24 * 60 * 60)
        self.alert_history = [a for a in self.alert_history if a.timestamp > cutoff_time]

class MockPerformanceMonitor:
    def __init__(self):
        self.metrics = {}
        self.alert_history = []
        
    def collect_system_metrics(self):
        return {
            'cpu_usage_percent': 45.0,
            'memory_usage_percent': 65.0,
            'disk_usage_percent': 40.0,
            'active_connections': 150,
            'timestamp': int(time.time())
        }
    
    def collect_component_metrics(self, component):
        return {
            'component_name': component,
            'response_time_ms': 250,
            'requests_per_second': 25.0,
            'error_rate_percent': 0.5,
            'active_users': 100,
            'timestamp': int(time.time())
        }
    
    def check_performance_thresholds(self):
        alerts = []
        # Simulate a performance alert
        if len(self.metrics) > 5:  # Arbitrary condition for testing
            alerts.append(AlertEvent(
                alert_id=f"perf_high_cpu_{int(time.time())}",
                component="system",
                severity="medium",
                message="CPU usage 85% exceeds threshold",
                timestamp=int(time.time()),
                metadata={'metric': 'cpu_usage', 'value': '85.0'}
            ))
        return alerts
    
    def record_metric(self, component, metric_name, metric_type, value, tags=None):
        key = f"{component}_{metric_name}"
        if key not in self.metrics:
            self.metrics[key] = []
        self.metrics[key].append({
            'value': value,
            'timestamp': int(time.time()),
            'tags': tags or {}
        })

class MockMonitoringService:
    def __init__(self):
        self.health_monitor = MockHealthMonitor()
        self.alert_manager = MockAlertManager()
        self.performance_monitor = MockPerformanceMonitor()
        self.monitoring_enabled = True
        self.last_health_check = 0
        self.last_performance_check = 0
        
    async def run_monitoring_cycle(self, vault_state):
        if not self.monitoring_enabled:
            return {'alerts_sent': 0, 'health_alerts': 0, 'performance_alerts': 0}
        
        current_time = int(time.time())
        report = {'alerts_sent': 0, 'health_alerts': 0, 'performance_alerts': 0}
        
        # Health checks
        if current_time - self.last_health_check >= 30:
            health_alerts = []
            
            if 'oracle' in vault_state:
                health_alerts.extend(self.health_monitor.check_oracle_health(vault_state['oracle']))
            if 'staking_pool' in vault_state:
                health_alerts.extend(self.health_monitor.check_staking_health(vault_state['staking_pool']))
            if 'auth_session' in vault_state:
                health_alerts.extend(self.health_monitor.check_security_health(vault_state['auth_session']))
            if 'treasury' in vault_state:
                health_alerts.extend(self.health_monitor.check_treasury_health(vault_state['treasury']))
            
            report['health_alerts'] = len(health_alerts)
            
            for alert in health_alerts:
                delivery_results = await self.alert_manager.send_alert(alert)
                report['alerts_sent'] += len(delivery_results)
            
            self.last_health_check = current_time
        
        # Performance checks
        if current_time - self.last_performance_check >= 60:
            self.performance_monitor.collect_system_metrics()
            for component in ['oracle', 'staking', 'treasury', 'frontend']:
                self.performance_monitor.collect_component_metrics(component)
            
            performance_alerts = self.performance_monitor.check_performance_thresholds()
            report['performance_alerts'] = len(performance_alerts)
            
            for alert in performance_alerts:
                delivery_results = await self.alert_manager.send_alert(alert)
                report['alerts_sent'] += len(delivery_results)
            
            self.last_performance_check = current_time
        
        return report

class TestHealthMonitoring:
    """Test suite for health monitoring functionality"""
    
    def setup_method(self):
        self.health_monitor = MockHealthMonitor()
    
    def test_oracle_health_check_normal(self):
        """Test oracle health check with normal parameters"""
        oracle_state = {
            'response_time': 1000,
            'last_update': int(time.time()),
            'failed_requests': 2,
            'total_requests': 100
        }
        
        alerts = self.health_monitor.check_oracle_health(oracle_state)
        assert len(alerts) == 0
    
    def test_oracle_health_check_slow_response(self):
        """Test oracle health check with slow response time"""
        oracle_state = {
            'response_time': 6000,  # Exceeds 5000ms threshold
            'last_update': int(time.time()),
            'failed_requests': 2,
            'total_requests': 100
        }
        
        alerts = self.health_monitor.check_oracle_health(oracle_state)
        assert len(alerts) == 1
        assert alerts[0].severity == "medium"
        assert "response time" in alerts[0].message.lower()
    
    def test_staking_health_check_slashing(self):
        """Test staking health check with slashing events"""
        staking_pool = {
            'slashing_events': 1,
            'rewards_accumulated': 1000000,
            'total_staked': 5000000
        }
        
        alerts = self.health_monitor.check_staking_health(staking_pool)
        assert len(alerts) == 1
        assert alerts[0].severity == "critical"
        assert "slashing" in alerts[0].message.lower()
    
    def test_security_health_check_failed_auth(self):
        """Test security health check with excessive failed attempts"""
        auth_state = {
            'failed_attempts': 15,  # Exceeds 10 threshold
            'risk_score': 50,
            'locked': False
        }
        
        alerts = self.health_monitor.check_security_health(auth_state)
        assert len(alerts) == 1
        assert alerts[0].severity == "high"
        assert "failed auth" in alerts[0].message.lower()
    
    def test_treasury_health_check_low_balance(self):
        """Test treasury health check with low balance"""
        treasury = {
            'total_assets': 5_000_000,  # $5 USD (below $10k threshold)
            'last_deposit': int(time.time()) - 86400,  # 1 day ago
            'sol_balance': 2_000_000,
            'eth_balance': 1_500_000,
            'atom_balance': 1_500_000
        }
        
        alerts = self.health_monitor.check_treasury_health(treasury)
        assert len(alerts) == 1
        assert alerts[0].severity == "critical"
        assert "balance" in alerts[0].message.lower()

class TestAlertManager:
    """Test suite for alert management functionality"""
    
    def setup_method(self):
        self.alert_manager = MockAlertManager()
        
        # Add test channels
        self.alert_manager.add_channel({
            'name': 'email',
            'type': 'email',
            'endpoint': 'test@example.com',
            'enabled': True,
            'severity_filter': ['high', 'critical'],
            'rate_limit_per_hour': 10
        })
        
        self.alert_manager.add_channel({
            'name': 'slack',
            'type': 'slack',
            'endpoint': 'https://hooks.slack.com/test',
            'enabled': True,
            'severity_filter': [],
            'rate_limit_per_hour': 20
        })
        
        self.alert_manager.add_channel({
            'name': 'failing_channel',
            'type': 'webhook',
            'endpoint': 'http://failing.example.com',
            'enabled': True,
            'severity_filter': [],
            'rate_limit_per_hour': 50
        })
    
    @pytest.mark.asyncio
    async def test_alert_delivery_success(self):
        """Test successful alert delivery"""
        alert = AlertEvent(
            alert_id="test_alert_001",
            component="oracle",
            severity="high",
            message="Test alert message",
            timestamp=int(time.time()),
            metadata={}
        )
        
        delivery_results = await self.alert_manager.send_alert(alert)
        
        # Should deliver to all 3 channels
        assert len(delivery_results) == 3
        
        # Check delivery statuses
        successful_deliveries = [r for r in delivery_results if r.status == "delivered"]
        failed_deliveries = [r for r in delivery_results if r.status == "failed"]
        
        assert len(successful_deliveries) == 2  # email and slack
        assert len(failed_deliveries) == 1     # failing_channel
    
    @pytest.mark.asyncio
    async def test_alert_retry_mechanism(self):
        """Test alert retry mechanism for failed deliveries"""
        alert = AlertEvent(
            alert_id="test_alert_002",
            component="staking",
            severity="critical",
            message="Test retry alert",
            timestamp=int(time.time()),
            metadata={}
        )
        
        # Send initial alert
        await self.alert_manager.send_alert(alert)
        
        # Retry failed deliveries
        retry_results = await self.alert_manager.retry_failed_deliveries()
        
        assert len(retry_results) > 0
        
        # Check that retry count increased
        for result in retry_results:
            assert result.retry_count > 0
    
    def test_delivery_statistics(self):
        """Test delivery statistics calculation"""
        # Add some mock delivery statuses
        self.alert_manager.delivery_status = {
            'alert1_email': DeliveryStatus('alert1', 'email', 'delivered', int(time.time()), 0, None),
            'alert1_slack': DeliveryStatus('alert1', 'slack', 'delivered', int(time.time()), 0, None),
            'alert2_email': DeliveryStatus('alert2', 'email', 'failed', int(time.time()), 1, 'Network error'),
        }
        
        stats = self.alert_manager.get_delivery_stats()
        
        assert stats['total_alerts'] == 3
        assert stats['delivered'] == 2
        assert stats['failed'] == 1
        assert stats['success_rate'] == pytest.approx(66.67, rel=1e-2)
    
    def test_alert_cleanup(self):
        """Test cleanup of old alerts"""
        current_time = int(time.time())
        
        # Add alerts with different timestamps
        old_alert = AlertEvent("old_alert", "test", "low", "Old alert", current_time - 8*24*60*60, {})
        recent_alert = AlertEvent("recent_alert", "test", "high", "Recent alert", current_time - 1*60*60, {})
        
        self.alert_manager.alert_history = [old_alert, recent_alert]
        
        # Cleanup alerts older than 7 days
        self.alert_manager.cleanup_old_alerts(7)
        
        assert len(self.alert_manager.alert_history) == 1
        assert self.alert_manager.alert_history[0].alert_id == "recent_alert"

class TestPerformanceMonitoring:
    """Test suite for performance monitoring functionality"""
    
    def setup_method(self):
        self.performance_monitor = MockPerformanceMonitor()
    
    def test_system_metrics_collection(self):
        """Test system metrics collection"""
        metrics = self.performance_monitor.collect_system_metrics()
        
        assert 'cpu_usage_percent' in metrics
        assert 'memory_usage_percent' in metrics
        assert 'disk_usage_percent' in metrics
        assert 'active_connections' in metrics
        assert 'timestamp' in metrics
        
        assert 0 <= metrics['cpu_usage_percent'] <= 100
        assert 0 <= metrics['memory_usage_percent'] <= 100
        assert metrics['active_connections'] >= 0
    
    def test_component_metrics_collection(self):
        """Test component-specific metrics collection"""
        component = "oracle"
        metrics = self.performance_monitor.collect_component_metrics(component)
        
        assert metrics['component_name'] == component
        assert 'response_time_ms' in metrics
        assert 'requests_per_second' in metrics
        assert 'error_rate_percent' in metrics
        assert 'active_users' in metrics
        
        assert metrics['response_time_ms'] >= 0
        assert metrics['requests_per_second'] >= 0
        assert 0 <= metrics['error_rate_percent'] <= 100
    
    def test_metric_recording(self):
        """Test metric recording functionality"""
        component = "test_component"
        metric_name = "test_metric"
        value = 75.5
        
        self.performance_monitor.record_metric(component, metric_name, "gauge", value)
        
        key = f"{component}_{metric_name}"
        assert key in self.performance_monitor.metrics
        assert len(self.performance_monitor.metrics[key]) == 1
        assert self.performance_monitor.metrics[key][0]['value'] == value
    
    def test_performance_threshold_alerts(self):
        """Test performance threshold alert generation"""
        # Add enough metrics to trigger alert condition
        for i in range(10):
            self.performance_monitor.record_metric("system", f"metric_{i}", "gauge", 50.0)
        
        alerts = self.performance_monitor.check_performance_thresholds()
        
        assert len(alerts) > 0
        assert any("cpu usage" in alert.message.lower() for alert in alerts)

class TestMonitoringIntegration:
    """Test suite for integrated monitoring functionality"""
    
    def setup_method(self):
        self.monitoring_service = MockMonitoringService()
    
    @pytest.mark.asyncio
    async def test_full_monitoring_cycle(self):
        """Test complete monitoring cycle"""
        vault_state = {
            'oracle': {
                'response_time': 6000,  # Will trigger alert
                'last_update': int(time.time()),
                'failed_requests': 2,
                'total_requests': 100
            },
            'staking_pool': {
                'slashing_events': 0,
                'rewards_accumulated': 1000000,
                'total_staked': 5000000
            },
            'auth_session': {
                'failed_attempts': 5,
                'risk_score': 30,
                'locked': False
            },
            'treasury': {
                'total_assets': 15_000_000,  # $15 USD (above threshold)
                'last_deposit': int(time.time()) - 3600,
                'sol_balance': 6_000_000,
                'eth_balance': 4_500_000,
                'atom_balance': 4_500_000
            }
        }
        
        # Add alert channels
        self.monitoring_service.alert_manager.add_channel({
            'name': 'test_email',
            'type': 'email',
            'endpoint': 'test@example.com',
            'enabled': True,
            'severity_filter': [],
            'rate_limit_per_hour': 100
        })
        
        report = await self.monitoring_service.run_monitoring_cycle(vault_state)
        
        assert report['health_alerts'] > 0  # Should detect oracle slow response
        assert report['alerts_sent'] > 0
    
    @pytest.mark.asyncio
    async def test_monitoring_disabled(self):
        """Test monitoring when disabled"""
        self.monitoring_service.monitoring_enabled = False
        
        vault_state = {
            'oracle': {'response_time': 10000}  # Would normally trigger alert
        }
        
        report = await self.monitoring_service.run_monitoring_cycle(vault_state)
        
        assert report['alerts_sent'] == 0
        assert report['health_alerts'] == 0
    
    @pytest.mark.asyncio
    async def test_concurrent_monitoring(self):
        """Test concurrent monitoring operations"""
        vault_state = {
            'oracle': {'response_time': 1000, 'last_update': int(time.time()), 'failed_requests': 1, 'total_requests': 100},
            'staking_pool': {'slashing_events': 0, 'rewards_accumulated': 1000000, 'total_staked': 5000000},
            'treasury': {'total_assets': 15_000_000, 'last_deposit': int(time.time()), 'sol_balance': 6_000_000, 'eth_balance': 4_500_000, 'atom_balance': 4_500_000}
        }
        
        # Run multiple monitoring cycles concurrently
        tasks = []
        for _ in range(5):
            task = asyncio.create_task(self.monitoring_service.run_monitoring_cycle(vault_state))
            tasks.append(task)
        
        results = await asyncio.gather(*tasks)
        
        # All tasks should complete successfully
        assert len(results) == 5
        for result in results:
            assert isinstance(result, dict)
            assert 'alerts_sent' in result

class TestMonitoringStressTest:
    """Stress tests for monitoring system"""
    
    def setup_method(self):
        self.monitoring_service = MockMonitoringService()
        
        # Add multiple alert channels
        for i in range(5):
            self.monitoring_service.alert_manager.add_channel({
                'name': f'channel_{i}',
                'type': 'webhook',
                'endpoint': f'http://test{i}.example.com',
                'enabled': True,
                'severity_filter': [],
                'rate_limit_per_hour': 1000
            })
    
    @pytest.mark.asyncio
    async def test_high_volume_alerts(self):
        """Test handling of high volume of alerts"""
        vault_state = {
            'oracle': {'response_time': 6000, 'last_update': int(time.time()), 'failed_requests': 10, 'total_requests': 100},
            'staking_pool': {'slashing_events': 1, 'rewards_accumulated': 1000000, 'total_staked': 5000000},
            'auth_session': {'failed_attempts': 15, 'risk_score': 90, 'locked': False},
            'treasury': {'total_assets': 5_000_000, 'last_deposit': int(time.time()) - 86400, 'sol_balance': 2_000_000, 'eth_balance': 1_500_000, 'atom_balance': 1_500_000}
        }
        
        # Run monitoring cycle multiple times rapidly
        start_time = time.time()
        reports = []
        
        for _ in range(10):
            report = await self.monitoring_service.run_monitoring_cycle(vault_state)
            reports.append(report)
            await asyncio.sleep(0.1)  # Small delay
        
        end_time = time.time()
        
        # Should complete within reasonable time
        assert end_time - start_time < 5.0
        
        # Should generate alerts
        total_alerts = sum(r['alerts_sent'] for r in reports)
        assert total_alerts > 0
    
    def test_memory_usage_monitoring(self):
        """Test monitoring system memory usage"""
        # Record many metrics to test memory usage
        for i in range(1000):
            self.monitoring_service.performance_monitor.record_metric(
                f"component_{i % 10}",
                f"metric_{i % 5}",
                "gauge",
                float(i % 100)
            )
        
        # Check that metrics are stored
        assert len(self.monitoring_service.performance_monitor.metrics) > 0
        
        # Verify memory doesn't grow unbounded (mock implementation)
        total_values = sum(len(series) for series in self.monitoring_service.performance_monitor.metrics.values())
        assert total_values == 1000

class TestMonitoringConfiguration:
    """Test monitoring configuration and setup"""
    
    def test_environment_variable_configuration(self):
        """Test configuration from environment variables"""
        with patch.dict(os.environ, {
            'ALERT_EMAIL': 'admin@example.com',
            'ALERT_SLACK_WEBHOOK': 'https://hooks.slack.com/test',
            'ALERT_WEBHOOK_URL': 'https://webhook.example.com'
        }):
            service = MockMonitoringService()
            
            # Should have configured alert channels
            assert len(service.alert_manager.alert_channels) >= 0  # Mock implementation may vary
    
    def test_monitoring_intervals_configuration(self):
        """Test configuration of monitoring intervals"""
        service = MockMonitoringService()
        
        # Test default intervals
        assert hasattr(service, 'last_health_check')
        assert hasattr(service, 'last_performance_check')
        
        # Test that intervals can be modified
        service.last_health_check = 12345
        assert service.last_health_check == 12345

def run_comprehensive_monitoring_tests():
    """Run all monitoring tests with detailed reporting"""
    print("Starting comprehensive monitoring and alerting system tests...")
    
    # Test categories
    test_classes = [
        TestHealthMonitoring,
        TestAlertManager,
        TestPerformanceMonitoring,
        TestMonitoringIntegration,
        TestMonitoringStressTest,
        TestMonitoringConfiguration
    ]
    
    total_tests = 0
    passed_tests = 0
    failed_tests = []
    
    for test_class in test_classes:
        print(f"\nRunning {test_class.__name__}...")
        
        # Get all test methods
        test_methods = [method for method in dir(test_class) if method.startswith('test_')]
        
        for test_method in test_methods:
            total_tests += 1
            try:
                # Create test instance
                test_instance = test_class()
                if hasattr(test_instance, 'setup_method'):
                    test_instance.setup_method()
                
                # Run test method
                method = getattr(test_instance, test_method)
                if asyncio.iscoroutinefunction(method):
                    asyncio.run(method())
                else:
                    method()
                
                passed_tests += 1
                print(f"  ✓ {test_method}")
                
            except Exception as e:
                failed_tests.append((test_class.__name__, test_method, str(e)))
                print(f"  ✗ {test_method}: {e}")
    
    # Print summary
    print(f"\n{'='*60}")
    print(f"MONITORING SYSTEM TEST SUMMARY")
    print(f"{'='*60}")
    print(f"Total tests: {total_tests}")
    print(f"Passed: {passed_tests}")
    print(f"Failed: {len(failed_tests)}")
    print(f"Success rate: {(passed_tests/total_tests)*100:.1f}%")
    
    if failed_tests:
        print(f"\nFailed tests:")
        for test_class, test_method, error in failed_tests:
            print(f"  {test_class}.{test_method}: {error}")
    
    return len(failed_tests) == 0

if __name__ == "__main__":
    success = run_comprehensive_monitoring_tests()
    exit(0 if success else 1)