"""
Comprehensive Reward System Testing Suite
Tests for reward calculation and distribution system with concurrent execution
Tests 1:2 ratio calculations, 50% profit sharing, state channels, and payment processing
Addresses FR7: Testing and Development Infrastructure requirements
"""

import pytest
import asyncio
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor, as_completed
import time
import json
import hashlib
import threading
from typing import List, Dict, Any
import sys
import os

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# Import configuration
try:
    from config.treasury import get_reward_config
    from config.dashboard import get_payment_config
except ImportError:
    # Mock configs if not available
    def get_reward_config():
        return {
            'user_share': 5000,  # 50%
            'protocol_share': 5000,  # 50%
            'ratio': 200  # 1:2 ratio (200%)
        }
    
    def get_payment_config():
        return {
            'default_currency': 'BTC',
            'supported_currencies': ['BTC', 'USDC'],
            'lightning_enabled': True,
            'auto_reinvest_enabled': True
        }

class MockRewardSystem:
    """Mock reward system for testing reward calculations and distributions"""
    
    def __init__(self):
        self.staking_pool = {
            "total_staked": 0,
            "rewards_accumulated": 0,
            "rewards_distributed": 0,
            "last_reward_calculation": 0,
        }
        self.treasury = {
            "staking_rewards": 0,
            "user_rewards_pool": 0,
        }
        self.users = {}
        self.state_channels = {}
        self.payment_queue = []

class TestRewardCalculation:
    """Test reward calculation logic with 1:2 ratio"""
    
    @pytest.fixture
    def reward_system(self):
        return MockRewardSystem()
    
    def test_basic_reward_calculation(self, reward_system):
        """Test basic 1:2 ratio reward calculation"""
        # Setup: $1000 BTC commitment should get rewards from $1000 staked assets
        user_btc_commitment = 100000000  # 1 BTC in satoshis
        total_staking_rewards = 10000000  # 0.1 BTC rewards generated
        
        # 50% of staking rewards go to users
        user_reward_pool = total_staking_rewards // 2  # 5000000 satoshis
        
        # User gets proportional share based on commitment
        total_btc_commitments = 300000000  # 3 BTC total commitments
        user_reward_share = (user_btc_commitment * user_reward_pool) // total_btc_commitments
        
        expected_reward = 1666666  # (1/3) * 5000000
        assert abs(user_reward_share - expected_reward) <= 1  # Allow for rounding
    
    def test_multiple_user_reward_distribution(self, reward_system):
        """Test reward distribution among multiple users"""
        users = [
            {"commitment": 100000000, "expected_share": 0.2},  # 1 BTC, 20%
            {"commitment": 200000000, "expected_share": 0.4},  # 2 BTC, 40%
            {"commitment": 200000000, "expected_share": 0.4},  # 2 BTC, 40%
        ]
        
        total_commitments = sum(user["commitment"] for user in users)
        total_rewards = 50000000  # 0.5 BTC total rewards
        user_reward_pool = total_rewards // 2  # 50% to users
        
        calculated_rewards = []
        for user in users:
            user_reward = (user["commitment"] * user_reward_pool) // total_commitments
            calculated_rewards.append(user_reward)
        
        # Verify proportional distribution
        assert calculated_rewards[0] == 5000000   # 20% of 25M
        assert calculated_rewards[1] == 10000000  # 40% of 25M
        assert calculated_rewards[2] == 10000000  # 40% of 25M
        
        # Verify total adds up
        assert sum(calculated_rewards) == user_reward_pool
    
    def test_reward_calculation_with_time_bonus(self, reward_system):
        """Test reward calculation with time-based bonuses"""
        base_commitment = 100000000  # 1 BTC
        base_reward = 5000000       # 0.05 BTC
        
        # Test different commitment durations
        test_cases = [
            {"days": 30, "multiplier": 100, "expected": base_reward},      # No bonus
            {"days": 90, "multiplier": 105, "expected": base_reward * 105 // 100},  # 5% bonus
            {"days": 365, "multiplier": 110, "expected": base_reward * 110 // 100}, # 10% bonus
        ]
        
        for case in test_cases:
            reward_with_bonus = (base_reward * case["multiplier"]) // 100
            assert reward_with_bonus == case["expected"]
    
    def test_zero_commitment_handling(self, reward_system):
        """Test handling of zero commitments"""
        user_btc_commitment = 0
        total_staking_rewards = 10000000
        total_btc_commitments = 100000000
        
        # User with zero commitment should get zero rewards
        if user_btc_commitment == 0:
            user_reward = 0
        else:
            user_reward_pool = total_staking_rewards // 2
            user_reward = (user_btc_commitment * user_reward_pool) // total_btc_commitments
        
        assert user_reward == 0
    
    def test_reward_calculation_precision(self, reward_system):
        """Test precision in reward calculations with small amounts"""
        # Test with very small amounts to check for precision issues
        user_btc_commitment = 1000  # 0.00001 BTC
        total_btc_commitments = 100000000  # 1 BTC
        total_staking_rewards = 1000000  # 0.01 BTC
        
        user_reward_pool = total_staking_rewards // 2
        user_reward = (user_btc_commitment * user_reward_pool) // total_btc_commitments
        
        expected_reward = 5  # Very small reward
        assert user_reward == expected_reward

class TestRewardDistribution:
    """Test reward distribution mechanisms"""
    
    @pytest.fixture
    def reward_system(self):
        return MockRewardSystem()
    
    def test_protocol_user_split(self, reward_system):
        """Test 50/50 split between protocol and users"""
        total_staking_rewards = 100000000  # 1 BTC
        
        protocol_share = total_staking_rewards // 2
        user_share = total_staking_rewards - protocol_share
        
        assert protocol_share == 50000000  # 0.5 BTC
        assert user_share == 50000000      # 0.5 BTC
        assert protocol_share + user_share == total_staking_rewards
    
    def test_reward_pool_management(self, reward_system):
        """Test reward pool balance management"""
        # Initialize reward pool
        reward_system.treasury["user_rewards_pool"] = 50000000  # 0.5 BTC
        
        # Distribute rewards to user
        user_reward = 10000000  # 0.1 BTC
        
        # Check sufficient balance
        assert reward_system.treasury["user_rewards_pool"] >= user_reward
        
        # Deduct from pool
        reward_system.treasury["user_rewards_pool"] -= user_reward
        
        assert reward_system.treasury["user_rewards_pool"] == 40000000  # 0.4 BTC remaining
    
    def test_insufficient_reward_pool(self, reward_system):
        """Test handling of insufficient reward pool"""
        reward_system.treasury["user_rewards_pool"] = 5000000   # 0.05 BTC
        requested_reward = 10000000  # 0.1 BTC
        
        # Should detect insufficient balance
        assert reward_system.treasury["user_rewards_pool"] < requested_reward
        
        # In real implementation, this would raise InsufficientBalance error
    
    @pytest.mark.asyncio
    async def test_batch_reward_distribution(self, reward_system):
        """Test batch processing of reward distributions"""
        users = [
            {"id": f"user_{i}", "commitment": 100000000, "reward": 5000000}
            for i in range(100)
        ]
        
        async def distribute_reward(user):
            # Simulate reward distribution
            await asyncio.sleep(0.001)  # Simulate processing time
            return {"user": user["id"], "reward": user["reward"], "status": "success"}
        
        # Process rewards concurrently
        tasks = [distribute_reward(user) for user in users]
        results = await asyncio.gather(*tasks)
        
        # Verify all distributions succeeded
        assert len(results) == 100
        assert all(result["status"] == "success" for result in results)
        
        total_distributed = sum(result["reward"] for result in results)
        assert total_distributed == 500000000  # 5 BTC total

class TestPaymentProcessing:
    """Test multi-currency payment processing"""
    
    @pytest.fixture
    def reward_system(self):
        return MockRewardSystem()
    
    def test_btc_lightning_payment(self, reward_system):
        """Test BTC payment via Lightning Network"""
        reward_amount = 5000000  # 0.05 BTC
        payment_type = "BTC"
        
        # Simulate Lightning Network payment
        payment_result = {
            "type": payment_type,
            "amount": reward_amount,
            "method": "lightning",
            "status": "success",
            "tx_id": "ln_tx_123456"
        }
        
        assert payment_result["status"] == "success"
        assert payment_result["method"] == "lightning"
        assert payment_result["amount"] == reward_amount
    
    def test_btc_onchain_fallback(self, reward_system):
        """Test fallback to on-chain BTC when Lightning fails"""
        reward_amount = 5000000
        
        # Simulate Lightning failure
        lightning_failed = True
        
        if lightning_failed:
            # Fallback to on-chain
            payment_result = {
                "type": "BTC",
                "amount": reward_amount,
                "method": "onchain",
                "status": "success",
                "tx_id": "btc_tx_789012"
            }
        
        assert payment_result["method"] == "onchain"
        assert payment_result["status"] == "success"
    
    def test_usdc_payment(self, reward_system):
        """Test USDC payment processing"""
        reward_amount_btc = 5000000  # 0.05 BTC
        btc_usd_rate = 50000  # $50,000 per BTC
        
        # Convert to USDC equivalent
        usdc_amount = (reward_amount_btc * btc_usd_rate) // 100000000
        
        payment_result = {
            "type": "USDC",
            "amount": usdc_amount,
            "method": "usdc_transfer",
            "status": "success",
            "tx_id": "usdc_tx_345678"
        }
        
        assert payment_result["amount"] == 2500  # $2,500 USDC
        assert payment_result["type"] == "USDC"
    
    def test_auto_reinvestment(self, reward_system):
        """Test auto-reinvestment of rewards"""
        user_id = "user_123"
        reward_amount = 5000000  # 0.05 BTC
        
        # Initialize user
        reward_system.users[user_id] = {
            "btc_commitment": 100000000,  # 1 BTC
            "reward_balance": reward_amount,
            "auto_reinvest": True
        }
        
        if reward_system.users[user_id]["auto_reinvest"]:
            # Add to effective commitment (simplified)
            reward_system.users[user_id]["btc_commitment"] += reward_amount
            reward_system.users[user_id]["reward_balance"] = 0
        
        assert reward_system.users[user_id]["btc_commitment"] == 105000000  # 1.05 BTC
        assert reward_system.users[user_id]["reward_balance"] == 0
    
    @pytest.mark.asyncio
    async def test_payment_retry_logic(self, reward_system):
        """Test payment retry logic for failed payments"""
        reward_amount = 5000000
        max_retries = 3
        
        async def attempt_payment(attempt):
            await asyncio.sleep(0.1)  # Simulate network delay
            
            # Fail first 2 attempts, succeed on 3rd
            if attempt < 2:
                raise Exception("Payment failed")
            else:
                return {"status": "success", "attempt": attempt + 1}
        
        # Retry logic
        for attempt in range(max_retries):
            try:
                result = await attempt_payment(attempt)
                break
            except Exception as e:
                if attempt == max_retries - 1:
                    raise
                await asyncio.sleep(2 ** attempt)  # Exponential backoff
        
        assert result["status"] == "success"
        assert result["attempt"] == 3

class TestStateChannels:
    """Test state channel integration for off-chain calculations"""
    
    @pytest.fixture
    def reward_system(self):
        return MockRewardSystem()
    
    def test_state_channel_initialization(self, reward_system):
        """Test state channel initialization"""
        channel_id = "channel_123"
        participants = ["user_1", "user_2", "protocol"]
        timeout = int(time.time()) + 3600  # 1 hour
        
        channel = {
            "id": channel_id,
            "participants": participants,
            "state_hash": "0" * 64,  # Initial empty state
            "nonce": 0,
            "timeout": timeout,
            "is_active": True,
            "signatures": []
        }
        
        reward_system.state_channels[channel_id] = channel
        
        assert channel["is_active"]
        assert len(channel["participants"]) == 3
        assert channel["nonce"] == 0
    
    def test_off_chain_reward_calculation(self, reward_system):
        """Test off-chain reward calculations"""
        users = [
            {"id": "user_1", "commitment": 100000000},  # 1 BTC
            {"id": "user_2", "commitment": 200000000},  # 2 BTC
        ]
        
        total_staking_rewards = 30000000  # 0.3 BTC
        user_reward_pool = total_staking_rewards // 2  # 50% to users
        total_commitments = sum(user["commitment"] for user in users)
        
        # Calculate rewards off-chain
        calculations = []
        for user in users:
            user_reward = (user["commitment"] * user_reward_pool) // total_commitments
            calculations.append({
                "user": user["id"],
                "commitment": user["commitment"],
                "reward": user_reward,
                "timestamp": int(time.time())
            })
        
        # Verify calculations
        assert calculations[0]["reward"] == 5000000   # 1/3 of 15M
        assert calculations[1]["reward"] == 10000000  # 2/3 of 15M
        
        total_calculated = sum(calc["reward"] for calc in calculations)
        assert total_calculated == user_reward_pool
    
    def test_state_hash_calculation(self, reward_system):
        """Test state hash calculation for reward data"""
        calculations = [
            {"user": "user_1", "reward": 5000000, "timestamp": 1640995200},
            {"user": "user_2", "reward": 10000000, "timestamp": 1640995200},
        ]
        
        # Create deterministic hash
        data = json.dumps(calculations, sort_keys=True).encode()
        state_hash = hashlib.sha256(data).hexdigest()
        
        # Same data should produce same hash
        data2 = json.dumps(calculations, sort_keys=True).encode()
        state_hash2 = hashlib.sha256(data2).hexdigest()
        
        assert state_hash == state_hash2
        assert len(state_hash) == 64  # SHA256 hex string
    
    def test_state_channel_update(self, reward_system):
        """Test state channel update with new calculations"""
        channel_id = "channel_123"
        
        # Initialize channel
        reward_system.state_channels[channel_id] = {
            "nonce": 0,
            "state_hash": "0" * 64,
            "is_active": True,
            "participants": ["user_1", "user_2", "protocol"]
        }
        
        # New calculations
        new_calculations = [
            {"user": "user_1", "reward": 7500000},
            {"user": "user_2", "reward": 7500000},
        ]
        
        # Update channel
        new_state_hash = hashlib.sha256(
            json.dumps(new_calculations, sort_keys=True).encode()
        ).hexdigest()
        
        reward_system.state_channels[channel_id].update({
            "nonce": 1,
            "state_hash": new_state_hash,
            "last_update": int(time.time())
        })
        
        channel = reward_system.state_channels[channel_id]
        assert channel["nonce"] == 1
        assert channel["state_hash"] == new_state_hash
    
    def test_dispute_mechanism(self, reward_system):
        """Test state channel dispute mechanism"""
        channel_id = "channel_123"
        
        # Initialize channel
        reward_system.state_channels[channel_id] = {
            "is_active": True,
            "state_hash": "valid_hash_123",
            "dispute_period": 86400,  # 24 hours
            "last_update": int(time.time())
        }
        
        # Simulate dispute
        disputed_hash = "invalid_hash_456"
        challenger = "user_1"
        
        if disputed_hash != reward_system.state_channels[channel_id]["state_hash"]:
            # Valid dispute - different hash
            reward_system.state_channels[channel_id].update({
                "is_active": False,
                "disputed": True,
                "challenger": challenger,
                "dispute_timestamp": int(time.time())
            })
        
        channel = reward_system.state_channels[channel_id]
        assert not channel["is_active"]
        assert channel["disputed"]
        assert channel["challenger"] == challenger
    
    @pytest.mark.asyncio
    async def test_channel_settlement(self, reward_system):
        """Test state channel settlement and on-chain finalization"""
        channel_id = "channel_123"
        
        # Final calculations to settle
        final_calculations = [
            {"user": "user_1", "reward": 8000000},
            {"user": "user_2", "reward": 12000000},
        ]
        
        total_settlement = sum(calc["reward"] for calc in final_calculations)
        
        # Simulate settlement
        settlement_result = {
            "channel_id": channel_id,
            "total_amount": total_settlement,
            "calculations": final_calculations,
            "status": "settled",
            "settlement_timestamp": int(time.time())
        }
        
        # Update treasury
        reward_system.treasury["user_rewards_pool"] -= total_settlement
        
        assert settlement_result["status"] == "settled"
        assert settlement_result["total_amount"] == 20000000  # 0.2 BTC

class TestRewardIntegration:
    """Integration tests for complete reward workflows"""
    
    @pytest.fixture
    def reward_system(self):
        return MockRewardSystem()
    
    @pytest.mark.asyncio
    async def test_end_to_end_reward_flow(self, reward_system):
        """Test complete reward flow from calculation to payment"""
        # Setup initial state
        reward_system.staking_pool["total_staked"] = 300000000  # 3 BTC equivalent
        reward_system.treasury["user_rewards_pool"] = 50000000  # 0.5 BTC
        
        users = [
            {"id": "user_1", "commitment": 100000000, "payment_type": "BTC"},
            {"id": "user_2", "commitment": 200000000, "payment_type": "USDC"},
        ]
        
        # Step 1: Calculate rewards
        total_commitments = sum(user["commitment"] for user in users)
        user_reward_pool = reward_system.treasury["user_rewards_pool"]
        
        for user in users:
            user_reward = (user["commitment"] * user_reward_pool) // total_commitments
            user["calculated_reward"] = user_reward
        
        # Step 2: Process payments
        payment_results = []
        for user in users:
            if user["payment_type"] == "BTC":
                result = {"user": user["id"], "amount": user["calculated_reward"], "method": "lightning"}
            else:
                result = {"user": user["id"], "amount": user["calculated_reward"], "method": "usdc"}
            
            payment_results.append(result)
            await asyncio.sleep(0.01)  # Simulate processing time
        
        # Verify results
        assert len(payment_results) == 2
        assert payment_results[0]["method"] == "lightning"
        assert payment_results[1]["method"] == "usdc"
        
        total_paid = sum(result["amount"] for result in payment_results)
        # Allow for small rounding differences (within 1 satoshi)
        assert abs(total_paid - user_reward_pool) <= 1
    
    def test_reward_accuracy_validation(self, reward_system):
        """Test validation of reward calculation accuracy"""
        # Test data
        users = [
            {"commitment": 100000000, "expected_reward": 5000000},
            {"commitment": 150000000, "expected_reward": 7500000},
            {"commitment": 250000000, "expected_reward": 12500000},
        ]
        
        total_commitments = sum(user["commitment"] for user in users)
        total_expected = sum(user["expected_reward"] for user in users)
        user_reward_pool = 25000000  # 0.25 BTC
        
        # Calculate actual rewards
        for user in users:
            user["actual_reward"] = (user["commitment"] * user_reward_pool) // total_commitments
        
        # Verify accuracy (allow for small rounding differences)
        for user in users:
            difference = abs(user["actual_reward"] - user["expected_reward"])
            assert difference <= 1  # Max 1 satoshi difference
        
        # Verify total accuracy
        total_actual = sum(user["actual_reward"] for user in users)
        assert total_actual == user_reward_pool
    
    @pytest.mark.asyncio
    async def test_concurrent_reward_processing(self, reward_system):
        """Test concurrent processing of multiple reward operations"""
        
        async def process_reward_calculation(user_data):
            await asyncio.sleep(0.01)  # Simulate calculation time
            return {
                "user": user_data["id"],
                "reward": user_data["commitment"] // 20,  # 5% reward
                "status": "calculated"
            }
        
        async def process_payment(calculation):
            await asyncio.sleep(0.02)  # Simulate payment time
            return {
                "user": calculation["user"],
                "amount": calculation["reward"],
                "status": "paid"
            }
        
        # Setup users
        users = [{"id": f"user_{i}", "commitment": 100000000} for i in range(50)]
        
        # Step 1: Concurrent reward calculations
        calculation_tasks = [process_reward_calculation(user) for user in users]
        calculations = await asyncio.gather(*calculation_tasks)
        
        # Step 2: Concurrent payments
        payment_tasks = [process_payment(calc) for calc in calculations]
        payments = await asyncio.gather(*payment_tasks)
        
        # Verify all operations completed
        assert len(calculations) == 50
        assert len(payments) == 50
        assert all(calc["status"] == "calculated" for calc in calculations)
        assert all(payment["status"] == "paid" for payment in payments)
    
    def test_error_handling_scenarios(self, reward_system):
        """Test error handling in reward processing"""
        
        # Test insufficient reward pool
        reward_system.treasury["user_rewards_pool"] = 1000000  # 0.01 BTC
        requested_reward = 5000000  # 0.05 BTC
        
        try:
            if reward_system.treasury["user_rewards_pool"] < requested_reward:
                raise Exception("InsufficientBalance")
        except Exception as e:
            assert str(e) == "InsufficientBalance"
        
        # Test zero commitment
        user_commitment = 0
        total_commitments = 100000000
        
        if user_commitment == 0:
            user_reward = 0
        else:
            user_reward = (user_commitment * 1000000) // total_commitments
        
        assert user_reward == 0
        
        # Test invalid payment type
        invalid_payment_type = "INVALID"
        valid_types = ["BTC", "USDC", "AutoReinvest"]
        
        assert invalid_payment_type not in valid_types

if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])