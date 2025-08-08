# Vault Protocol Monitoring and Alerting System

## Overview

The Vault Protocol monitoring and alerting system provides comprehensive real-time monitoring of all protocol components, including oracle services, staking mechanisms, treasury management, security systems, and frontend/backend performance. The system automatically detects issues, generates alerts, and delivers notifications through multiple channels.

## Architecture

### Core Components

1. **Health Monitor** (`programs/vault/src/monitoring/health_monitor.rs`)
   - Monitors component health status
   - Tracks response times and error rates
   - Calculates uptime percentages
   - Generates health-based alerts

2. **Alert Manager** (`programs/vault/src/monitoring/alert_manager.rs`)
   - Manages alert delivery through multiple channels
   - Implements rate limiting and cooldown mechanisms
   - Tracks delivery statistics and retry logic
   - Supports email, Slack, webhooks, SMS, Discord, and PagerDuty

3. **Performance Monitor** (`programs/vault/src/monitoring/performance_monitor.rs`)
   - Collects system and component performance metrics
   - Monitors CPU, memory, disk, and network usage
   - Tracks application-specific metrics (response times, throughput, error rates)
   - Generates performance threshold alerts

4. **Monitoring Service** (`programs/vault/src/monitoring/mod.rs`)
   - Orchestrates all monitoring components
   - Runs periodic health and performance checks
   - Manages alert lifecycle and cleanup
   - Provides unified monitoring status

### Monitoring Dashboard

The frontend monitoring dashboard (`frontend/src/components/MonitoringDashboard.tsx`) provides:
- Real-time system health overview
- Component status visualization
- Recent alerts and notifications
- Performance metrics charts
- Configuration management

## Configuration

### Environment Variables

```bash
# Monitoring intervals (seconds)
HEALTH_CHECK_INTERVAL=30
PERFORMANCE_CHECK_INTERVAL=60
ALERT_RETRY_INTERVAL=300
CLEANUP_INTERVAL=3600

# Service endpoints
SOLANA_RPC_URL=http://localhost:8899
API_BASE_URL=http://localhost:8080
FRONTEND_URL=http://localhost:3000

# Alert channels
ALERT_EMAIL_ENDPOINT=admin@example.com
ALERT_SLACK_WEBHOOK=https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK
ALERT_WEBHOOK_URL=https://your-webhook-endpoint.com/alerts

# Performance thresholds
CPU_WARNING_THRESHOLD=70.0
CPU_CRITICAL_THRESHOLD=85.0
MEMORY_WARNING_THRESHOLD=75.0
MEMORY_CRITICAL_THRESHOLD=90.0
RESPONSE_TIME_WARNING_MS=1000
RESPONSE_TIME_CRITICAL_MS=3000
```

### Python Configuration

The monitoring configuration is managed through `config/monitoring.py`:

```python
from config.monitoring import monitoring_config

# Access thresholds
oracle_thresholds = monitoring_config.get_component_thresholds(ComponentType.ORACLE)
staking_thresholds = monitoring_config.get_component_thresholds(ComponentType.STAKING)

# Alert templates
alert_template = ALERT_TEMPLATES['oracle_failure']
```

## Monitored Components

### 1. Oracle Health Monitoring

**Metrics Tracked:**
- Response time (threshold: 5000ms)
- Data freshness (threshold: 2 minutes)
- Failure rate (threshold: 5%)
- ECDSA proof validation success rate

**Alert Conditions:**
- Oracle response time exceeds 5 seconds
- Oracle data is stale (>2 minutes old)
- Oracle failure rate exceeds 5%
- ECDSA proof validation failures

### 2. Staking System Monitoring

**Metrics Tracked:**
- Validator uptime percentage
- Slashing events
- Reward variance from expected
- Cross-chain communication status

**Alert Conditions:**
- Validator slashing events (immediate critical alert)
- Validator uptime below 95%
- Reward variance exceeds 10%
- Cross-chain communication failures

### 3. Security Monitoring

**Metrics Tracked:**
- Failed authentication attempts
- Risk score assessments
- Account lockout events
- Multisig transaction timeouts

**Alert Conditions:**
- Excessive failed authentication attempts (>10/hour)
- High risk sessions (risk score >80)
- Account lockouts due to security concerns
- Multisig transactions pending >24 hours

### 4. Treasury Monitoring

**Metrics Tracked:**
- Treasury balance (minimum $10,000 threshold)
- Deposit schedule adherence
- Asset allocation variance
- Rebalancing operations

**Alert Conditions:**
- Treasury balance below minimum threshold
- Deposit delays (>2 hours from schedule)
- Asset allocation variance >5%
- Failed rebalancing operations

### 5. System Performance Monitoring

**Metrics Tracked:**
- CPU usage percentage
- Memory usage percentage
- Disk usage percentage
- Network I/O statistics
- Active connections

**Alert Conditions:**
- CPU usage >85% (critical) or >70% (warning)
- Memory usage >90% (critical) or >75% (warning)
- Disk usage >95%
- Network connectivity issues

### 6. Frontend/Backend Monitoring

**Metrics Tracked:**
- Response times
- Error rates
- Active user sessions
- API request throughput
- Cache hit rates

**Alert Conditions:**
- Response times >3000ms (critical) or >1000ms (warning)
- Error rates >5% (critical) or >1% (warning)
- API throughput below minimum threshold
- Cache hit rate below 80%

## Alert Channels

### 1. Email Alerts
- **Use Case:** Critical alerts and daily summaries
- **Configuration:** Set `ALERT_EMAIL_ENDPOINT` environment variable
- **Format:** HTML email with alert details and metadata

### 2. Slack Integration
- **Use Case:** Real-time team notifications
- **Configuration:** Set `ALERT_SLACK_WEBHOOK` environment variable
- **Format:** Rich attachments with color coding by severity

### 3. Webhook Notifications
- **Use Case:** Integration with external monitoring systems
- **Configuration:** Set `ALERT_WEBHOOK_URL` environment variable
- **Format:** JSON payload with full alert data

### 4. SMS Alerts (Critical Only)
- **Use Case:** Critical alerts requiring immediate attention
- **Configuration:** Set `ALERT_SMS_ENDPOINT` environment variable
- **Format:** Concise text message with essential information

### 5. Discord Integration
- **Use Case:** Community and development team notifications
- **Configuration:** Discord webhook URL
- **Format:** Embedded messages with severity color coding

### 6. PagerDuty Integration
- **Use Case:** Incident management and escalation
- **Configuration:** PagerDuty integration key
- **Format:** Structured incident creation with metadata

## Alert Severity Levels

### Critical
- System components offline or failing
- Security breaches or compromise attempts
- Treasury balance critically low
- Data corruption or loss events

### High
- Performance degradation affecting users
- Failed authentication spike
- Validator slashing events
- Cross-chain communication failures

### Medium
- Performance warnings
- Configuration drift
- Non-critical service degradation
- Scheduled maintenance notifications

### Low
- Informational messages
- Successful operations
- System status updates
- Routine maintenance completion

## Rate Limiting and Cooldowns

### Alert Rate Limiting
- Maximum alerts per channel per hour
- Configurable per channel type
- Prevents alert spam during incidents

### Cooldown Mechanisms
- 15-minute cooldown for duplicate alerts
- Component-specific cooldown periods
- Severity-based cooldown adjustments

### Retry Logic
- Failed alert deliveries are retried up to 3 times
- Exponential backoff between retries
- Failed deliveries tracked for analysis

## Running the Monitoring System

### 1. Standalone Monitoring Service

```bash
# Install dependencies
pip install aiohttp psutil

# Run monitoring service
python scripts/monitoring-service.py
```

### 2. Testing the System

```bash
# Run comprehensive tests
python tests/test_monitoring_alerting.py

# Run interactive test
python scripts/test-monitoring.py
```

### 3. Integration with Main Application

```rust
use crate::monitoring::{MonitoringService, VaultState};

// Initialize monitoring
let mut monitoring_service = MonitoringService::new();

// Create vault state for monitoring
let vault_state = VaultState {
    oracle: Some(oracle_account),
    staking_pool: Some(staking_pool_account),
    auth_session: Some(auth_session_account),
    treasury: Some(treasury_account),
};

// Run monitoring cycle
let report = monitoring_service.run_monitoring_cycle(&vault_state).await?;
```

## Monitoring Dashboard Usage

### Accessing the Dashboard

1. Navigate to `/monitoring` in the frontend application
2. View real-time system health overview
3. Monitor component status and metrics
4. Review recent alerts and notifications

### Dashboard Features

- **System Overview:** High-level health status and statistics
- **Component Health:** Detailed status for each monitored component
- **Recent Alerts:** Timeline of recent alerts with severity indicators
- **Performance Metrics:** Real-time charts of system performance
- **Configuration:** Monitoring settings and alert channel status

### Auto-Refresh

- Dashboard automatically refreshes every 30 seconds
- Manual refresh available via button
- Auto-refresh can be toggled on/off

## Troubleshooting

### Common Issues

1. **Monitoring Service Won't Start**
   - Check Python dependencies: `pip install aiohttp psutil`
   - Verify configuration environment variables
   - Check log files for detailed error messages

2. **Alerts Not Being Delivered**
   - Verify alert channel configuration
   - Check network connectivity to alert endpoints
   - Review rate limiting and cooldown settings
   - Check delivery statistics in dashboard

3. **High False Positive Rate**
   - Adjust monitoring thresholds in configuration
   - Review alert cooldown periods
   - Analyze historical data for threshold tuning

4. **Performance Impact**
   - Adjust monitoring intervals
   - Optimize metric collection frequency
   - Review cleanup intervals for old data

### Log Analysis

```bash
# View monitoring logs
tail -f monitoring.log

# Filter by severity
grep "ERROR" monitoring.log
grep "CRITICAL" monitoring.log

# View alert delivery logs
grep "alert" monitoring.log | grep "delivered"
```

### Health Check Endpoints

```bash
# Check individual component health
curl http://localhost:8080/health/solana
curl http://localhost:8080/health/oracle
curl http://localhost:8080/health/staking
curl http://localhost:3000/api/health
```

## Performance Considerations

### Resource Usage

- **CPU:** Monitoring adds ~2-5% CPU overhead
- **Memory:** ~50-100MB for monitoring service
- **Network:** Minimal impact, periodic health checks
- **Storage:** Log rotation and cleanup prevent unbounded growth

### Optimization Tips

1. **Adjust Check Intervals:** Balance monitoring frequency with performance
2. **Optimize Thresholds:** Reduce false positives with proper threshold tuning
3. **Efficient Alerting:** Use appropriate channels for different alert types
4. **Data Retention:** Configure appropriate retention periods for metrics and alerts

## Security Considerations

### Alert Channel Security

- Use HTTPS for all webhook endpoints
- Implement authentication for alert endpoints
- Rotate webhook URLs and API keys regularly
- Monitor alert delivery for potential security issues

### Sensitive Data Handling

- Avoid including sensitive data in alert messages
- Use alert IDs for correlation instead of full data
- Implement proper access controls for monitoring dashboard
- Audit alert delivery logs regularly

## Compliance and Auditing

### Audit Trail

- All alerts are logged with timestamps and metadata
- Delivery status tracked for compliance reporting
- Component health history maintained
- Performance metrics archived for analysis

### Compliance Features

- Configurable data retention periods
- Alert acknowledgment tracking
- Incident response integration
- Regulatory reporting capabilities

## Future Enhancements

### Planned Features

1. **Machine Learning Integration**
   - Anomaly detection for performance metrics
   - Predictive alerting based on trends
   - Automated threshold adjustment

2. **Advanced Analytics**
   - Historical trend analysis
   - Performance correlation analysis
   - Capacity planning insights

3. **Enhanced Integrations**
   - Additional alert channel support
   - Third-party monitoring tool integration
   - Cloud provider native monitoring

4. **Mobile Support**
   - Mobile-optimized dashboard
   - Push notifications
   - Offline alert queuing

## Support and Maintenance

### Regular Maintenance Tasks

1. **Weekly:**
   - Review alert statistics and delivery rates
   - Check monitoring service health
   - Analyze false positive rates

2. **Monthly:**
   - Update monitoring thresholds based on performance data
   - Review and rotate alert channel credentials
   - Analyze system performance trends

3. **Quarterly:**
   - Comprehensive monitoring system review
   - Update documentation and procedures
   - Performance optimization review

### Getting Help

- Check the troubleshooting section above
- Review monitoring logs for detailed error information
- Test individual components using the test scripts
- Contact the development team for complex issues

---

For more information about the Vault Protocol monitoring system, see the source code documentation and inline comments in the monitoring modules.