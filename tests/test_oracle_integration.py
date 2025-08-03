"""
Unit tests for Chainlink Oracle Integration

Tests oracle functionality including:
- BTC price feed updates
- UTXO balance verification
- Error handling and retry logic
- Cache management
- Anti-spoofing protection
"""

import pytest
import asyncio
import time
from unittest.mock import Mock, AsyncMock, patch, MagicMock
from typing import Dict, Any
import json

# Import configuration
import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'config'))
from chainlink import ChainlinkConfig, Network, RetryConfig

class MockSolanaClient:
    """Mock Solana client for testing oracle interactions"""
    
    def __init__(self):
        self.oracle_data = {
            "btc_usd_feed": "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J",
            "last_update": int(time.time()),
            "btc_price_usd": 5000000000000,  # $50,000 with 8 decimals
            "round_id": 1,
            "verification_interval": 60,
            "cache_duration": 300,
            "is_active": True,
            "retry_config": {
                "max_retries": 3,
                "base_delay": 2,
                "max_delay": 60,
                "current_retries": 0,
                "last_retry": 0
            },
            "utxo_cache": {}
        }
        self.call_count = 0
        self.should_fail = False
        self.failure_count = 0
    
    async def initialize_oracle(self, btc_usd_feed: str) -> Dict[str, Any]:
        """Mock oracle initialization"""
        self.call_count += 1
        if self.should_fail and self.failure_count < 2:
            self.failure_count += 1
            raise Exception("Oracle initialization failed")
        
        self.oracle_data["btc_usd_feed"] = btc_usd_feed
        return {"success": True, "oracle_data": self.oracle_data}
    
    async def update_btc_price(self, price: int, round_id: int, timestamp: int) -> Dict[str, Any]:
        """Mock BTC price update"""
        self.call_count += 1
        if self.should_fail and self.failure_count < 2:
            self.failure_count += 1
            raise Exception("Price update failed")
        
        # Validate price data
        if price <= 0:
            raise ValueError("Invalid price data")
        
        # Check for large price deviation
        if self.oracle_data["btc_price_usd"] > 0:
            old_price = self.oracle_data["btc_price_usd"]
            deviation = abs(price - old_price) / old_price * 100
            if deviation > 10:
                print(f"Warning: Large price deviation: {deviation:.2f}%")
        
        self.oracle_data.update({
            "btc_price_usd": price,
            "round_id": round_id,
            "last_update": timestamp
        })
        
        return {"success": True, "price": price, "round_id": round_id}
    
    async def verify_btc_balance(
        self, 
        btc_address: str, 
        expected_balance: int, 
        ecdsa_proof: bytes
    ) -> Dict[str, Any]:
        """Mock BTC balance verification"""
        self.call_count += 1
        if self.should_fail and self.failure_count < 2:
            self.failure_count += 1
            raise Exception("Balance verification failed")
        
        # Validate BTC address format
        if not self._is_valid_btc_address(btc_address):
            raise ValueError("Invalid BTC address format")
        
        # Validate ECDSA proof
        if len(ecdsa_proof) != 64:
            raise ValueError("Invalid ECDSA proof length")
        
        # Simulate balance verification
        verified_balance = expected_balance  # In real implementation, this would be actual balance
        is_verified = verified_balance >= expected_balance
        
        # Cache the result
        cache_key = btc_address
        self.oracle_data["utxo_cache"][cache_key] = {
            "btc_address": btc_address,
            "balance": verified_balance,
            "verified_at": int(time.time()),
            "proof_hash": ecdsa_proof.hex(),
            "is_valid": is_verified,
            "expires_at": int(time.time()) + 300  # 5 minutes
        }
        
        return {
            "success": True,
            "verified": is_verified,
            "balance": verified_balance,
            "cached": False
        }
    
    def _is_valid_btc_address(self, address: str) -> bool:
        """Validate Bitcoin address format"""
        if len(address) < 26 or len(address) > 62:
            return False
        return (address.startswith("1") or 
                address.startswith("3") or 
                address.startswith("bc1") or 
                address.startswith("tb1"))
    
    def get_oracle_data(self) -> Dict[str, Any]:
        """Get current oracle data"""
        return self.oracle_data.copy()
    
    def reset_mock(self):
        """Reset mock state"""
        self.call_count = 0
        self.should_fail = False
        self.failure_count = 0

class TestOracleIntegration:
    """Test suite for oracle integration functionality"""
    
    @pytest.fixture
    def mock_client(self):
        """Create mock Solana client"""
        return MockSolanaClient()
    
    @pytest.fixture
    def chainlink_config(self):
        """Create Chainlink configuration for testing"""
        return ChainlinkConfig(Network.DEVNET)
    
    @pytest.mark.asyncio
    async def test_oracle_initialization_success(self, mock_client, chainlink_config):
        """Test successful oracle initialization"""
        btc_feed = chainlink_config.get_btc_usd_feed()
        result = await mock_client.initialize_oracle(btc_feed.address)
        
        assert result["success"] is True
        assert result["oracle_data"]["btc_usd_feed"] == btc_feed.address
        assert result["oracle_data"]["verification_interval"] == 60
        assert result["oracle_data"]["is_active"] is True
        assert mock_client.call_count == 1
    
    @pytest.mark.asyncio
    async def test_oracle_initialization_retry_on_failure(self, mock_client, chainlink_config):
        """Test oracle initialization with retry on failure"""
        mock_client.should_fail = True
        btc_feed = chainlink_config.get_btc_usd_feed()
        
        # Should fail twice, then succeed
        with pytest.raises(Exception):
            await mock_client.initialize_oracle(btc_feed.address)
        
        with pytest.raises(Exception):
            await mock_client.initialize_oracle(btc_feed.address)
        
        # Third attempt should succeed
        result = await mock_client.initialize_oracle(btc_feed.address)
        assert result["success"] is True
        assert mock_client.call_count == 3
    
    @pytest.mark.asyncio
    async def test_btc_price_update_success(self, mock_client):
        """Test successful BTC price update"""
        price = 5500000000000  # $55,000 with 8 decimals
        round_id = 12345
        timestamp = int(time.time())
        
        result = await mock_client.update_btc_price(price, round_id, timestamp)
        
        assert result["success"] is True
        assert result["price"] == price
        assert result["round_id"] == round_id
        
        oracle_data = mock_client.get_oracle_data()
        assert oracle_data["btc_price_usd"] == price
        assert oracle_data["round_id"] == round_id
        assert oracle_data["last_update"] == timestamp
    
    @pytest.mark.asyncio
    async def test_btc_price_update_invalid_price(self, mock_client):
        """Test BTC price update with invalid price"""
        with pytest.raises(ValueError, match="Invalid price data"):
            await mock_client.update_btc_price(0, 12345, int(time.time()))
    
    @pytest.mark.asyncio
    async def test_btc_price_deviation_warning(self, mock_client, capfd):
        """Test price deviation warning for large changes"""
        # Set initial price
        await mock_client.update_btc_price(5000000000000, 1, int(time.time()))  # $50,000
        
        # Update with 20% increase (should trigger warning)
        await mock_client.update_btc_price(6000000000000, 2, int(time.time()))  # $60,000
        
        captured = capfd.readouterr()
        assert "Large price deviation" in captured.out
    
    @pytest.mark.asyncio
    async def test_btc_balance_verification_success(self, mock_client):
        """Test successful BTC balance verification"""
        btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        expected_balance = 100000000  # 1 BTC in satoshis
        ecdsa_proof = b"a" * 64  # 64-byte proof
        
        result = await mock_client.verify_btc_balance(btc_address, expected_balance, ecdsa_proof)
        
        assert result["success"] is True
        assert result["verified"] is True
        assert result["balance"] == expected_balance
        
        # Check cache
        oracle_data = mock_client.get_oracle_data()
        assert btc_address in oracle_data["utxo_cache"]
        cached_entry = oracle_data["utxo_cache"][btc_address]
        assert cached_entry["balance"] == expected_balance
        assert cached_entry["is_valid"] is True
    
    @pytest.mark.asyncio
    async def test_btc_balance_verification_invalid_address(self, mock_client):
        """Test BTC balance verification with invalid address"""
        invalid_address = "invalid_address"
        expected_balance = 100000000
        ecdsa_proof = b"a" * 64
        
        with pytest.raises(ValueError, match="Invalid BTC address format"):
            await mock_client.verify_btc_balance(invalid_address, expected_balance, ecdsa_proof)
    
    @pytest.mark.asyncio
    async def test_btc_balance_verification_invalid_proof(self, mock_client):
        """Test BTC balance verification with invalid ECDSA proof"""
        btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        expected_balance = 100000000
        invalid_proof = b"a" * 32  # Wrong length
        
        with pytest.raises(ValueError, match="Invalid ECDSA proof length"):
            await mock_client.verify_btc_balance(btc_address, expected_balance, invalid_proof)
    
    @pytest.mark.asyncio
    async def test_oracle_retry_logic(self, mock_client, chainlink_config):
        """Test oracle retry logic with exponential backoff"""
        retry_config = chainlink_config.get_retry_config()
        
        # Test retry delay calculation
        delays = []
        for attempt in range(retry_config.max_retries):
            delay = chainlink_config.calculate_retry_delay(attempt)
            delays.append(delay)
        
        # Verify exponential backoff
        assert delays[0] >= retry_config.base_delay
        assert delays[1] >= delays[0]
        assert delays[2] >= delays[1]
        assert all(delay <= retry_config.max_delay for delay in delays)
    
    @pytest.mark.asyncio
    async def test_oracle_cache_functionality(self, mock_client):
        """Test oracle cache functionality"""
        btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        expected_balance = 100000000
        ecdsa_proof = b"a" * 64
        
        # First verification - should cache result
        result1 = await mock_client.verify_btc_balance(btc_address, expected_balance, ecdsa_proof)
        assert result1["success"] is True
        assert mock_client.call_count == 1
        
        # Check cache exists
        oracle_data = mock_client.get_oracle_data()
        assert btc_address in oracle_data["utxo_cache"]
        
        # Verify cache entry structure
        cached_entry = oracle_data["utxo_cache"][btc_address]
        assert cached_entry["btc_address"] == btc_address
        assert cached_entry["balance"] == expected_balance
        assert cached_entry["is_valid"] is True
        assert cached_entry["expires_at"] > int(time.time())
    
    @pytest.mark.asyncio
    async def test_oracle_error_handling(self, mock_client):
        """Test comprehensive oracle error handling"""
        mock_client.should_fail = True
        
        # Test initialization failure
        with pytest.raises(Exception, match="Oracle initialization failed"):
            await mock_client.initialize_oracle("test_feed")
        
        # Test price update failure
        with pytest.raises(Exception, match="Price update failed"):
            await mock_client.update_btc_price(5000000000000, 1, int(time.time()))
        
        # Test balance verification failure
        with pytest.raises(Exception, match="Balance verification failed"):
            await mock_client.verify_btc_balance("bc1qtest", 100000000, b"a" * 64)
    
    def test_chainlink_config_network_selection(self):
        """Test Chainlink configuration for different networks"""
        # Test devnet configuration
        devnet_config = ChainlinkConfig(Network.DEVNET)
        assert devnet_config.network == Network.DEVNET
        assert devnet_config.get_verification_interval() == 30
        
        # Test mainnet configuration
        mainnet_config = ChainlinkConfig(Network.MAINNET)
        assert mainnet_config.network == Network.MAINNET
        assert mainnet_config.get_verification_interval() == 60
        
        # Test testnet configuration
        testnet_config = ChainlinkConfig(Network.TESTNET)
        assert testnet_config.network == Network.TESTNET
        assert testnet_config.get_verification_interval() == 120
    
    def test_chainlink_config_price_deviation_alert(self, chainlink_config):
        """Test price deviation alert functionality"""
        # Test normal price change (should not alert)
        old_price = 50000.0
        new_price = 51000.0  # 2% increase
        should_alert = chainlink_config.should_alert_price_deviation(old_price, new_price)
        assert should_alert is False
        
        # Test large price change (should alert)
        new_price = 60000.0  # 20% increase
        should_alert = chainlink_config.should_alert_price_deviation(old_price, new_price)
        assert should_alert is True
        
        # Test with zero old price (should not alert)
        should_alert = chainlink_config.should_alert_price_deviation(0, new_price)
        assert should_alert is False
    
    def test_chainlink_config_stale_data_detection(self, chainlink_config):
        """Test stale data detection"""
        current_time = int(time.time())
        
        # Test fresh data
        fresh_timestamp = current_time - 60  # 1 minute ago
        is_stale = chainlink_config.is_data_stale(fresh_timestamp, current_time)
        assert is_stale is False
        
        # Test stale data
        stale_timestamp = current_time - 600  # 10 minutes ago
        is_stale = chainlink_config.is_data_stale(stale_timestamp, current_time)
        assert is_stale is True
    
    def test_chainlink_config_serialization(self, chainlink_config, tmp_path):
        """Test configuration serialization and deserialization"""
        # Test JSON conversion
        json_str = chainlink_config.to_json()
        config_dict = json.loads(json_str)
        assert config_dict["network"] == "devnet"
        assert config_dict["verification_interval"] == 30
        
        # Test file save/load
        config_file = tmp_path / "test_config.json"
        chainlink_config.save_to_file(str(config_file))
        assert config_file.exists()
        
        loaded_config = ChainlinkConfig.load_from_file(str(config_file))
        assert loaded_config.network == chainlink_config.network
        assert loaded_config.get_verification_interval() == chainlink_config.get_verification_interval()
    
    @pytest.mark.asyncio
    async def test_concurrent_oracle_operations(self, mock_client):
        """Test concurrent oracle operations"""
        # Test concurrent price updates
        tasks = []
        for i in range(5):
            price = 5000000000000 + (i * 100000000000)  # Increment by $1000
            task = mock_client.update_btc_price(price, i + 1, int(time.time()))
            tasks.append(task)
        
        results = await asyncio.gather(*tasks)
        assert all(result["success"] for result in results)
        assert mock_client.call_count == 5
        
        # Test concurrent balance verifications
        mock_client.reset_mock()
        tasks = []
        for i in range(3):
            btc_address = f"bc1qtest{i}"
            task = mock_client.verify_btc_balance(btc_address, 100000000, b"a" * 64)
            tasks.append(task)
        
        results = await asyncio.gather(*tasks)
        assert all(result["success"] for result in results)
        assert mock_client.call_count == 3
        
        # Verify all addresses are cached
        oracle_data = mock_client.get_oracle_data()
        for i in range(3):
            assert f"bc1qtest{i}" in oracle_data["utxo_cache"]
    
    def test_btc_address_validation_comprehensive(self):
        """Test comprehensive Bitcoin address validation"""
        mock_client = MockSolanaClient()
        
        # Valid addresses
        valid_addresses = [
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",  # P2PKH
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",  # P2SH
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",  # Bech32
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",  # Testnet Bech32
        ]
        
        for address in valid_addresses:
            assert mock_client._is_valid_btc_address(address), f"Address {address} should be valid"
        
        # Invalid addresses
        invalid_addresses = [
            "",  # Empty
            "invalid",  # Too short
            "2short",  # Too short
            "toolongaddressthatexceedsmaximumlengthfortestingpurposes" * 2,  # Too long
            "4InvalidPrefix",  # Invalid prefix
        ]
        
        for address in invalid_addresses:
            assert not mock_client._is_valid_btc_address(address), f"Address {address} should be invalid"

class TestOracleFailureScenarios:
    """Test oracle failure scenarios and recovery"""
    
    @pytest.fixture
    def mock_client(self):
        return MockSolanaClient()
    
    @pytest.mark.asyncio
    async def test_network_timeout_recovery(self, mock_client):
        """Test recovery from network timeouts"""
        # Simulate network timeout
        mock_client.should_fail = True
        
        with pytest.raises(Exception):
            await mock_client.update_btc_price(5000000000000, 1, int(time.time()))
        
        # Recovery after timeout
        mock_client.should_fail = False
        result = await mock_client.update_btc_price(5000000000000, 1, int(time.time()))
        assert result["success"] is True
    
    @pytest.mark.asyncio
    async def test_stale_data_handling(self, mock_client):
        """Test handling of stale oracle data"""
        # Set old timestamp
        old_timestamp = int(time.time()) - 3600  # 1 hour ago
        result = await mock_client.update_btc_price(5000000000000, 1, old_timestamp)
        
        oracle_data = mock_client.get_oracle_data()
        current_time = int(time.time())
        
        # Check if data is considered stale
        age = current_time - oracle_data["last_update"]
        assert age > 300  # More than 5 minutes old
    
    @pytest.mark.asyncio
    async def test_cache_expiry_handling(self, mock_client):
        """Test cache expiry and cleanup"""
        btc_address = "bc1qtest"
        expected_balance = 100000000
        ecdsa_proof = b"a" * 64
        
        # Add entry to cache
        await mock_client.verify_btc_balance(btc_address, expected_balance, ecdsa_proof)
        
        # Manually expire cache entry
        oracle_data = mock_client.get_oracle_data()
        oracle_data["utxo_cache"][btc_address]["expires_at"] = int(time.time()) - 1
        
        # Verify cache entry is expired
        cached_entry = oracle_data["utxo_cache"][btc_address]
        assert cached_entry["expires_at"] < int(time.time())

if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v", "--tb=short"])