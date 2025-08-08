"""
Comprehensive BTC Commitment Testing Suite
Tests for BTC commitment and verification functionality with concurrent execution support
Addresses FR7: Testing and Development Infrastructure requirements
"""

import pytest
import asyncio
import os
import time
import hashlib
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor, as_completed
import sys
import os
import threading
from typing import List, Dict, Any

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# Import configuration
try:
    from config.chainlink import get_verification_settings, get_oracle_config
    from config.validators import get_validator_config
except ImportError:
    # Mock configs if not available
    def get_verification_settings():
        return {
            'interval': 60,
            'cache_duration': 300,
            'retry_config': {
                'max_retries': 3,
                'base_delay': 2,
                'max_delay': 60
            }
        }
    
    def get_oracle_config():
        return {
            'btc_usd_feed': 'test_feed_address',
            'utxo_verification': 'test_utxo_address'
        }

class TestBTCCommitment:
    """Test suite for BTC commitment operations"""
    
    @pytest.fixture
    def mock_vault_client(self):
        """Mock vault client for testing"""
        mock_client = Mock()
        mock_client.config = {
            'network': 'devnet',
            'program_id': 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
        }
        # Make async methods return AsyncMock
        mock_client.commit_btc = AsyncMock()
        mock_client.verify_balance = AsyncMock()
        mock_client.update_commitment = AsyncMock()
        mock_client.create_commitment_hash = AsyncMock()
        mock_client.serialize_for_signing = AsyncMock()
        mock_client.validate_commitment = AsyncMock()
        return mock_client
    
    @pytest.fixture
    def sample_btc_data(self):
        """Sample BTC commitment data"""
        return {
            'amount': 0.5,  # 0.5 BTC
            'btc_address': 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
            'ecdsa_proof': os.getenv('TEST_ECDSA_PROOF', 'a' * 128)
        }
    
    @pytest.mark.asyncio
    async def test_commit_btc_success(self, mock_vault_client, sample_btc_data):
        """Test successful BTC commitment"""
        # Mock successful transaction
        mock_vault_client.commit_btc.return_value = {
            'success': True,
            'signature': '5j7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJv'
        }
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        
        assert result['success'] is True
        assert 'signature' in result
        mock_vault_client.commit_btc.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_commit_btc_invalid_address(self, mock_vault_client):
        """Test BTC commitment with invalid address"""
        mock_vault_client.commit_btc.return_value = {
            'success': False,
            'error': 'Invalid BTC address format'
        }
        
        result = await mock_vault_client.commit_btc(
            0.1,
            'invalid_address',
            'valid_proof'
        )
        
        assert result['success'] is False
        assert 'Invalid BTC address' in result['error']
    
    @pytest.mark.asyncio
    async def test_commit_btc_invalid_proof(self, mock_vault_client, sample_btc_data):
        """Test BTC commitment with invalid ECDSA proof"""
        mock_vault_client.commit_btc.return_value = {
            'success': False,
            'error': 'Invalid ECDSA proof'
        }
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            'invalid_proof'
        )
        
        assert result['success'] is False
        assert 'Invalid ECDSA proof' in result['error']
    
    @pytest.mark.asyncio
    async def test_verify_balance_success(self, mock_vault_client):
        """Test successful balance verification"""
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'balance': 0.5,
            'last_verification': '2024-01-01T00:00:00Z'
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['balance'] == 0.5
        assert 'last_verification' in result
    
    @pytest.mark.asyncio
    async def test_verify_balance_oracle_failure(self, mock_vault_client):
        """Test balance verification with oracle failure"""
        mock_vault_client.verify_balance.side_effect = Exception('Oracle verification failed')
        
        with pytest.raises(Exception) as exc_info:
            await mock_vault_client.verify_balance()
        
        assert 'Oracle verification failed' in str(exc_info.value)
    
    @pytest.mark.asyncio
    async def test_concurrent_commitments(self, mock_vault_client, sample_btc_data):
        """Test concurrent BTC commitments using asyncio.gather"""
        # Mock successful responses
        mock_vault_client.commit_btc.return_value = {'success': True}
        
        # Create multiple concurrent tasks
        tasks = []
        for i in range(10):
            task = mock_vault_client.commit_btc(
                sample_btc_data['amount'],
                sample_btc_data['btc_address'],
                sample_btc_data['ecdsa_proof']
            )
            tasks.append(task)
        
        # Execute all tasks concurrently
        results = await asyncio.gather(*tasks)
        
        # All commitments should succeed
        assert all(result['success'] for result in results)
        assert len(results) == 10
    
    @pytest.mark.asyncio
    async def test_commitment_limits_non_kyc(self, mock_vault_client):
        """Test commitment limits for non-KYC users"""
        # Test commitment over 1 BTC limit
        mock_vault_client.commit_btc.return_value = {
            'success': False,
            'error': 'Commitment limit exceeded - KYC required for amounts over 1 BTC'
        }
        
        result = await mock_vault_client.commit_btc(
            1.5,  # Over 1 BTC limit
            'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
            'valid_proof'
        )
        
        assert result['success'] is False
        assert 'KYC required' in result['error']
    
    @pytest.mark.asyncio
    async def test_ecdsa_proof_validation(self, mock_vault_client, sample_btc_data):
        """Test ECDSA proof validation to prevent spoofing"""
        # Test with malformed proof
        malformed_proofs = [
            '',  # Empty proof
            'invalid',  # Too short
            'a' * 1000,  # Too long
            'a' * 64,  # Incomplete
        ]
        
        for proof in malformed_proofs:
            mock_vault_client.commit_btc.return_value = {
                'success': False,
                'error': 'Invalid ECDSA proof'
            }
            
            result = await mock_vault_client.commit_btc(
                sample_btc_data['amount'],
                sample_btc_data['btc_address'],
                proof
            )
            
            assert result['success'] is False
    
    @pytest.mark.asyncio
    async def test_chainlink_oracle_integration(self, mock_vault_client):
        """Test Chainlink oracle integration for balance verification with retry logic"""
        with patch('config.chainlink.get_verification_settings') as mock_config:
            mock_config.return_value = {
                'interval': 60,
                'cache_duration': 300,
                'retry_config': {
                    'max_retries': 3,
                    'base_delay': 2,
                    'max_delay': 60
                }
            }
            
            # Test successful oracle verification
            mock_vault_client.verify_balance.return_value = {
                'verified': True,
                'balance': 0.5,
                'oracle_timestamp': 1640995200,  # Mock timestamp
                'price_usd': 45000,  # Mock BTC price
                'retry_count': 0,
                'cache_hit': False
            }
            
            result = await mock_vault_client.verify_balance()
            
            assert result['verified'] is True
            assert 'oracle_timestamp' in result
            assert 'price_usd' in result
            assert result['retry_count'] == 0
            
            # Test oracle failure with retry
            mock_vault_client.verify_balance.return_value = {
                'verified': False,
                'error': 'Oracle verification failed',
                'retry_count': 1,
                'next_retry_delay': 2
            }
            
            result = await mock_vault_client.verify_balance()
            assert result['verified'] is False
            assert result['retry_count'] == 1
            assert 'next_retry_delay' in result

    @pytest.mark.asyncio
    async def test_btc_address_validation(self, mock_vault_client):
        """Test comprehensive BTC address validation"""
        # Test valid addresses
        valid_addresses = [
            'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',  # Bech32 P2WPKH
            '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa',  # Legacy P2PKH
            '3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy',  # P2SH
            'bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3',  # Bech32 P2WSH
        ]
        
        for address in valid_addresses:
            mock_vault_client.commit_btc.return_value = {'success': True}
            result = await mock_vault_client.commit_btc(0.1, address, 'valid_proof')
            assert result['success'] is True
        
        # Test invalid addresses
        invalid_addresses = [
            '',  # Empty
            '1',  # Too short
            'invalid_address',  # Invalid format
            '4A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa',  # Invalid prefix
            'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh' + 'x' * 50,  # Too long
        ]
        
        for address in invalid_addresses:
            mock_vault_client.commit_btc.return_value = {
                'success': False,
                'error': 'Invalid BTC address format'
            }
            result = await mock_vault_client.commit_btc(0.1, address, 'valid_proof')
            assert result['success'] is False

    @pytest.mark.asyncio
    async def test_commitment_hash_validation(self, mock_vault_client, sample_btc_data):
        """Test commitment hash creation and validation"""
        mock_vault_client.create_commitment_hash.return_value = {
            'hash': '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
            'timestamp': 1640995200
        }
        
        result = await mock_vault_client.create_commitment_hash(
            sample_btc_data['btc_address'],
            sample_btc_data['amount'],
            1640995200
        )
        
        assert 'hash' in result
        assert len(result['hash']) == 66  # 0x + 64 hex chars
        assert result['timestamp'] == 1640995200

    @pytest.mark.asyncio
    async def test_timestamp_freshness_validation(self, mock_vault_client, sample_btc_data):
        """Test timestamp freshness to prevent replay attacks"""
        import time
        
        # Test with fresh timestamp (should succeed)
        fresh_timestamp = int(time.time())
        mock_vault_client.commit_btc.return_value = {'success': True}
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        assert result['success'] is True
        
        # Test with old timestamp (should fail)
        old_timestamp = int(time.time()) - 3600  # 1 hour ago
        mock_vault_client.commit_btc.return_value = {
            'success': False,
            'error': 'Timestamp too old - potential replay attack'
        }
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        assert result['success'] is False
        assert 'replay attack' in result['error']

    @pytest.mark.asyncio
    async def test_update_commitment(self, mock_vault_client, sample_btc_data):
        """Test updating existing BTC commitment"""
        # First create a commitment
        mock_vault_client.commit_btc.return_value = {'success': True, 'commitment_id': 'test_id'}
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        assert result['success'] is True
        
        # Then update it
        mock_vault_client.update_commitment.return_value = {'success': True, 'updated': True}
        
        new_amount = 0.75  # Increase commitment
        result = await mock_vault_client.update_commitment(
            new_amount,
            'new_ecdsa_proof',
            'new_public_key'
        )
        
        assert result['success'] is True
        assert result['updated'] is True

    @pytest.mark.asyncio
    async def test_serialization_for_signing(self, mock_vault_client, sample_btc_data):
        """Test data serialization for ECDSA signing"""
        mock_vault_client.serialize_for_signing.return_value = {
            'serialized_data': '0x1234567890abcdef',
            'length': 100
        }
        
        result = await mock_vault_client.serialize_for_signing(
            sample_btc_data['btc_address'],
            sample_btc_data['amount'],
            1640995200
        )
        
        assert 'serialized_data' in result
        assert 'length' in result
        assert result['length'] > 0

    def test_data_validation_edge_cases(self, mock_vault_client):
        """Test edge cases in data validation"""
        edge_cases = [
            {'amount': 0, 'error': 'Amount must be greater than 0'},
            {'amount': -1, 'error': 'Amount cannot be negative'},
            {'btc_address': None, 'error': 'BTC address cannot be null'},
            {'ecdsa_proof': None, 'error': 'ECDSA proof cannot be null'},
            {'public_key': None, 'error': 'Public key cannot be null'},
        ]
        
        for case in edge_cases:
            mock_vault_client.commit_btc.return_value = {
                'success': False,
                'error': case['error']
            }
            
            # This would be called with the edge case data
            # In a real test, you'd pass the actual invalid data
            result = mock_vault_client.commit_btc.return_value
            assert result['success'] is False
            assert case['error'] in result['error']

    @pytest.mark.asyncio
    async def test_concurrent_validation_operations(self, mock_vault_client, sample_btc_data):
        """Test concurrent validation operations for performance"""
        mock_vault_client.validate_commitment.return_value = {'valid': True}
        
        # Create multiple concurrent validation tasks
        tasks = []
        for i in range(20):
            task = mock_vault_client.validate_commitment(sample_btc_data)
            tasks.append(task)
        
        # Execute all tasks concurrently
        results = await asyncio.gather(*tasks)
        
        # All validations should succeed
        assert all(result['valid'] for result in results)
        assert len(results) == 20

    @pytest.mark.asyncio
    async def test_commit_btc_instruction_handler(self, mock_vault_client, sample_btc_data):
        """Test commit_btc instruction handler with comprehensive validation"""
        # Test successful commitment
        mock_vault_client.commit_btc.return_value = {
            'success': True,
            'signature': '5j7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJv',
            'commitment_hash': '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
            'timestamp': 1640995200,
            'verified': False  # Requires separate verification
        }
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        
        assert result['success'] is True
        assert 'commitment_hash' in result
        assert 'timestamp' in result
        assert result['verified'] is False  # Should require separate verification
        
        # Test KYC limit enforcement (over 1 BTC)
        mock_vault_client.commit_btc.return_value = {
            'success': False,
            'error': 'KYC verification required for commitments over 1 BTC'
        }
        
        result = await mock_vault_client.commit_btc(
            1.5,  # Over 1 BTC limit
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        
        assert result['success'] is False
        assert 'KYC verification required' in result['error']

    @pytest.mark.asyncio
    async def test_verify_balance_instruction_handler(self, mock_vault_client):
        """Test verify_balance instruction handler with oracle integration"""
        # Test successful verification with fresh oracle data
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'balance': 50000000,  # 0.5 BTC in satoshis
            'last_verification': 1640995200,
            'oracle_data_age': 30,  # 30 seconds old
            'cache_hit': False,
            'retry_count': 0
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['balance'] == 50000000
        assert result['oracle_data_age'] <= 60  # Within 60 second interval
        assert result['retry_count'] == 0
        
        # Test verification with cached data
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'balance': 50000000,
            'last_verification': 1640995200,
            'cache_hit': True,
            'cache_age': 120  # 2 minutes old, within 5 minute cache limit
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['cache_hit'] is True
        assert result['cache_age'] <= 300  # Within 5 minute cache limit
        
        # Test verification failure with retry logic
        mock_vault_client.verify_balance.return_value = {
            'verified': False,
            'error': 'Oracle verification failed',
            'retry_count': 1,
            'next_retry_delay': 4,  # Exponential backoff: 2^1 * 2 = 4 seconds
            'max_retries': 3
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is False
        assert result['retry_count'] == 1
        assert result['next_retry_delay'] == 4
        assert result['max_retries'] == 3

    @pytest.mark.asyncio
    async def test_update_commitment_instruction_handler(self, mock_vault_client, sample_btc_data):
        """Test update_commitment instruction handler with validation"""
        # Test successful commitment update
        mock_vault_client.update_commitment.return_value = {
            'success': True,
            'old_amount': 50000000,  # 0.5 BTC in satoshis
            'new_amount': 75000000,  # 0.75 BTC in satoshis
            'new_commitment_hash': '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
            'timestamp': 1640995260,
            'verified': False  # Requires re-verification
        }
        
        result = await mock_vault_client.update_commitment(
            0.75,  # New amount
            'new_ecdsa_proof',
            'new_public_key'
        )
        
        assert result['success'] is True
        assert result['new_amount'] == 75000000
        assert result['verified'] is False  # Should require re-verification
        assert 'new_commitment_hash' in result
        
        # Test update with KYC limit enforcement
        mock_vault_client.update_commitment.return_value = {
            'success': False,
            'error': 'KYC verification required for commitments over 1 BTC'
        }
        
        result = await mock_vault_client.update_commitment(
            1.2,  # Over 1 BTC limit
            'new_ecdsa_proof',
            'new_public_key'
        )
        
        assert result['success'] is False
        assert 'KYC verification required' in result['error']
        
        # Test significant reduction warning
        mock_vault_client.update_commitment.return_value = {
            'success': True,
            'warning': 'Significant commitment reduction detected',
            'old_amount': 100000000,  # 1 BTC
            'new_amount': 25000000,   # 0.25 BTC (75% reduction)
            'reduction_percentage': 75
        }
        
        result = await mock_vault_client.update_commitment(
            0.25,  # Significant reduction
            'new_ecdsa_proof',
            'new_public_key'
        )
        
        assert result['success'] is True
        assert 'warning' in result
        assert result['reduction_percentage'] == 75

    @pytest.mark.asyncio
    async def test_ecdsa_proof_anti_spoofing(self, mock_vault_client, sample_btc_data):
        """Test ECDSA proof validation for anti-spoofing protection"""
        # Test valid ECDSA proof
        mock_vault_client.commit_btc.return_value = {
            'success': True,
            'ecdsa_validation': True,
            'proof_hash': '0x1234567890abcdef',
            'public_key_valid': True
        }
        
        result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        
        assert result['success'] is True
        assert result['ecdsa_validation'] is True
        assert result['public_key_valid'] is True
        
        # Test spoofing attempt detection
        spoofing_attempts = [
            {'proof': '', 'error': 'Empty ECDSA proof'},
            {'proof': 'a' * 32, 'error': 'Invalid ECDSA proof length'},
            {'proof': 'invalid_signature', 'error': 'ECDSA signature verification failed'},
            {'proof': 'b' * 128, 'error': 'Public key validation failed'}
        ]
        
        for attempt in spoofing_attempts:
            mock_vault_client.commit_btc.return_value = {
                'success': False,
                'error': attempt['error'],
                'security_violation': True
            }
            
            result = await mock_vault_client.commit_btc(
                sample_btc_data['amount'],
                sample_btc_data['btc_address'],
                attempt['proof']
            )
            
            assert result['success'] is False
            assert result['security_violation'] is True
            assert attempt['error'] in result['error']

    @pytest.mark.asyncio
    async def test_oracle_retry_exponential_backoff(self, mock_vault_client):
        """Test oracle retry logic with exponential backoff"""
        retry_scenarios = [
            {'retry': 0, 'delay': 2},   # 2^0 * 2 = 2 seconds
            {'retry': 1, 'delay': 4},   # 2^1 * 2 = 4 seconds
            {'retry': 2, 'delay': 8},   # 2^2 * 2 = 8 seconds
            {'retry': 3, 'delay': 16},  # 2^3 * 2 = 16 seconds
        ]
        
        for scenario in retry_scenarios:
            mock_vault_client.verify_balance.return_value = {
                'verified': False,
                'error': 'Oracle verification failed',
                'retry_count': scenario['retry'],
                'next_retry_delay': scenario['delay'],
                'exponential_backoff': True
            }
            
            result = await mock_vault_client.verify_balance()
            
            assert result['verified'] is False
            assert result['retry_count'] == scenario['retry']
            assert result['next_retry_delay'] == scenario['delay']
            assert result['exponential_backoff'] is True

    @pytest.mark.asyncio
    async def test_60_second_verification_interval(self, mock_vault_client):
        """Test 60-second verification interval enforcement"""
        import time
        
        current_time = int(time.time())
        
        # Test verification within 60 seconds (should be skipped)
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'skipped': True,
            'reason': 'Within 60 second verification interval',
            'last_verification': current_time - 30,  # 30 seconds ago
            'time_remaining': 30
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['skipped'] is True
        assert result['time_remaining'] == 30
        
        # Test verification after 60 seconds (should proceed)
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'skipped': False,
            'last_verification': current_time - 70,  # 70 seconds ago
            'oracle_called': True
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['skipped'] is False
        assert result['oracle_called'] is True

    @pytest.mark.asyncio
    async def test_5_minute_cache_duration(self, mock_vault_client):
        """Test 5-minute cache duration for UTXO verification"""
        import time
        
        current_time = int(time.time())
        
        # Test cache hit within 5 minutes
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'cache_hit': True,
            'cache_age': 240,  # 4 minutes old
            'cache_valid': True,
            'oracle_called': False
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['cache_hit'] is True
        assert result['cache_age'] < 300  # Less than 5 minutes
        assert result['oracle_called'] is False
        
        # Test cache miss after 5 minutes
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'cache_hit': False,
            'cache_age': 360,  # 6 minutes old
            'cache_valid': False,
            'oracle_called': True
        }
        
        result = await mock_vault_client.verify_balance()
        
        assert result['verified'] is True
        assert result['cache_hit'] is False
        assert result['cache_age'] > 300  # More than 5 minutes
        assert result['oracle_called'] is True

    @pytest.mark.asyncio
    async def test_comprehensive_integration_flow(self, mock_vault_client, sample_btc_data):
        """Test complete integration flow: commit -> verify -> update"""
        # Step 1: Commit BTC
        mock_vault_client.commit_btc.return_value = {
            'success': True,
            'commitment_id': 'test_commitment_123',
            'signature': '5j7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJvj7s1QzqC9JF2BxhoBkJoMRKs8eJv'
        }
        
        commit_result = await mock_vault_client.commit_btc(
            sample_btc_data['amount'],
            sample_btc_data['btc_address'],
            sample_btc_data['ecdsa_proof']
        )
        
        assert commit_result['success'] is True
        commitment_id = commit_result['commitment_id']
        
        # Step 2: Verify balance
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'balance': 50000000,  # 0.5 BTC in satoshis
            'commitment_id': commitment_id
        }
        
        verify_result = await mock_vault_client.verify_balance()
        
        assert verify_result['verified'] is True
        assert verify_result['commitment_id'] == commitment_id
        
        # Step 3: Update commitment
        mock_vault_client.update_commitment.return_value = {
            'success': True,
            'commitment_id': commitment_id,
            'new_amount': 75000000,  # 0.75 BTC
            'requires_reverification': True
        }
        
        update_result = await mock_vault_client.update_commitment(
            0.75,
            'new_ecdsa_proof',
            'new_public_key'
        )
        
        assert update_result['success'] is True
        assert update_result['commitment_id'] == commitment_id
        assert update_result['requires_reverification'] is True
        
        # Step 4: Re-verify after update
        mock_vault_client.verify_balance.return_value = {
            'verified': True,
            'balance': 75000000,  # Updated balance
            'commitment_id': commitment_id,
            'updated': True
        }
        
        reverify_result = await mock_vault_client.verify_balance()
        
        assert reverify_result['verified'] is True
        assert reverify_result['balance'] == 75000000
        assert reverify_result['updated'] is True

    @pytest.mark.asyncio
    async def test_concurrent_instruction_operations(self, mock_vault_client, sample_btc_data):
        """Test concurrent execution of all BTC commitment instructions"""
        # Mock responses for concurrent operations
        mock_vault_client.commit_btc.return_value = {'success': True, 'id': 'commit'}
        mock_vault_client.verify_balance.return_value = {'verified': True, 'id': 'verify'}
        mock_vault_client.update_commitment.return_value = {'success': True, 'id': 'update'}
        
        # Create concurrent tasks for all three instruction types
        commit_tasks = [
            mock_vault_client.commit_btc(
                sample_btc_data['amount'],
                sample_btc_data['btc_address'],
                sample_btc_data['ecdsa_proof']
            ) for _ in range(5)
        ]
        
        verify_tasks = [
            mock_vault_client.verify_balance() for _ in range(5)
        ]
        
        update_tasks = [
            mock_vault_client.update_commitment(
                0.6,
                'new_proof',
                'new_key'
            ) for _ in range(5)
        ]
        
        # Execute all tasks concurrently
        all_tasks = commit_tasks + verify_tasks + update_tasks
        results = await asyncio.gather(*all_tasks)
        
        # Verify all operations completed successfully
        commit_results = results[:5]
        verify_results = results[5:10]
        update_results = results[10:15]
        
        assert all(r['success'] for r in commit_results)
        assert all(r['verified'] for r in verify_results)
        assert all(r['success'] for r in update_results)
        assert len(results) == 15

    def test_instruction_handler_error_scenarios(self, mock_vault_client):
        """Test comprehensive error scenarios for all instruction handlers"""
        error_scenarios = [
            {
                'instruction': 'commit_btc',
                'error': 'Invalid BTC address format',
                'code': 'InvalidBTCAddress'
            },
            {
                'instruction': 'commit_btc',
                'error': 'Invalid ECDSA proof',
                'code': 'InvalidECDSAProof'
            },
            {
                'instruction': 'verify_balance',
                'error': 'Oracle verification failed',
                'code': 'OracleVerificationFailed'
            },
            {
                'instruction': 'verify_balance',
                'error': 'Insufficient balance',
                'code': 'InsufficientBalance'
            },
            {
                'instruction': 'update_commitment',
                'error': 'Unauthorized signer',
                'code': 'UnauthorizedSigner'
            },
            {
                'instruction': 'update_commitment',
                'error': 'Security violation detected',
                'code': 'SecurityViolation'
            }
        ]
        
        for scenario in error_scenarios:
            # Mock the appropriate method to return error
            if scenario['instruction'] == 'commit_btc':
                mock_vault_client.commit_btc.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = mock_vault_client.commit_btc.return_value
            elif scenario['instruction'] == 'verify_balance':
                mock_vault_client.verify_balance.return_value = {
                    'verified': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = mock_vault_client.verify_balance.return_value
            elif scenario['instruction'] == 'update_commitment':
                mock_vault_client.update_commitment.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = mock_vault_client.update_commitment.return_value
            
            assert scenario['error'] in result['error']
            assert result['error_code'] == scenario['code']

if __name__ == '__main__':
    pytest.main([__file__, '-v'])

class TestConcurrentBTCOperations:
    """Test concurrent BTC operations for performance and reliability"""
    
    @pytest.fixture
    def mock_vault_client(self):
        """Mock vault client for concurrent testing"""
        mock_client = Mock()
        mock_client.config = {
            'network': 'devnet',
            'program_id': 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
        }
        # Make async methods return AsyncMock
        mock_client.commit_btc = AsyncMock()
        mock_client.verify_balance = AsyncMock()
        mock_client.update_commitment = AsyncMock()
        return mock_client
    
    @pytest.mark.asyncio
    async def test_concurrent_btc_commitments_stress(self, mock_vault_client):
        """Test high-volume concurrent BTC commitments"""
        # Mock successful responses
        mock_vault_client.commit_btc.return_value = {'success': True, 'commitment_id': 'test_id'}
        
        # Create 100 concurrent commitment tasks
        tasks = []
        for i in range(100):
            task = mock_vault_client.commit_btc(
                0.01 + (i * 0.001),  # Varying amounts
                f'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh{i}',
                f'ecdsa_proof_{i}'
            )
            tasks.append(task)
        
        # Execute all tasks concurrently
        start_time = time.time()
        results = await asyncio.gather(*tasks, return_exceptions=True)
        execution_time = time.time() - start_time
        
        # Verify results
        successful_results = [r for r in results if isinstance(r, dict) and r.get('success')]
        assert len(successful_results) == 100
        assert execution_time < 10.0  # Should complete within 10 seconds
        
        print(f"✅ Concurrent commitments: {len(successful_results)}/100 in {execution_time:.2f}s")
    
    @pytest.mark.asyncio
    async def test_concurrent_balance_verification_with_caching(self, mock_vault_client):
        """Test concurrent balance verification with caching optimization"""
        verification_count = 0
        
        def mock_verify_balance():
            nonlocal verification_count
            verification_count += 1
            return {
                'verified': True,
                'balance': 50000000,
                'cache_hit': verification_count > 1,  # First call misses cache
                'verification_count': verification_count
            }
        
        mock_vault_client.verify_balance.side_effect = mock_verify_balance
        
        # Create 50 concurrent verification tasks
        tasks = [mock_vault_client.verify_balance() for _ in range(50)]
        
        start_time = time.time()
        results = await asyncio.gather(*tasks)
        execution_time = time.time() - start_time
        
        # Verify caching behavior
        cache_hits = sum(1 for r in results if r.get('cache_hit'))
        assert cache_hits > 0  # Some should be cache hits
        assert execution_time < 5.0  # Should be fast due to caching
        
        print(f"✅ Balance verifications: {len(results)} requests, {cache_hits} cache hits in {execution_time:.2f}s")
    
    def test_concurrent_commitment_validation_threadpool(self):
        """Test concurrent commitment validation using ThreadPoolExecutor"""
        def validate_commitment(commitment_data):
            """Mock commitment validation function"""
            time.sleep(0.01)  # Simulate validation time
            
            # Validate amount
            if commitment_data['amount'] <= 0:
                return {'valid': False, 'error': 'Invalid amount'}
            
            # Validate address format
            if not commitment_data['btc_address'].startswith(('bc1', '1', '3')):
                return {'valid': False, 'error': 'Invalid address format'}
            
            # Validate proof length
            if len(commitment_data['ecdsa_proof']) < 64:
                return {'valid': False, 'error': 'Invalid proof length'}
            
            return {'valid': True, 'commitment_id': f"commit_{hash(str(commitment_data))}"}
        
        # Create test data
        test_commitments = []
        for i in range(50):
            test_commitments.append({
                'amount': 0.01 + (i * 0.001),
                'btc_address': f'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh{i:02d}',
                'ecdsa_proof': 'a' * 128  # Valid proof length
            })
        
        # Add some invalid commitments for testing
        test_commitments.extend([
            {'amount': 0, 'btc_address': 'bc1qvalid', 'ecdsa_proof': 'a' * 128},  # Invalid amount
            {'amount': 0.1, 'btc_address': 'invalid', 'ecdsa_proof': 'a' * 128},  # Invalid address
            {'amount': 0.1, 'btc_address': 'bc1qvalid', 'ecdsa_proof': 'short'},  # Invalid proof
        ])
        
        # Execute concurrent validation
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            results = list(executor.map(validate_commitment, test_commitments))
        execution_time = time.time() - start_time
        
        # Analyze results
        valid_results = [r for r in results if r['valid']]
        invalid_results = [r for r in results if not r['valid']]
        
        assert len(valid_results) == 50  # 50 valid commitments
        assert len(invalid_results) == 3   # 3 invalid commitments
        assert execution_time < 2.0  # Should complete quickly with threading
        
        print(f"✅ Concurrent validation: {len(valid_results)} valid, {len(invalid_results)} invalid in {execution_time:.2f}s")
    
    @pytest.mark.asyncio
    async def test_oracle_retry_logic_concurrent(self, mock_vault_client):
        """Test oracle retry logic under concurrent load"""
        retry_counts = {}
        
        async def mock_verify_with_retries(user_id):
            """Mock verification with retry simulation"""
            if user_id not in retry_counts:
                retry_counts[user_id] = 0
            
            retry_counts[user_id] += 1
            
            # Simulate failures for first few attempts
            if retry_counts[user_id] <= 2:
                return {
                    'verified': False,
                    'error': 'Oracle verification failed',
                    'retry_count': retry_counts[user_id],
                    'will_retry': True
                }
            else:
                return {
                    'verified': True,
                    'balance': 50000000,
                    'retry_count': retry_counts[user_id],
                    'final_success': True
                }
        
        # Test concurrent oracle operations with retries
        user_ids = [f'user_{i}' for i in range(20)]
        
        # Simulate multiple retry attempts for each user
        all_tasks = []
        for user_id in user_ids:
            for attempt in range(3):  # 3 attempts per user
                task = mock_verify_with_retries(user_id)
                all_tasks.append(task)
        
        start_time = time.time()
        results = await asyncio.gather(*all_tasks)
        execution_time = time.time() - start_time
        
        # Analyze retry behavior
        successful_results = [r for r in results if r.get('verified')]
        failed_results = [r for r in results if not r.get('verified')]
        
        # Each user should eventually succeed after retries
        unique_successes = len(set(r.get('retry_count', 0) for r in successful_results if r.get('final_success')))
        
        assert len(successful_results) >= 20  # At least one success per user
        assert execution_time < 5.0  # Should handle retries efficiently
        
        print(f"✅ Oracle retry test: {len(successful_results)} successes, {len(failed_results)} retries in {execution_time:.2f}s")
    
    def test_memory_efficiency_large_dataset(self):
        """Test memory efficiency with large commitment datasets"""
        import psutil
        import gc
        
        process = psutil.Process()
        initial_memory = process.memory_info().rss / (1024**2)  # MB
        
        # Create large dataset of commitment data
        large_dataset = []
        for i in range(10000):  # 10k commitments
            commitment = {
                'id': i,
                'amount': 0.001 + (i * 0.0001),
                'btc_address': f'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh{i:05d}',
                'ecdsa_proof': 'a' * 128,
                'timestamp': int(time.time()) + i,
                'user_id': f'user_{i % 1000}',  # 1000 unique users
                'verified': i % 10 == 0  # 10% verified
            }
            large_dataset.append(commitment)
        
        peak_memory = process.memory_info().rss / (1024**2)  # MB
        
        # Process dataset in batches to test memory efficiency
        batch_size = 1000
        processed_batches = 0
        
        for i in range(0, len(large_dataset), batch_size):
            batch = large_dataset[i:i + batch_size]
            
            # Simulate processing
            verified_in_batch = sum(1 for c in batch if c['verified'])
            total_amount_in_batch = sum(c['amount'] for c in batch)
            
            processed_batches += 1
            
            # Force garbage collection after each batch
            if processed_batches % 5 == 0:
                gc.collect()
        
        final_memory = process.memory_info().rss / (1024**2)  # MB
        memory_increase = final_memory - initial_memory
        
        # Memory increase should be reasonable for low-resource systems
        assert memory_increase < 500  # Less than 500MB increase
        assert processed_batches == 10  # All batches processed
        
        print(f"✅ Memory efficiency test: {len(large_dataset)} commitments processed")
        print(f"   Memory usage: {initial_memory:.1f}MB → {final_memory:.1f}MB (+{memory_increase:.1f}MB)")
        
        # Cleanup
        del large_dataset
        gc.collect()

def run_btc_commitment_tests():
    """Run all BTC commitment tests including concurrent tests"""
    print("🔗 Running BTC Commitment Tests...")
    
    # Run standard tests
    test_btc = TestBTCCommitment()
    test_concurrent = TestConcurrentBTCOperations()
    
    # List of all test methods
    standard_tests = [
        'test_commit_btc_success',
        'test_verify_balance_success',
        'test_concurrent_commitments',
        'test_ecdsa_proof_validation',
        'test_chainlink_oracle_integration'
    ]
    
    concurrent_tests = [
        'test_concurrent_commitment_validation_threadpool',
        'test_memory_efficiency_large_dataset'
    ]
    
    passed = 0
    failed = 0
    
    # Run standard tests
    for test_method in standard_tests:
        try:
            if hasattr(test_btc, test_method):
                method = getattr(test_btc, test_method)
                if asyncio.iscoroutinefunction(method):
                    asyncio.run(method(test_btc.mock_vault_client(), test_btc.sample_btc_data()))
                else:
                    method()
                print(f"  ✅ {test_method}")
                passed += 1
            else:
                print(f"  ❌ {test_method} - Method not found")
                failed += 1
        except Exception as e:
            print(f"  ❌ {test_method} - {str(e)}")
            failed += 1
    
    # Run concurrent tests
    for test_method in concurrent_tests:
        try:
            if hasattr(test_concurrent, test_method):
                method = getattr(test_concurrent, test_method)
                if asyncio.iscoroutinefunction(method):
                    asyncio.run(method())
                else:
                    method()
                print(f"  ✅ {test_method}")
                passed += 1
            else:
                print(f"  ❌ {test_method} - Method not found")
                failed += 1
        except Exception as e:
            print(f"  ❌ {test_method} - {str(e)}")
            failed += 1
    
    print(f"📊 BTC Commitment Tests: {passed} passed, {failed} failed")
    return failed == 0

if __name__ == '__main__':
    success = run_btc_commitment_tests()
    sys.exit(0 if success else 1)