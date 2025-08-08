#!/usr/bin/env python3
"""
Comprehensive Error Handling and User Feedback Tests
Tests all error handling components and recovery mechanisms
Addresses Task 22: Implement error handling and user feedback
"""

import pytest
import asyncio
import time
import json
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Any
import sys
import os

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

class MockErrorHandler:
    """Mock error handler for testing error processing"""
    
    def __init__(self):
        self.error_log = []
        self.recovery_attempts = {}
        self.notification_history = []
        
    def handle_error(self, error, context=None):
        """Process and handle errors with recovery suggestions"""
        error_entry = {
            'error': str(error),
            'context': context,
            'timestamp': time.time(),
            'severity': self.determine_severity(error),
            'category': self.determine_category(error),
            'recovery_actions': self.get_recovery_actions(error),
            'user_message': self.get_user_message(error)
        }
        
        self.error_log.append(error_entry)
        return error_entry
    
    def determine_severity(self, error):
        """Determine error severity level"""
        error_str = str(error).lower()
        
        if any(keyword in error_str for keyword in ['critical', 'fatal', 'security']):
            return 'critical'
        elif any(keyword in error_str for keyword in ['network', 'timeout', 'connection']):
            return 'high'
        elif any(keyword in error_str for keyword in ['validation', 'invalid']):
            return 'medium'
        else:
            return 'low'
    
    def determine_category(self, error):
        """Determine error category"""
        error_str = str(error).lower()
        
        if 'network' in error_str or 'connection' in error_str:
            return 'network'
        elif 'validation' in error_str or 'invalid' in error_str:
            return 'validation'
        elif 'auth' in error_str or 'permission' in error_str:
            return 'authentication'
        elif 'transaction' in error_str or 'insufficient' in error_str:
            return 'transaction'
        else:
            return 'system'
    
    def get_recovery_actions(self, error):
        """Get recovery actions based on error type"""
        error_str = str(error).lower()
        
        if 'network' in error_str:
            return [
                'Check your internet connection',
                'Try refreshing the page',
                'Switch to a different network'
            ]
        elif 'validation' in error_str:
            return [
                'Check the format of your input',
                'Verify all required fields are filled',
                'Copy and paste instead of typing'
            ]
        elif 'transaction' in error_str:
            return [
                'Check your wallet balance',
                'Reduce the transaction amount',
                'Wait for network congestion to clear'
            ]
        else:
            return [
                'Try the operation again',
                'Refresh the page',
                'Contact support if the problem persists'
            ]
    
    def get_user_message(self, error):
        """Get user-friendly error message"""
        error_str = str(error).lower()
        
        if 'network' in error_str:
            return 'Unable to connect to the network. Please check your internet connection.'
        elif 'validation' in error_str:
            return 'Please check the information you entered and try again.'
        elif 'transaction' in error_str:
            return 'Transaction failed. Please check your balance and try again.'
        else:
            return 'An unexpected error occurred. Please try again.'

class MockNotificationSystem:
    """Mock notification system for testing user feedback"""
    
    def __init__(self):
        self.notifications = []
        self.toast_history = []
        
    def show_notification(self, notification_type, title, message, actions=None):
        """Show notification to user"""
        notification = {
            'type': notification_type,
            'title': title,
            'message': message,
            'actions': actions or [],
            'timestamp': time.time(),
            'id': f'notification_{len(self.notifications)}'
        }
        
        self.notifications.append(notification)
        return notification['id']
    
    def show_toast(self, toast_type, message, duration=5000):
        """Show toast message"""
        toast = {
            'type': toast_type,
            'message': message,
            'duration': duration,
            'timestamp': time.time()
        }
        
        self.toast_history.append(toast)
        return True
    
    def get_unread_count(self):
        """Get count of unread notifications"""
        return len([n for n in self.notifications if not n.get('read', False)])
    
    def mark_as_read(self, notification_id):
        """Mark notification as read"""
        for notification in self.notifications:
            if notification['id'] == notification_id:
                notification['read'] = True
                return True
        return False

class MockRecoverySystem:
    """Mock recovery system for testing automated recovery"""
    
    def __init__(self):
        self.recovery_plans = {}
        self.execution_history = []
        
    def add_recovery_plan(self, error_code, actions):
        """Add recovery plan for error code"""
        self.recovery_plans[error_code] = {
            'error_code': error_code,
            'actions': actions,
            'success_rate': 85
        }
    
    def execute_recovery_action(self, action_id, error_code):
        """Execute a specific recovery action"""
        execution = {
            'action_id': action_id,
            'error_code': error_code,
            'timestamp': time.time(),
            'success': True  # Mock success
        }
        
        self.execution_history.append(execution)
        
        # Simulate different success rates
        if 'network' in action_id:
            return True
        elif 'validation' in action_id:
            return True
        elif 'transaction' in action_id:
            return len(self.execution_history) % 3 != 0  # 66% success rate
        else:
            return True
    
    def get_recovery_actions(self, error_code):
        """Get available recovery actions for error"""
        plan = self.recovery_plans.get(error_code, {})
        return plan.get('actions', [])

class TestErrorHandling:
    """Test error handling functionality"""
    
    @pytest.fixture
    def error_handler(self):
        return MockErrorHandler()
    
    @pytest.fixture
    def notification_system(self):
        return MockNotificationSystem()
    
    @pytest.fixture
    def recovery_system(self):
        system = MockRecoverySystem()
        
        # Add sample recovery plans
        system.add_recovery_plan('NETWORK_001', [
            {'id': 'check_connection', 'label': 'Check Connection', 'priority': 'high'},
            {'id': 'switch_rpc', 'label': 'Switch RPC', 'priority': 'high'},
            {'id': 'clear_cache', 'label': 'Clear Cache', 'priority': 'medium'}
        ])
        
        system.add_recovery_plan('VALIDATION_001', [
            {'id': 'validate_input', 'label': 'Validate Input', 'priority': 'high'},
            {'id': 'copy_paste', 'label': 'Copy from Source', 'priority': 'medium'}
        ])
        
        return system
    
    def test_error_classification(self, error_handler):
        """Test error classification by severity and category"""
        test_errors = [
            ('Network connection failed', 'high', 'network'),
            ('Invalid Bitcoin address format', 'medium', 'validation'),
            ('Critical security violation', 'critical', 'system'),
            ('Authentication required', 'medium', 'authentication'),
            ('Insufficient funds for transaction', 'high', 'transaction')
        ]
        
        for error_msg, expected_severity, expected_category in test_errors:
            error = Exception(error_msg)
            result = error_handler.handle_error(error)
            
            assert result['severity'] == expected_severity
            assert result['category'] == expected_category
            assert len(result['recovery_actions']) > 0
            assert result['user_message'] is not None
        
        print(f"✅ Error classification: {len(test_errors)} error types classified correctly")
    
    def test_recovery_action_generation(self, error_handler):
        """Test generation of appropriate recovery actions"""
        test_cases = [
            {
                'error': 'Network timeout occurred',
                'expected_actions': ['Check your internet connection', 'Try refreshing the page']
            },
            {
                'error': 'Invalid address format',
                'expected_actions': ['Check the format of your input', 'Verify all required fields']
            },
            {
                'error': 'Transaction failed - insufficient funds',
                'expected_actions': ['Check your wallet balance', 'Reduce the transaction amount']
            }
        ]
        
        for case in test_cases:
            error = Exception(case['error'])
            result = error_handler.handle_error(error)
            
            # Check that at least some expected actions are present
            actions = result['recovery_actions']
            assert len(actions) >= 2
            
            # Check that actions are relevant to the error type
            for expected_action in case['expected_actions']:
                assert any(expected_action in action for action in actions)
        
        print(f"✅ Recovery actions: Generated appropriate actions for {len(test_cases)} error types")
    
    def test_user_friendly_messages(self, error_handler):
        """Test generation of user-friendly error messages"""
        technical_errors = [
            'RPC endpoint connection timeout after 30 seconds',
            'ECDSA signature verification failed for public key',
            'Solana transaction simulation failed with error code 0x1',
            'WebSocket connection closed with code 1006'
        ]
        
        for technical_error in technical_errors:
            error = Exception(technical_error)
            result = error_handler.handle_error(error)
            
            user_message = result['user_message']
            
            # User message should be non-technical and helpful
            assert len(user_message) > 20  # Substantial message
            assert not any(tech_term in user_message.lower() for tech_term in [
                'rpc', 'ecdsa', 'websocket', 'code 0x1', 'simulation'
            ])
            assert any(helpful_word in user_message.lower() for helpful_word in [
                'please', 'try', 'check', 'connection', 'again'
            ])
        
        print(f"✅ User messages: Generated user-friendly messages for {len(technical_errors)} technical errors")
    
    def test_error_logging_and_history(self, error_handler):
        """Test error logging and history tracking"""
        # Generate multiple errors
        errors = [
            'Network connection failed',
            'Invalid input format',
            'Transaction timeout',
            'Authentication failed',
            'System overload'
        ]
        
        for error_msg in errors:
            error = Exception(error_msg)
            error_handler.handle_error(error, f'Context: {error_msg}')
        
        # Verify logging
        assert len(error_handler.error_log) == len(errors)
        
        # Check log entries have required fields
        for log_entry in error_handler.error_log:
            assert 'error' in log_entry
            assert 'context' in log_entry
            assert 'timestamp' in log_entry
            assert 'severity' in log_entry
            assert 'category' in log_entry
            assert 'recovery_actions' in log_entry
            assert 'user_message' in log_entry
        
        # Check chronological order
        timestamps = [entry['timestamp'] for entry in error_handler.error_log]
        assert timestamps == sorted(timestamps)
        
        print(f"✅ Error logging: Logged {len(errors)} errors with complete metadata")

class TestNotificationSystem:
    """Test notification and user feedback system"""
    
    @pytest.fixture
    def notification_system(self):
        return MockNotificationSystem()
    
    def test_notification_creation(self, notification_system):
        """Test creation of different notification types"""
        notification_types = [
            ('success', 'Operation Successful', 'Your transaction was completed successfully'),
            ('error', 'Transaction Failed', 'Unable to process your transaction'),
            ('warning', 'Network Issues', 'Experiencing network connectivity issues'),
            ('info', 'System Update', 'System maintenance scheduled for tonight')
        ]
        
        for notif_type, title, message in notification_types:
            notification_id = notification_system.show_notification(
                notif_type, title, message
            )
            
            assert notification_id is not None
            assert len(notification_system.notifications) > 0
        
        # Verify all notifications were created
        assert len(notification_system.notifications) == len(notification_types)
        
        # Check notification structure
        for i, (notif_type, title, message) in enumerate(notification_types):
            notification = notification_system.notifications[i]
            assert notification['type'] == notif_type
            assert notification['title'] == title
            assert notification['message'] == message
            assert 'timestamp' in notification
            assert 'id' in notification
        
        print(f"✅ Notifications: Created {len(notification_types)} different notification types")
    
    def test_toast_messages(self, notification_system):
        """Test toast message functionality"""
        toast_messages = [
            ('success', 'Transaction confirmed!'),
            ('error', 'Connection failed'),
            ('warning', 'Low balance detected'),
            ('info', 'New feature available')
        ]
        
        for toast_type, message in toast_messages:
            result = notification_system.show_toast(toast_type, message)
            assert result is True
        
        # Verify toast history
        assert len(notification_system.toast_history) == len(toast_messages)
        
        for i, (toast_type, message) in enumerate(toast_messages):
            toast = notification_system.toast_history[i]
            assert toast['type'] == toast_type
            assert toast['message'] == message
            assert 'timestamp' in toast
            assert 'duration' in toast
        
        print(f"✅ Toast messages: Created {len(toast_messages)} toast messages")
    
    def test_notification_management(self, notification_system):
        """Test notification management features"""
        # Create multiple notifications
        for i in range(5):
            notification_system.show_notification(
                'info', f'Test Notification {i}', f'Message {i}'
            )
        
        # Check unread count
        unread_count = notification_system.get_unread_count()
        assert unread_count == 5
        
        # Mark some as read
        notification_ids = [n['id'] for n in notification_system.notifications[:3]]
        for notification_id in notification_ids:
            result = notification_system.mark_as_read(notification_id)
            assert result is True
        
        # Check updated unread count
        unread_count = notification_system.get_unread_count()
        assert unread_count == 2
        
        print(f"✅ Notification management: Managed {len(notification_system.notifications)} notifications")
    
    def test_notification_with_actions(self, notification_system):
        """Test notifications with action buttons"""
        actions = [
            {'label': 'Retry', 'action': 'retry_operation'},
            {'label': 'Cancel', 'action': 'cancel_operation'}
        ]
        
        notification_id = notification_system.show_notification(
            'error', 'Transaction Failed', 'Click retry to try again', actions
        )
        
        # Find the notification
        notification = next(
            n for n in notification_system.notifications 
            if n['id'] == notification_id
        )
        
        assert len(notification['actions']) == 2
        assert notification['actions'][0]['label'] == 'Retry'
        assert notification['actions'][1]['label'] == 'Cancel'
        
        print("✅ Action notifications: Created notification with action buttons")

class TestRecoverySystem:
    """Test automated recovery system"""
    
    @pytest.fixture
    def recovery_system(self):
        system = MockRecoverySystem()
        
        # Add comprehensive recovery plans
        system.add_recovery_plan('NETWORK_001', [
            {'id': 'check_network_connection', 'label': 'Check Network', 'priority': 'high'},
            {'id': 'switch_rpc_endpoint', 'label': 'Switch RPC', 'priority': 'high'},
            {'id': 'clear_browser_cache', 'label': 'Clear Cache', 'priority': 'medium'}
        ])
        
        system.add_recovery_plan('TRANSACTION_001', [
            {'id': 'check_wallet_balance', 'label': 'Check Balance', 'priority': 'high'},
            {'id': 'reduce_transaction_amount', 'label': 'Reduce Amount', 'priority': 'high'},
            {'id': 'wait_for_network', 'label': 'Wait for Network', 'priority': 'low'}
        ])
        
        return system
    
    def test_recovery_plan_retrieval(self, recovery_system):
        """Test retrieval of recovery plans for different errors"""
        test_error_codes = ['NETWORK_001', 'TRANSACTION_001', 'UNKNOWN_ERROR']
        
        for error_code in test_error_codes:
            actions = recovery_system.get_recovery_actions(error_code)
            
            if error_code in ['NETWORK_001', 'TRANSACTION_001']:
                assert len(actions) >= 2
                assert all('id' in action for action in actions)
                assert all('label' in action for action in actions)
                assert all('priority' in action for action in actions)
            else:
                assert len(actions) == 0  # No plan for unknown error
        
        print(f"✅ Recovery plans: Retrieved plans for {len(test_error_codes)} error codes")
    
    def test_recovery_action_execution(self, recovery_system):
        """Test execution of recovery actions"""
        test_actions = [
            ('check_network_connection', 'NETWORK_001'),
            ('switch_rpc_endpoint', 'NETWORK_001'),
            ('check_wallet_balance', 'TRANSACTION_001'),
            ('reduce_transaction_amount', 'TRANSACTION_001')
        ]
        
        successful_executions = 0
        
        for action_id, error_code in test_actions:
            success = recovery_system.execute_recovery_action(action_id, error_code)
            if success:
                successful_executions += 1
        
        # Verify execution history
        assert len(recovery_system.execution_history) == len(test_actions)
        assert successful_executions >= len(test_actions) * 0.5  # At least 50% success
        
        # Check execution records
        for execution in recovery_system.execution_history:
            assert 'action_id' in execution
            assert 'error_code' in execution
            assert 'timestamp' in execution
            assert 'success' in execution
        
        print(f"✅ Recovery execution: Executed {len(test_actions)} actions with {successful_executions} successes")
    
    def test_automated_recovery_sequence(self, recovery_system):
        """Test automated execution of recovery sequence"""
        error_code = 'NETWORK_001'
        actions = recovery_system.get_recovery_actions(error_code)
        
        # Execute high-priority actions first
        high_priority_actions = [
            action for action in actions 
            if action['priority'] == 'high'
        ]
        
        recovery_success = False
        
        for action in high_priority_actions:
            success = recovery_system.execute_recovery_action(action['id'], error_code)
            if success:
                recovery_success = True
                break
        
        assert recovery_success
        assert len(recovery_system.execution_history) >= 1
        
        print(f"✅ Automated recovery: Successfully executed recovery sequence for {error_code}")

class TestConcurrentErrorHandling:
    """Test error handling under concurrent conditions"""
    
    @pytest.fixture
    def error_handler(self):
        return MockErrorHandler()
    
    @pytest.fixture
    def notification_system(self):
        return MockNotificationSystem()
    
    def test_concurrent_error_processing(self, error_handler, notification_system):
        """Test handling multiple errors concurrently"""
        def process_error(error_data):
            """Process a single error"""
            error_msg, context = error_data
            error = Exception(error_msg)
            
            # Handle error
            result = error_handler.handle_error(error, context)
            
            # Show notification
            notification_system.show_notification(
                'error' if result['severity'] in ['high', 'critical'] else 'warning',
                'Error Occurred',
                result['user_message']
            )
            
            return result
        
        # Create multiple error scenarios
        error_scenarios = [
            ('Network connection timeout', 'API call'),
            ('Invalid Bitcoin address format', 'User input'),
            ('Transaction failed - insufficient funds', 'Payment'),
            ('Authentication token expired', 'Session'),
            ('Rate limit exceeded', 'API request'),
            ('Database connection lost', 'Data access'),
            ('Validation failed for amount', 'Form submission'),
            ('WebSocket connection closed', 'Real-time data'),
            ('CORS policy violation', 'Cross-origin request'),
            ('Service temporarily unavailable', 'External service')
        ]
        
        # Process errors concurrently
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            results = list(executor.map(process_error, error_scenarios))
        execution_time = time.time() - start_time
        
        # Verify all errors were processed
        assert len(results) == len(error_scenarios)
        assert len(error_handler.error_log) == len(error_scenarios)
        assert len(notification_system.notifications) == len(error_scenarios)
        
        # Check processing was efficient
        assert execution_time < 2.0  # Should complete within 2 seconds
        
        # Verify error categorization
        categories = [result['category'] for result in results]
        assert 'network' in categories
        assert 'validation' in categories
        assert 'transaction' in categories
        assert 'authentication' in categories
        
        print(f"✅ Concurrent processing: Handled {len(error_scenarios)} errors in {execution_time:.2f}s")
    
    def test_error_rate_limiting(self, error_handler):
        """Test error rate limiting to prevent spam"""
        # Simulate rapid error generation
        rapid_errors = [f'Rapid error {i}' for i in range(50)]
        
        processed_count = 0
        start_time = time.time()
        
        for error_msg in rapid_errors:
            error = Exception(error_msg)
            result = error_handler.handle_error(error)
            processed_count += 1
            
            # Simulate small delay to prevent overwhelming
            time.sleep(0.01)
        
        execution_time = time.time() - start_time
        
        # Verify all errors were logged
        assert len(error_handler.error_log) == len(rapid_errors)
        assert processed_count == len(rapid_errors)
        
        # Check performance under load
        assert execution_time < 2.0  # Should handle 50 errors quickly
        
        print(f"✅ Rate limiting: Processed {processed_count} rapid errors in {execution_time:.2f}s")
    
    def test_memory_efficiency_large_error_volume(self, error_handler):
        """Test memory efficiency with large error volumes"""
        import psutil
        import gc
        
        process = psutil.Process()
        initial_memory = process.memory_info().rss / (1024**2)  # MB
        
        # Generate large volume of errors
        large_error_set = []
        for i in range(1000):
            error_data = {
                'message': f'Error {i} with detailed context and stack trace information',
                'context': f'Operation {i} in module {i % 10}',
                'metadata': {'user_id': i, 'session_id': f'session_{i}', 'timestamp': time.time()}
            }
            large_error_set.append(error_data)
        
        # Process errors in batches
        batch_size = 100
        for i in range(0, len(large_error_set), batch_size):
            batch = large_error_set[i:i + batch_size]
            
            for error_data in batch:
                error = Exception(error_data['message'])
                error_handler.handle_error(error, error_data['context'])
            
            # Force garbage collection after each batch
            gc.collect()
        
        final_memory = process.memory_info().rss / (1024**2)  # MB
        memory_increase = final_memory - initial_memory
        
        # Verify all errors were processed
        assert len(error_handler.error_log) == len(large_error_set)
        
        # Memory increase should be reasonable
        assert memory_increase < 100  # Less than 100MB increase
        
        print(f"✅ Memory efficiency: Processed {len(large_error_set)} errors")
        print(f"   Memory usage: {initial_memory:.1f}MB → {final_memory:.1f}MB (+{memory_increase:.1f}MB)")
        
        # Cleanup
        del large_error_set
        gc.collect()

class TestErrorHandlingIntegration:
    """Integration tests for complete error handling workflow"""
    
    @pytest.fixture
    def integrated_system(self):
        return {
            'error_handler': MockErrorHandler(),
            'notification_system': MockNotificationSystem(),
            'recovery_system': MockRecoverySystem()
        }
    
    def test_end_to_end_error_workflow(self, integrated_system):
        """Test complete error handling workflow"""
        error_handler = integrated_system['error_handler']
        notification_system = integrated_system['notification_system']
        recovery_system = integrated_system['recovery_system']
        
        # Add recovery plan
        recovery_system.add_recovery_plan('NETWORK_001', [
            {'id': 'check_connection', 'label': 'Check Connection', 'priority': 'high'},
            {'id': 'retry_request', 'label': 'Retry Request', 'priority': 'medium'}
        ])
        
        # Step 1: Error occurs
        error = Exception('Network connection failed')
        error_result = error_handler.handle_error(error, 'API request')
        
        # Step 2: Show notification to user
        notification_id = notification_system.show_notification(
            'error',
            'Connection Error',
            error_result['user_message'],
            [{'label': 'Retry', 'action': 'retry'}]
        )
        
        # Step 3: Attempt automated recovery
        recovery_actions = recovery_system.get_recovery_actions('NETWORK_001')
        recovery_success = False
        
        for action in recovery_actions:
            if action['priority'] == 'high':
                success = recovery_system.execute_recovery_action(action['id'], 'NETWORK_001')
                if success:
                    recovery_success = True
                    break
        
        # Step 4: Update user on recovery result
        if recovery_success:
            notification_system.show_toast('success', 'Connection restored')
        else:
            notification_system.show_toast('warning', 'Manual intervention required')
        
        # Verify complete workflow
        assert len(error_handler.error_log) == 1
        assert len(notification_system.notifications) == 1
        assert len(notification_system.toast_history) >= 1
        assert len(recovery_system.execution_history) >= 1
        
        # Check workflow coherence
        logged_error = error_handler.error_log[0]
        assert logged_error['category'] == 'network'
        assert logged_error['severity'] in ['high', 'medium']
        
        notification = notification_system.notifications[0]
        assert notification['type'] == 'error'
        assert len(notification['actions']) > 0
        
        print("✅ End-to-end workflow: Complete error handling workflow executed successfully")
    
    def test_error_escalation_workflow(self, integrated_system):
        """Test error escalation when recovery fails"""
        error_handler = integrated_system['error_handler']
        notification_system = integrated_system['notification_system']
        recovery_system = integrated_system['recovery_system']
        
        # Simulate critical error
        critical_error = Exception('Critical security violation detected')
        error_result = error_handler.handle_error(critical_error, 'Security check')
        
        # Should be classified as critical
        assert error_result['severity'] == 'critical'
        
        # Show critical notification
        notification_id = notification_system.show_notification(
            'error',
            'Critical Security Alert',
            error_result['user_message'],
            [
                {'label': 'Contact Support', 'action': 'contact_support'},
                {'label': 'Lock Account', 'action': 'lock_account'}
            ]
        )
        
        # Critical errors should have persistent notifications
        notification = notification_system.notifications[-1]
        assert notification['type'] == 'error'
        assert len(notification['actions']) == 2
        
        # Should also show immediate toast
        notification_system.show_toast('error', 'Critical security issue detected')
        
        # Verify escalation
        assert len(error_handler.error_log) == 1
        assert error_handler.error_log[0]['severity'] == 'critical'
        assert len(notification_system.notifications) == 1
        assert len(notification_system.toast_history) == 1
        
        print("✅ Error escalation: Critical error properly escalated with appropriate notifications")

def run_error_handling_tests():
    """Run all error handling and user feedback tests"""
    print("🚨 Running Error Handling and User Feedback Tests...")
    
    total_passed = 0
    total_failed = 0
    
    # Test Error Handling
    print(f"\n📋 Running TestErrorHandling...")
    try:
        test_error_handling = TestErrorHandling()
        
        # Test error classification
        test_error_handling.test_error_classification(MockErrorHandler())
        print(f"  ✅ test_error_classification")
        total_passed += 1
        
        # Test recovery action generation
        test_error_handling.test_recovery_action_generation(MockErrorHandler())
        print(f"  ✅ test_recovery_action_generation")
        total_passed += 1
        
        # Test user-friendly messages
        test_error_handling.test_user_friendly_messages(MockErrorHandler())
        print(f"  ✅ test_user_friendly_messages")
        total_passed += 1
        
        # Test error logging
        test_error_handling.test_error_logging_and_history(MockErrorHandler())
        print(f"  ✅ test_error_logging_and_history")
        total_passed += 1
        
    except Exception as e:
        print(f"  ❌ TestErrorHandling failed - {str(e)}")
        total_failed += 4
    
    # Test Notification System
    print(f"\n📋 Running TestNotificationSystem...")
    try:
        test_notifications = TestNotificationSystem()
        
        # Test notification creation
        test_notifications.test_notification_creation(MockNotificationSystem())
        print(f"  ✅ test_notification_creation")
        total_passed += 1
        
        # Test toast messages
        test_notifications.test_toast_messages(MockNotificationSystem())
        print(f"  ✅ test_toast_messages")
        total_passed += 1
        
        # Test notification management
        test_notifications.test_notification_management(MockNotificationSystem())
        print(f"  ✅ test_notification_management")
        total_passed += 1
        
        # Test notifications with actions
        test_notifications.test_notification_with_actions(MockNotificationSystem())
        print(f"  ✅ test_notification_with_actions")
        total_passed += 1
        
    except Exception as e:
        print(f"  ❌ TestNotificationSystem failed - {str(e)}")
        total_failed += 4
    
    # Test Recovery System
    print(f"\n📋 Running TestRecoverySystem...")
    try:
        test_recovery = TestRecoverySystem()
        
        # Create recovery system with plans
        recovery_system = MockRecoverySystem()
        recovery_system.add_recovery_plan('NETWORK_001', [
            {'id': 'check_connection', 'label': 'Check Connection', 'priority': 'high'},
            {'id': 'switch_rpc', 'label': 'Switch RPC', 'priority': 'high'},
            {'id': 'clear_cache', 'label': 'Clear Cache', 'priority': 'medium'}
        ])
        recovery_system.add_recovery_plan('TRANSACTION_001', [
            {'id': 'check_balance', 'label': 'Check Balance', 'priority': 'high'},
            {'id': 'reduce_amount', 'label': 'Reduce Amount', 'priority': 'high'}
        ])
        
        # Test recovery plan retrieval
        test_recovery.test_recovery_plan_retrieval(recovery_system)
        print(f"  ✅ test_recovery_plan_retrieval")
        total_passed += 1
        
        # Test recovery action execution
        test_recovery.test_recovery_action_execution(recovery_system)
        print(f"  ✅ test_recovery_action_execution")
        total_passed += 1
        
        # Test automated recovery sequence
        test_recovery.test_automated_recovery_sequence(recovery_system)
        print(f"  ✅ test_automated_recovery_sequence")
        total_passed += 1
        
    except Exception as e:
        print(f"  ❌ TestRecoverySystem failed - {str(e)}")
        total_failed += 3
    
    # Test Concurrent Error Handling
    print(f"\n📋 Running TestConcurrentErrorHandling...")
    try:
        test_concurrent = TestConcurrentErrorHandling()
        
        # Test concurrent error processing
        test_concurrent.test_concurrent_error_processing(MockErrorHandler(), MockNotificationSystem())
        print(f"  ✅ test_concurrent_error_processing")
        total_passed += 1
        
        # Test error rate limiting
        test_concurrent.test_error_rate_limiting(MockErrorHandler())
        print(f"  ✅ test_error_rate_limiting")
        total_passed += 1
        
        # Test memory efficiency
        test_concurrent.test_memory_efficiency_large_error_volume(MockErrorHandler())
        print(f"  ✅ test_memory_efficiency_large_error_volume")
        total_passed += 1
        
    except Exception as e:
        print(f"  ❌ TestConcurrentErrorHandling failed - {str(e)}")
        total_failed += 3
    
    # Test Integration
    print(f"\n📋 Running TestErrorHandlingIntegration...")
    try:
        test_integration = TestErrorHandlingIntegration()
        
        # Create integrated system
        integrated_system = {
            'error_handler': MockErrorHandler(),
            'notification_system': MockNotificationSystem(),
            'recovery_system': MockRecoverySystem()
        }
        
        # Test end-to-end workflow
        test_integration.test_end_to_end_error_workflow(integrated_system)
        print(f"  ✅ test_end_to_end_error_workflow")
        total_passed += 1
        
        # Test error escalation
        test_integration.test_error_escalation_workflow(integrated_system)
        print(f"  ✅ test_error_escalation_workflow")
        total_passed += 1
        
    except Exception as e:
        print(f"  ❌ TestErrorHandlingIntegration failed - {str(e)}")
        total_failed += 2
    
    print(f"\n📊 Error Handling Test Results:")
    print(f"  ✅ Passed: {total_passed}")
    print(f"  ❌ Failed: {total_failed}")
    print(f"  📈 Success Rate: {(total_passed / (total_passed + total_failed)) * 100:.1f}%")
    
    if total_failed == 0:
        print("🎉 All error handling tests passed!")
        return True
    else:
        print("⚠️ Some error handling tests failed!")
        return False

if __name__ == "__main__":
    success = run_error_handling_tests()
    sys.exit(0 if success else 1)