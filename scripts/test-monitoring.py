#!/usr/bin/env python3
"""
Test script to demonstrate monitoring and alerting functionality
"""

import asyncio
import json
import os
import sys
import time
from datetime import datetime

# Add the scripts directory to the path so we can import the monitoring service
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from monitoring_service import MonitoringService, MonitoringConfig, AlertEvent

async def test_monitoring_service():
    """Test the monitoring service functionality"""
    print("🚀 Testing Vault Protocol Monitoring Service")
    print("=" * 60)
    
    # Create test configuration
    config = MonitoringConfig(
        health_check_interval=5,  # Check every 5 seconds for testing
        performance_check_interval=10,
        alert_retry_interval=30,
        cleanup_interval=60,
        
        # Use mock endpoints for testing
        solana_rpc_url="http://localhost:8899",
        api_base_url="http://localhost:8080",
        frontend_url="http://localhost:3000",
        
        # Mock alert channels
        slack_webhook="https://hooks.slack.com/test",
        webhook_url="https://webhook.example.com/alerts",
        
        # Lower thresholds for testing
        cpu_warning_threshold=50.0,
        cpu_critical_threshold=70.0,
        memory_warning_threshold=60.0,
        memory_critical_threshold=80.0,
        response_time_warning_ms=500,
        response_time_critical_ms=1000
    )
    
    # Create monitoring service
    service = MonitoringService(config)
    
    print(f"✅ Created monitoring service with {len(service.monitors)} monitors:")
    for name in service.monitors.keys():
        print(f"   - {name}")
    
    print("\n📊 Testing individual component health checks...")
    
    # Test each monitor individually
    for name, monitor in service.monitors.items():
        try:
            print(f"\n🔍 Checking {name} health...")
            health_status = await monitor.check_health()
            
            print(f"   Status: {health_status.status}")
            print(f"   Response Time: {health_status.response_time_ms}ms")
            print(f"   Error Count: {health_status.error_count}")
            print(f"   Uptime: {health_status.uptime_percentage}%")
            
            if health_status.metrics:
                print(f"   Metrics: {json.dumps(health_status.metrics, indent=4)}")
            
        except Exception as e:
            print(f"   ❌ Error checking {name}: {e}")
    
    print("\n🚨 Testing alert system...")
    
    # Test manual alert
    test_alert = AlertEvent(
        alert_id=f"test_alert_{int(time.time())}",
        component="test",
        severity="high",
        message="This is a test alert to verify the alerting system",
        timestamp=datetime.now(),
        metadata={"test": "true", "source": "manual"}
    )
    
    print(f"📤 Sending test alert: {test_alert.alert_id}")
    success = await service.alert_manager.send_alert(test_alert)
    print(f"   Alert delivery: {'✅ Success' if success else '❌ Failed'}")
    
    # Show alert statistics
    stats = service.alert_manager.delivery_stats
    print(f"\n📈 Alert Statistics:")
    print(f"   Total Sent: {stats['total_sent']}")
    print(f"   Successful: {stats['successful']}")
    print(f"   Failed: {stats['failed']}")
    if stats['total_sent'] > 0:
        success_rate = (stats['successful'] / stats['total_sent']) * 100
        print(f"   Success Rate: {success_rate:.1f}%")
    
    print("\n⏱️  Running monitoring cycle for 30 seconds...")
    
    # Run monitoring for a short period
    start_time = time.time()
    cycle_count = 0
    
    while time.time() - start_time < 30:
        try:
            # Run a single monitoring cycle
            health_statuses = {}
            alerts_generated = 0
            
            for name, monitor in service.monitors.items():
                health_status = await monitor.check_health()
                health_statuses[name] = health_status
                
                # Check for alerts (simplified)
                if health_status.status in ['critical', 'warning']:
                    alert = AlertEvent(
                        alert_id=f"auto_{name}_{int(time.time())}",
                        component=name,
                        severity="medium" if health_status.status == "warning" else "high",
                        message=f"Component {name} status: {health_status.status}",
                        timestamp=datetime.now(),
                        metadata={
                            "response_time_ms": str(health_status.response_time_ms),
                            "error_count": str(health_status.error_count)
                        }
                    )
                    
                    await service.alert_manager.send_alert(alert)
                    alerts_generated += 1
            
            cycle_count += 1
            
            # Print status every 10 seconds
            if cycle_count % 2 == 0:
                healthy_count = sum(1 for status in health_statuses.values() if status.status == "healthy")
                warning_count = sum(1 for status in health_statuses.values() if status.status == "warning")
                critical_count = sum(1 for status in health_statuses.values() if status.status == "critical")
                
                print(f"   Cycle {cycle_count}: ✅ {healthy_count} healthy, ⚠️ {warning_count} warning, ❌ {critical_count} critical")
                if alerts_generated > 0:
                    print(f"   Generated {alerts_generated} alerts this cycle")
            
            await asyncio.sleep(5)  # Wait 5 seconds between cycles
            
        except Exception as e:
            print(f"   ❌ Error in monitoring cycle: {e}")
            break
    
    print(f"\n✅ Completed {cycle_count} monitoring cycles")
    
    # Final statistics
    final_stats = service.alert_manager.delivery_stats
    print(f"\n📊 Final Statistics:")
    print(f"   Total Alerts Sent: {final_stats['total_sent']}")
    print(f"   Successful Deliveries: {final_stats['successful']}")
    print(f"   Failed Deliveries: {final_stats['failed']}")
    print(f"   Recent Alerts: {len(service.alert_manager.alert_history)}")
    
    # Show recent alerts
    if service.alert_manager.alert_history:
        print(f"\n📋 Recent Alerts:")
        for alert in service.alert_manager.alert_history[-5:]:  # Show last 5 alerts
            print(f"   {alert.timestamp.strftime('%H:%M:%S')} - {alert.component} - {alert.severity.upper()} - {alert.message}")
    
    # Show service status
    status = service.get_status()
    print(f"\n🔧 Service Status:")
    print(json.dumps(status, indent=2, default=str))
    
    print("\n🎉 Monitoring service test completed successfully!")

async def test_alert_verification():
    """Test alert verification procedures"""
    print("\n🔍 Testing Alert Verification Procedures")
    print("=" * 50)
    
    config = MonitoringConfig()
    service = MonitoringService(config)
    
    # Test different alert severities
    severities = ["low", "medium", "high", "critical"]
    components = ["oracle", "staking", "treasury", "security"]
    
    print("📤 Testing alert delivery for different severities and components...")
    
    for i, (severity, component) in enumerate(zip(severities, components)):
        alert = AlertEvent(
            alert_id=f"verify_alert_{i}_{int(time.time())}",
            component=component,
            severity=severity,
            message=f"Test {severity} alert for {component} component",
            timestamp=datetime.now(),
            metadata={
                "test_id": str(i),
                "verification": "true",
                "expected_severity": severity
            }
        )
        
        print(f"   Sending {severity.upper()} alert for {component}...")
        success = await service.alert_manager.send_alert(alert)
        print(f"   Result: {'✅ Delivered' if success else '❌ Failed'}")
        
        # Small delay between alerts
        await asyncio.sleep(1)
    
    # Test alert cooldown
    print("\n⏰ Testing alert cooldown mechanism...")
    
    # Send two identical alerts quickly
    duplicate_alert = AlertEvent(
        alert_id=f"cooldown_test_1_{int(time.time())}",
        component="test",
        severity="medium",
        message="Testing cooldown mechanism",
        timestamp=datetime.now(),
        metadata={"cooldown_test": "first"}
    )
    
    print("   Sending first alert...")
    success1 = await service.alert_manager.send_alert(duplicate_alert)
    print(f"   First alert: {'✅ Delivered' if success1 else '❌ Failed'}")
    
    # Send duplicate immediately
    duplicate_alert.alert_id = f"cooldown_test_2_{int(time.time())}"
    duplicate_alert.metadata = {"cooldown_test": "second"}
    
    print("   Sending duplicate alert immediately...")
    success2 = await service.alert_manager.send_alert(duplicate_alert)
    print(f"   Second alert: {'✅ Delivered' if success2 else '❌ Suppressed (expected)'}")
    
    # Show final statistics
    stats = service.alert_manager.delivery_stats
    print(f"\n📈 Verification Statistics:")
    print(f"   Total Alerts: {stats['total_sent']}")
    print(f"   Successful: {stats['successful']}")
    print(f"   Failed: {stats['failed']}")
    print(f"   Success Rate: {(stats['successful'] / max(stats['total_sent'], 1)) * 100:.1f}%")
    
    print("\n✅ Alert verification completed!")

async def main():
    """Main test function"""
    print("🔧 Vault Protocol Monitoring & Alerting System Test")
    print("=" * 70)
    print(f"Started at: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print()
    
    try:
        # Test monitoring service
        await test_monitoring_service()
        
        # Test alert verification
        await test_alert_verification()
        
        print(f"\n🎯 All tests completed successfully at {datetime.now().strftime('%H:%M:%S')}")
        
    except KeyboardInterrupt:
        print("\n⚠️  Test interrupted by user")
    except Exception as e:
        print(f"\n❌ Test failed with error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    # Install required packages if not available
    try:
        import aiohttp
        import psutil
    except ImportError as e:
        print(f"❌ Missing required package: {e}")
        print("Please install required packages:")
        print("pip install aiohttp psutil")
        sys.exit(1)
    
    asyncio.run(main())