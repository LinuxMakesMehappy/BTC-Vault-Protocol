#!/usr/bin/env python3
"""
Vault Protocol Monitoring Service
Continuous monitoring service that runs health checks, performance monitoring,
and alert delivery for all protocol components
"""

import asyncio
import json
import logging
import os
import signal
import sys
import time
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any
import aiohttp
import psutil
from dataclasses import dataclass, asdict
from concurrent.futures import ThreadPoolExecutor
import threading

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('monitoring.log'),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger('vault-monitoring')

@dataclass
class MonitoringConfig:
    """Configuration for monitoring service"""
    health_check_interval: int = 30  # seconds
    performance_check_interval: int = 60  # seconds
    alert_retry_interval: int = 300  # seconds
    cleanup_interval: int = 3600  # seconds
    max_alerts_per_hour: int = 100
    alert_cooldown_minutes: int = 15
    
    # Component endpoints
    solana_rpc_url: str = "http://localhost:8899"
    api_base_url: str = "http://localhost:8080"
    frontend_url: str = "http://localhost:3000"
    
    # Alert channels
    email_endpoint: Optional[str] = None
    slack_webhook: Optional[str] = None
    webhook_url: Optional[str] = None
    
    # Thresholds
    cpu_warning_threshold: float = 70.0
    cpu_critical_threshold: float = 85.0
    memory_warning_threshold: float = 75.0
    memory_critical_threshold: float = 90.0
    response_time_warning_ms: int = 1000
    response_time_critical_ms: int = 3000
    error_rate_warning: float = 1.0
    error_rate_critical: float = 5.0

@dataclass
class HealthStatus:
    """Health status for a component"""
    component: str
    status: str  # healthy, warning, critical, offline
    response_time_ms: int
    error_count: int
    last_check: datetime
    uptime_percentage: float
    metrics: Dict[str, Any]

@dataclass
class AlertEvent:
    """Alert event data structure"""
    alert_id: str
    component: str
    severity: str  # low, medium, high, critical
    message: str
    timestamp: datetime
    metadata: Dict[str, str]

class ComponentMonitor:
    """Base class for component monitoring"""
    
    def __init__(self, name: str, config: MonitoringConfig):
        self.name = name
        self.config = config
        self.last_check = None
        self.error_count = 0
        self.response_times = []
        self.max_response_times = 100
        
    async def check_health(self) -> HealthStatus:
        """Check component health - to be implemented by subclasses"""
        raise NotImplementedError
    
    def calculate_uptime(self) -> float:
        """Calculate uptime percentage based on recent checks"""
        # Simplified calculation - in production this would track historical data
        if self.error_count == 0:
            return 99.9
        elif self.error_count < 5:
            return 95.0
        else:
            return 90.0
    
    def add_response_time(self, response_time_ms: int):
        """Add response time measurement"""
        self.response_times.append(response_time_ms)
        if len(self.response_times) > self.max_response_times:
            self.response_times.pop(0)
    
    def get_avg_response_time(self) -> int:
        """Get average response time"""
        if not self.response_times:
            return 0
        return int(sum(self.response_times) / len(self.response_times))

class SolanaMonitor(ComponentMonitor):
    """Monitor Solana RPC and program health"""
    
    async def check_health(self) -> HealthStatus:
        start_time = time.time()
        status = "healthy"
        metrics = {}
        
        try:
            async with aiohttp.ClientSession() as session:
                # Check RPC health
                rpc_payload = {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "getHealth"
                }
                
                async with session.post(
                    self.config.solana_rpc_url,
                    json=rpc_payload,
                    timeout=aiohttp.ClientTimeout(total=10)
                ) as response:
                    if response.status != 200:
                        status = "critical"
                        self.error_count += 1
                    else:
                        result = await response.json()
                        if "error" in result:
                            status = "warning"
                            self.error_count += 1
                        else:
                            metrics["rpc_status"] = "ok"
                
                # Check slot information
                slot_payload = {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "getSlot"
                }
                
                async with session.post(
                    self.config.solana_rpc_url,
                    json=slot_payload,
                    timeout=aiohttp.ClientTimeout(total=5)
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        if "result" in result:
                            metrics["current_slot"] = result["result"]
                
        except asyncio.TimeoutError:
            status = "critical"
            self.error_count += 1
            logger.error(f"Timeout checking Solana health")
        except Exception as e:
            status = "critical"
            self.error_count += 1
            logger.error(f"Error checking Solana health: {e}")
        
        response_time_ms = int((time.time() - start_time) * 1000)
        self.add_response_time(response_time_ms)
        self.last_check = datetime.now()
        
        return HealthStatus(
            component=self.name,
            status=status,
            response_time_ms=response_time_ms,
            error_count=self.error_count,
            last_check=self.last_check,
            uptime_percentage=self.calculate_uptime(),
            metrics=metrics
        )

class APIMonitor(ComponentMonitor):
    """Monitor API endpoints"""
    
    def __init__(self, name: str, config: MonitoringConfig, endpoint: str):
        super().__init__(name, config)
        self.endpoint = endpoint
    
    async def check_health(self) -> HealthStatus:
        start_time = time.time()
        status = "healthy"
        metrics = {}
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.endpoint}/health",
                    timeout=aiohttp.ClientTimeout(total=10)
                ) as response:
                    if response.status != 200:
                        status = "critical"
                        self.error_count += 1
                    else:
                        try:
                            data = await response.json()
                            metrics.update(data.get("metrics", {}))
                        except:
                            # Health endpoint might return plain text
                            pass
                
        except asyncio.TimeoutError:
            status = "critical"
            self.error_count += 1
            logger.error(f"Timeout checking {self.name} health")
        except Exception as e:
            status = "critical"
            self.error_count += 1
            logger.error(f"Error checking {self.name} health: {e}")
        
        response_time_ms = int((time.time() - start_time) * 1000)
        self.add_response_time(response_time_ms)
        self.last_check = datetime.now()
        
        return HealthStatus(
            component=self.name,
            status=status,
            response_time_ms=response_time_ms,
            error_count=self.error_count,
            last_check=self.last_check,
            uptime_percentage=self.calculate_uptime(),
            metrics=metrics
        )

class SystemMonitor(ComponentMonitor):
    """Monitor system resources"""
    
    async def check_health(self) -> HealthStatus:
        start_time = time.time()
        status = "healthy"
        
        try:
            # Get system metrics
            cpu_percent = psutil.cpu_percent(interval=1)
            memory = psutil.virtual_memory()
            disk = psutil.disk_usage('/')
            network = psutil.net_io_counters()
            
            metrics = {
                "cpu_usage_percent": cpu_percent,
                "memory_usage_percent": memory.percent,
                "memory_used_gb": memory.used / (1024**3),
                "memory_total_gb": memory.total / (1024**3),
                "disk_usage_percent": disk.percent,
                "disk_free_gb": disk.free / (1024**3),
                "network_bytes_sent": network.bytes_sent,
                "network_bytes_recv": network.bytes_recv,
                "active_connections": len(psutil.net_connections())
            }
            
            # Determine status based on thresholds
            if (cpu_percent > self.config.cpu_critical_threshold or 
                memory.percent > self.config.memory_critical_threshold):
                status = "critical"
                self.error_count += 1
            elif (cpu_percent > self.config.cpu_warning_threshold or 
                  memory.percent > self.config.memory_warning_threshold):
                status = "warning"
            
        except Exception as e:
            status = "critical"
            self.error_count += 1
            metrics = {}
            logger.error(f"Error checking system health: {e}")
        
        response_time_ms = int((time.time() - start_time) * 1000)
        self.add_response_time(response_time_ms)
        self.last_check = datetime.now()
        
        return HealthStatus(
            component=self.name,
            status=status,
            response_time_ms=response_time_ms,
            error_count=self.error_count,
            last_check=self.last_check,
            uptime_percentage=self.calculate_uptime(),
            metrics=metrics
        )

class AlertManager:
    """Manage alert delivery and notifications"""
    
    def __init__(self, config: MonitoringConfig):
        self.config = config
        self.alert_history = []
        self.alert_cooldowns = {}
        self.delivery_stats = {
            "total_sent": 0,
            "successful": 0,
            "failed": 0
        }
    
    async def send_alert(self, alert: AlertEvent) -> bool:
        """Send alert through configured channels"""
        # Check cooldown
        cooldown_key = f"{alert.component}_{alert.severity}"
        now = datetime.now()
        
        if cooldown_key in self.alert_cooldowns:
            last_sent = self.alert_cooldowns[cooldown_key]
            if now - last_sent < timedelta(minutes=self.config.alert_cooldown_minutes):
                logger.info(f"Alert {alert.alert_id} suppressed due to cooldown")
                return False
        
        self.alert_cooldowns[cooldown_key] = now
        self.alert_history.append(alert)
        
        # Send through all configured channels
        success = True
        
        if self.config.email_endpoint:
            success &= await self._send_email_alert(alert)
        
        if self.config.slack_webhook:
            success &= await self._send_slack_alert(alert)
        
        if self.config.webhook_url:
            success &= await self._send_webhook_alert(alert)
        
        # Update stats
        self.delivery_stats["total_sent"] += 1
        if success:
            self.delivery_stats["successful"] += 1
        else:
            self.delivery_stats["failed"] += 1
        
        return success
    
    async def _send_email_alert(self, alert: AlertEvent) -> bool:
        """Send email alert (mock implementation)"""
        try:
            logger.info(f"Sending email alert: {alert.alert_id}")
            # In production, this would use an email service like SendGrid
            await asyncio.sleep(0.1)  # Simulate API call
            return True
        except Exception as e:
            logger.error(f"Failed to send email alert: {e}")
            return False
    
    async def _send_slack_alert(self, alert: AlertEvent) -> bool:
        """Send Slack alert"""
        try:
            severity_colors = {
                "critical": "danger",
                "high": "warning",
                "medium": "good",
                "low": "#36a64f"
            }
            
            payload = {
                "attachments": [{
                    "color": severity_colors.get(alert.severity, "good"),
                    "title": f"Vault Protocol Alert - {alert.component}",
                    "text": alert.message,
                    "fields": [
                        {"title": "Severity", "value": alert.severity.upper(), "short": True},
                        {"title": "Component", "value": alert.component, "short": True},
                        {"title": "Time", "value": alert.timestamp.isoformat(), "short": True},
                        {"title": "Alert ID", "value": alert.alert_id, "short": True}
                    ]
                }]
            }
            
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    self.config.slack_webhook,
                    json=payload,
                    timeout=aiohttp.ClientTimeout(total=10)
                ) as response:
                    if response.status == 200:
                        logger.info(f"Sent Slack alert: {alert.alert_id}")
                        return True
                    else:
                        logger.error(f"Slack alert failed with status {response.status}")
                        return False
                        
        except Exception as e:
            logger.error(f"Failed to send Slack alert: {e}")
            return False
    
    async def _send_webhook_alert(self, alert: AlertEvent) -> bool:
        """Send webhook alert"""
        try:
            payload = {
                "alert_id": alert.alert_id,
                "component": alert.component,
                "severity": alert.severity,
                "message": alert.message,
                "timestamp": alert.timestamp.isoformat(),
                "metadata": alert.metadata
            }
            
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    self.config.webhook_url,
                    json=payload,
                    timeout=aiohttp.ClientTimeout(total=10)
                ) as response:
                    if response.status == 200:
                        logger.info(f"Sent webhook alert: {alert.alert_id}")
                        return True
                    else:
                        logger.error(f"Webhook alert failed with status {response.status}")
                        return False
                        
        except Exception as e:
            logger.error(f"Failed to send webhook alert: {e}")
            return False
    
    def cleanup_old_alerts(self, retention_hours: int = 24):
        """Clean up old alerts"""
        cutoff_time = datetime.now() - timedelta(hours=retention_hours)
        self.alert_history = [
            alert for alert in self.alert_history 
            if alert.timestamp > cutoff_time
        ]
        
        # Clean up cooldowns
        cutoff_cooldown = datetime.now() - timedelta(minutes=self.config.alert_cooldown_minutes)
        self.alert_cooldowns = {
            key: timestamp for key, timestamp in self.alert_cooldowns.items()
            if timestamp > cutoff_cooldown
        }

class MonitoringService:
    """Main monitoring service"""
    
    def __init__(self, config: MonitoringConfig):
        self.config = config
        self.monitors = {}
        self.alert_manager = AlertManager(config)
        self.running = False
        self.tasks = []
        
        # Initialize monitors
        self._setup_monitors()
    
    def _setup_monitors(self):
        """Setup component monitors"""
        self.monitors = {
            "solana": SolanaMonitor("solana", self.config),
            "api": APIMonitor("api", self.config, self.config.api_base_url),
            "frontend": APIMonitor("frontend", self.config, self.config.frontend_url),
            "system": SystemMonitor("system", self.config)
        }
    
    async def start(self):
        """Start monitoring service"""
        logger.info("Starting Vault Protocol Monitoring Service")
        self.running = True
        
        # Start monitoring tasks
        self.tasks = [
            asyncio.create_task(self._health_check_loop()),
            asyncio.create_task(self._performance_check_loop()),
            asyncio.create_task(self._cleanup_loop())
        ]
        
        # Wait for all tasks
        try:
            await asyncio.gather(*self.tasks)
        except asyncio.CancelledError:
            logger.info("Monitoring tasks cancelled")
    
    async def stop(self):
        """Stop monitoring service"""
        logger.info("Stopping Vault Protocol Monitoring Service")
        self.running = False
        
        # Cancel all tasks
        for task in self.tasks:
            task.cancel()
        
        # Wait for tasks to complete
        await asyncio.gather(*self.tasks, return_exceptions=True)
    
    async def _health_check_loop(self):
        """Main health check loop"""
        while self.running:
            try:
                logger.debug("Running health checks")
                
                # Check all components
                health_statuses = {}
                for name, monitor in self.monitors.items():
                    try:
                        health_status = await monitor.check_health()
                        health_statuses[name] = health_status
                        
                        # Generate alerts based on health status
                        await self._check_health_alerts(health_status)
                        
                    except Exception as e:
                        logger.error(f"Error checking {name} health: {e}")
                
                # Log overall system status
                critical_components = [
                    name for name, status in health_statuses.items()
                    if status.status == "critical"
                ]
                
                if critical_components:
                    logger.warning(f"Critical components: {', '.join(critical_components)}")
                else:
                    logger.debug("All components healthy")
                
                await asyncio.sleep(self.config.health_check_interval)
                
            except asyncio.CancelledError:
                break
            except Exception as e:
                logger.error(f"Error in health check loop: {e}")
                await asyncio.sleep(5)  # Brief pause before retrying
    
    async def _performance_check_loop(self):
        """Performance monitoring loop"""
        while self.running:
            try:
                logger.debug("Running performance checks")
                
                # Collect performance metrics
                for name, monitor in self.monitors.items():
                    try:
                        # Performance checks are integrated into health checks
                        # This loop can be extended for additional performance monitoring
                        pass
                    except Exception as e:
                        logger.error(f"Error in performance check for {name}: {e}")
                
                await asyncio.sleep(self.config.performance_check_interval)
                
            except asyncio.CancelledError:
                break
            except Exception as e:
                logger.error(f"Error in performance check loop: {e}")
                await asyncio.sleep(5)
    
    async def _cleanup_loop(self):
        """Cleanup old data loop"""
        while self.running:
            try:
                logger.debug("Running cleanup")
                
                # Cleanup old alerts
                self.alert_manager.cleanup_old_alerts()
                
                # Reset error counts periodically
                for monitor in self.monitors.values():
                    if monitor.error_count > 100:  # Reset if too high
                        monitor.error_count = max(0, monitor.error_count - 10)
                
                await asyncio.sleep(self.config.cleanup_interval)
                
            except asyncio.CancelledError:
                break
            except Exception as e:
                logger.error(f"Error in cleanup loop: {e}")
                await asyncio.sleep(60)
    
    async def _check_health_alerts(self, health_status: HealthStatus):
        """Check if health status should trigger alerts"""
        now = datetime.now()
        
        # Critical status alert
        if health_status.status == "critical":
            alert = AlertEvent(
                alert_id=f"health_critical_{health_status.component}_{int(now.timestamp())}",
                component=health_status.component,
                severity="critical",
                message=f"Component {health_status.component} is in critical state",
                timestamp=now,
                metadata={
                    "response_time_ms": str(health_status.response_time_ms),
                    "error_count": str(health_status.error_count),
                    "uptime_percentage": str(health_status.uptime_percentage)
                }
            )
            await self.alert_manager.send_alert(alert)
        
        # High response time alert
        if health_status.response_time_ms > self.config.response_time_critical_ms:
            alert = AlertEvent(
                alert_id=f"response_time_critical_{health_status.component}_{int(now.timestamp())}",
                component=health_status.component,
                severity="high",
                message=f"Component {health_status.component} response time {health_status.response_time_ms}ms exceeds critical threshold",
                timestamp=now,
                metadata={
                    "response_time_ms": str(health_status.response_time_ms),
                    "threshold_ms": str(self.config.response_time_critical_ms)
                }
            )
            await self.alert_manager.send_alert(alert)
        elif health_status.response_time_ms > self.config.response_time_warning_ms:
            alert = AlertEvent(
                alert_id=f"response_time_warning_{health_status.component}_{int(now.timestamp())}",
                component=health_status.component,
                severity="medium",
                message=f"Component {health_status.component} response time {health_status.response_time_ms}ms exceeds warning threshold",
                timestamp=now,
                metadata={
                    "response_time_ms": str(health_status.response_time_ms),
                    "threshold_ms": str(self.config.response_time_warning_ms)
                }
            )
            await self.alert_manager.send_alert(alert)
        
        # System-specific alerts
        if health_status.component == "system" and health_status.metrics:
            await self._check_system_alerts(health_status)
    
    async def _check_system_alerts(self, health_status: HealthStatus):
        """Check system-specific alerts"""
        now = datetime.now()
        metrics = health_status.metrics
        
        # CPU usage alerts
        cpu_usage = metrics.get("cpu_usage_percent", 0)
        if cpu_usage > self.config.cpu_critical_threshold:
            alert = AlertEvent(
                alert_id=f"cpu_critical_{int(now.timestamp())}",
                component="system",
                severity="critical",
                message=f"CPU usage {cpu_usage:.1f}% exceeds critical threshold {self.config.cpu_critical_threshold}%",
                timestamp=now,
                metadata={"cpu_usage": str(cpu_usage), "threshold": str(self.config.cpu_critical_threshold)}
            )
            await self.alert_manager.send_alert(alert)
        elif cpu_usage > self.config.cpu_warning_threshold:
            alert = AlertEvent(
                alert_id=f"cpu_warning_{int(now.timestamp())}",
                component="system",
                severity="medium",
                message=f"CPU usage {cpu_usage:.1f}% exceeds warning threshold {self.config.cpu_warning_threshold}%",
                timestamp=now,
                metadata={"cpu_usage": str(cpu_usage), "threshold": str(self.config.cpu_warning_threshold)}
            )
            await self.alert_manager.send_alert(alert)
        
        # Memory usage alerts
        memory_usage = metrics.get("memory_usage_percent", 0)
        if memory_usage > self.config.memory_critical_threshold:
            alert = AlertEvent(
                alert_id=f"memory_critical_{int(now.timestamp())}",
                component="system",
                severity="critical",
                message=f"Memory usage {memory_usage:.1f}% exceeds critical threshold {self.config.memory_critical_threshold}%",
                timestamp=now,
                metadata={"memory_usage": str(memory_usage), "threshold": str(self.config.memory_critical_threshold)}
            )
            await self.alert_manager.send_alert(alert)
        elif memory_usage > self.config.memory_warning_threshold:
            alert = AlertEvent(
                alert_id=f"memory_warning_{int(now.timestamp())}",
                component="system",
                severity="medium",
                message=f"Memory usage {memory_usage:.1f}% exceeds warning threshold {self.config.memory_warning_threshold}%",
                timestamp=now,
                metadata={"memory_usage": str(memory_usage), "threshold": str(self.config.memory_warning_threshold)}
            )
            await self.alert_manager.send_alert(alert)
    
    def get_status(self) -> Dict[str, Any]:
        """Get current monitoring status"""
        return {
            "running": self.running,
            "monitors": {
                name: {
                    "last_check": monitor.last_check.isoformat() if monitor.last_check else None,
                    "error_count": monitor.error_count,
                    "avg_response_time": monitor.get_avg_response_time()
                }
                for name, monitor in self.monitors.items()
            },
            "alert_stats": self.alert_manager.delivery_stats,
            "recent_alerts": len(self.alert_manager.alert_history)
        }

def load_config() -> MonitoringConfig:
    """Load configuration from environment variables"""
    return MonitoringConfig(
        health_check_interval=int(os.getenv("HEALTH_CHECK_INTERVAL", "30")),
        performance_check_interval=int(os.getenv("PERFORMANCE_CHECK_INTERVAL", "60")),
        alert_retry_interval=int(os.getenv("ALERT_RETRY_INTERVAL", "300")),
        cleanup_interval=int(os.getenv("CLEANUP_INTERVAL", "3600")),
        
        solana_rpc_url=os.getenv("SOLANA_RPC_URL", "http://localhost:8899"),
        api_base_url=os.getenv("API_BASE_URL", "http://localhost:8080"),
        frontend_url=os.getenv("FRONTEND_URL", "http://localhost:3000"),
        
        email_endpoint=os.getenv("ALERT_EMAIL_ENDPOINT"),
        slack_webhook=os.getenv("ALERT_SLACK_WEBHOOK"),
        webhook_url=os.getenv("ALERT_WEBHOOK_URL"),
        
        cpu_warning_threshold=float(os.getenv("CPU_WARNING_THRESHOLD", "70.0")),
        cpu_critical_threshold=float(os.getenv("CPU_CRITICAL_THRESHOLD", "85.0")),
        memory_warning_threshold=float(os.getenv("MEMORY_WARNING_THRESHOLD", "75.0")),
        memory_critical_threshold=float(os.getenv("MEMORY_CRITICAL_THRESHOLD", "90.0")),
        response_time_warning_ms=int(os.getenv("RESPONSE_TIME_WARNING_MS", "1000")),
        response_time_critical_ms=int(os.getenv("RESPONSE_TIME_CRITICAL_MS", "3000"))
    )

async def main():
    """Main entry point"""
    config = load_config()
    service = MonitoringService(config)
    
    # Setup signal handlers
    def signal_handler(signum, frame):
        logger.info(f"Received signal {signum}, shutting down...")
        asyncio.create_task(service.stop())
    
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    
    try:
        await service.start()
    except KeyboardInterrupt:
        logger.info("Received keyboard interrupt, shutting down...")
    finally:
        await service.stop()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Monitoring service stopped")
        sys.exit(0)
    except Exception as e:
        logger.error(f"Fatal error: {e}")
        sys.exit(1)