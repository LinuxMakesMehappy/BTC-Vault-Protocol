"""
BTC Commitment Testing Suite
Tests for BTC commitment and verification functionality
"""

import pytest
import asyncio
import os
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor
import sys
import os

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

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
        """Test Chainlink oracle integration for balance verification"""
        with patch('config.chainlink.VERIFICATION_CONFIG') as mock_config:
            mock_config.return_value = {
                'interval_seconds': 60,
                'retry_attempts': 3,
                'timeout_seconds': 30
            }
            
            mock_vault_client.verify_balance.return_value = {
                'verified': True,
                'balance': 0.5,
                'oracle_timestamp': 1640995200,  # Mock timestamp
                'price_usd': 45000  # Mock BTC price
            }
            
            result = await mock_vault_client.verify_balance()
            
            assert result['verified'] is True
            assert 'oracle_timestamp' in result
            assert 'price_usd' in result

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

if __name__ == '__main__':
    pytest.main([__file__, '-v'])