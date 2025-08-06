#!/usr/bin/env python3
"""
Treasury Management System Tests

This module contains comprehensive tests for the advanced treasury management system,
including yield farming strategies, liquidity management, and governance features.
"""

import pytest
import asyncio
from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum
import json
import time

class StrategyType(Enum):
    LIQUIDITY_PROVISION = "LiquidityProvision"
    LENDING = "Lending"
    LIQUID_STAKING = "LiquidStaking"
    YIELD_FARMING = "YieldFarming"
    ARBITRAGE = "Arbitrage"
    MARKET_MAKING = "MarketMaking"

class StrategyStatus(Enum):
    ACTIVE = "Active"
    PAUSED = "Paused"
    UNWINDING = "Unwinding"
    COMPLETED = "Completed"
    FAILED = "Failed"

class PoolStatus(Enum):
    ACTIVE = "Active"
    PAUSED = "Paused"
    WITHDRAWING = "Withdrawing"
    CLOSED = "Closed"

class ProposalType(Enum):
    ADD_STRATEGY = "AddStrategy"
    REMOVE_STRATEGY = "RemoveStrategy"
    RISK_PARAMETERS = "RiskParameters"
    EMERGENCY_ACTION = "EmergencyAction"
    FEE_CHANGE = "FeeChange"
    GOVERNANCE_CHANGE = "GovernanceChange"

class ProposalStatus(Enum):
    ACTIVE = "Active"
    APPROVED = "Approved"
    REJECTED = "Rejected"
    EXECUTED = "Executed"
    CANCELLED = "Cancelled"
    EXPIRED = "Expired"

@dataclass
class YieldStrategy:
    strategy_id: int
    name: str
    protocol: str
    strategy_type: StrategyType
    assets: List[str]
    allocated_amount: int  # USD in micro-dollars
    expected_apy: int  # Basis points
    current_apy: int
    risk_level: int  # 1-10
    status: StrategyStatus
    total_returns: int
    daily_returns: int
    weekly_returns: int
    monthly_returns: int
    max_drawdown: int
    successful_operations: int
    failed_operations: int
    created_at: int
    updated_at: int

@dataclass
class LiquidityPoolInfo:
    pool_id: str
    dex_protocol: str
    token_a: str
    token_b: str
    liquidity_provided: int  # USD in micro-dollars
    pool_share: int  # Basis points
    fees_earned: int
    impermanent_loss: int
    status: PoolStatus
    created_at: int

@dataclass
class RiskParameters:
    max_single_strategy_allocation: int  # Basis points
    max_high_risk_allocation: int
    max_daily_loss: int
    max_monthly_loss: int
    min_liquidity_ratio: int
    max_leverage: int
    var_limit: int  # USD in micro-dollars
    risk_monitoring_enabled: bool

@dataclass
class PerformanceMetrics:
    total_returns: int
    annualized_return: int
    volatility: int
    sharpe_ratio: int
    max_drawdown: int
    win_rate: int
    avg_holding_period: int
    total_fees_paid: int
    net_profit: int
    last_calculated: int

@dataclass
class TreasuryProposal:
    proposal_id: int
    proposer: str
    title: str
    description: str
    proposal_type: ProposalType
    voting_start: int
    voting_end: int
    votes_for: int
    votes_against: int
    total_voting_power: int
    quorum_threshold: int
    approval_threshold: int
    status: ProposalStatus

class MockTreasuryVault:
    """Mock implementation of the treasury vault for testing"""
    
    def __init__(self):
        self.treasury = "treasury_pubkey"
        self.authority = "authority_pubkey"
        self.multisig_wallet = "multisig_pubkey"
        self.total_yield_value = 0
        self.yield_strategies: List[YieldStrategy] = []
        self.liquidity_pools: List[LiquidityPoolInfo] = []
        self.risk_parameters = RiskParameters(
            max_single_strategy_allocation=2000,  # 20%
            max_high_risk_allocation=1500,  # 15%
            max_daily_loss=300,  # 3%
            max_monthly_loss=1000,  # 10%
            min_liquidity_ratio=1000,  # 10%
            max_leverage=200,  # 2x
            var_limit=500_000_000,  # $500k
            risk_monitoring_enabled=True
        )
        self.performance_metrics = PerformanceMetrics(
            total_returns=0,
            annualized_return=0,
            volatility=0,
            sharpe_ratio=0,
            max_drawdown=0,
            win_rate=0,
            avg_holding_period=0,
            total_fees_paid=0,
            net_profit=0,
            last_calculated=int(time.time())
        )
        self.emergency_pause = False
        self.created_at = int(time.time())
        self.updated_at = int(time.time())

    def add_yield_strategy(self, strategy: YieldStrategy) -> bool:
        """Add a new yield strategy with validation"""
        if len(self.yield_strategies) >= 20:
            raise ValueError("Too many yield strategies")
        
        if strategy.risk_level > 10:
            raise ValueError("Invalid risk level")
        
        # Check risk limits
        high_risk_allocation = self._calculate_high_risk_allocation(strategy)
        if high_risk_allocation > self.risk_parameters.max_high_risk_allocation:
            raise ValueError("Risk limit exceeded")
        
        self.yield_strategies.append(strategy)
        self.total_yield_value += strategy.allocated_amount
        self.updated_at = int(time.time())
        return True

    def add_liquidity_pool(self, pool_info: LiquidityPoolInfo) -> bool:
        """Add a new liquidity pool"""
        if len(self.liquidity_pools) >= 10:
            raise ValueError("Too many liquidity pools")
        
        self.liquidity_pools.append(pool_info)
        self.updated_at = int(time.time())
        return True

    def needs_rebalancing(self) -> bool:
        """Check if rebalancing is needed"""
        # Check performance-based triggers
        for strategy in self.yield_strategies:
            if strategy.status == StrategyStatus.ACTIVE:
                performance_gap = strategy.expected_apy - strategy.current_apy
                if performance_gap > 500:  # 5% underperformance
                    return True
        
        return False

    def _calculate_high_risk_allocation(self, new_strategy: YieldStrategy) -> int:
        """Calculate high-risk allocation percentage"""
        total_high_risk = sum(
            s.allocated_amount for s in self.yield_strategies 
            if s.risk_level >= 7
        )
        
        if new_strategy.risk_level >= 7:
            total_high_risk += new_strategy.allocated_amount
        
        total_value = self.total_yield_value + new_strategy.allocated_amount
        
        if total_value == 0:
            return 0
        
        return int((total_high_risk * 10000) / total_value)

class MockTreasuryProposalManager:
    """Mock implementation of treasury proposal management"""
    
    def __init__(self):
        self.proposals: Dict[int, TreasuryProposal] = {}
        self.next_proposal_id = 1

    def create_proposal(
        self,
        proposer: str,
        title: str,
        description: str,
        proposal_type: ProposalType,
        voting_duration: int,
        quorum_threshold: int = 2500,  # 25%
        approval_threshold: int = 5000  # 50%
    ) -> TreasuryProposal:
        """Create a new treasury proposal"""
        if len(title) > 100:
            raise ValueError("Title too long")
        if len(description) > 1000:
            raise ValueError("Description too long")
        if voting_duration <= 0 or voting_duration > 2_592_000:  # Max 30 days
            raise ValueError("Invalid voting duration")
        
        current_time = int(time.time())
        proposal = TreasuryProposal(
            proposal_id=self.next_proposal_id,
            proposer=proposer,
            title=title,
            description=description,
            proposal_type=proposal_type,
            voting_start=current_time,
            voting_end=current_time + voting_duration,
            votes_for=0,
            votes_against=0,
            total_voting_power=0,
            quorum_threshold=quorum_threshold,
            approval_threshold=approval_threshold,
            status=ProposalStatus.ACTIVE
        )
        
        self.proposals[self.next_proposal_id] = proposal
        self.next_proposal_id += 1
        return proposal

    def vote_on_proposal(
        self,
        proposal_id: int,
        voter: str,
        vote_for: bool,
        voting_power: int
    ) -> bool:
        """Vote on a treasury proposal"""
        if proposal_id not in self.proposals:
            raise ValueError("Proposal not found")
        
        proposal = self.proposals[proposal_id]
        current_time = int(time.time())
        
        # Check voting period
        if current_time < proposal.voting_start or current_time > proposal.voting_end:
            raise ValueError("Voting period has ended")
        
        if proposal.status != ProposalStatus.ACTIVE:
            raise ValueError("Proposal is not active")
        
        if voting_power <= 0:
            raise ValueError("Insufficient voting power")
        
        # Record vote
        if vote_for:
            proposal.votes_for += voting_power
        else:
            proposal.votes_against += voting_power
        
        proposal.total_voting_power += voting_power
        
        # Check if proposal should be finalized
        total_votes = proposal.votes_for + proposal.votes_against
        quorum_met = (total_votes * 10000) >= (proposal.total_voting_power * proposal.quorum_threshold)
        
        if quorum_met and current_time >= proposal.voting_end:
            approval_rate = (proposal.votes_for * 10000) // total_votes
            if approval_rate >= proposal.approval_threshold:
                proposal.status = ProposalStatus.APPROVED
            else:
                proposal.status = ProposalStatus.REJECTED
        
        return True

class TestTreasuryManagement:
    """Test suite for treasury management system"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.treasury_vault = MockTreasuryVault()
        self.proposal_manager = MockTreasuryProposalManager()

    def test_initialize_treasury_vault(self):
        """Test treasury vault initialization"""
        assert self.treasury_vault.treasury == "treasury_pubkey"
        assert self.treasury_vault.authority == "authority_pubkey"
        assert self.treasury_vault.total_yield_value == 0
        assert len(self.treasury_vault.yield_strategies) == 0
        assert len(self.treasury_vault.liquidity_pools) == 0
        assert not self.treasury_vault.emergency_pause

    def test_add_yield_strategy_success(self):
        """Test successful yield strategy addition"""
        strategy = YieldStrategy(
            strategy_id=1,
            name="Orca LP Strategy",
            protocol="Orca",
            strategy_type=StrategyType.LIQUIDITY_PROVISION,
            assets=["SOL", "USDC"],
            allocated_amount=100_000_000,  # $100k
            expected_apy=1200,  # 12%
            current_apy=0,
            risk_level=5,
            status=StrategyStatus.ACTIVE,
            total_returns=0,
            daily_returns=0,
            weekly_returns=0,
            monthly_returns=0,
            max_drawdown=0,
            successful_operations=0,
            failed_operations=0,
            created_at=int(time.time()),
            updated_at=int(time.time())
        )
        
        result = self.treasury_vault.add_yield_strategy(strategy)
        assert result is True
        assert len(self.treasury_vault.yield_strategies) == 1
        assert self.treasury_vault.total_yield_value == 100_000_000

    def test_add_yield_strategy_risk_validation(self):
        """Test yield strategy risk validation"""
        # Test invalid risk level
        strategy = YieldStrategy(
            strategy_id=1,
            name="High Risk Strategy",
            protocol="Unknown",
            strategy_type=StrategyType.ARBITRAGE,
            assets=["SOL"],
            allocated_amount=50_000_000,
            expected_apy=5000,  # 50%
            current_apy=0,
            risk_level=15,  # Invalid
            status=StrategyStatus.ACTIVE,
            total_returns=0,
            daily_returns=0,
            weekly_returns=0,
            monthly_returns=0,
            max_drawdown=0,
            successful_operations=0,
            failed_operations=0,
            created_at=int(time.time()),
            updated_at=int(time.time())
        )
        
        with pytest.raises(ValueError, match="Invalid risk level"):
            self.treasury_vault.add_yield_strategy(strategy)

    def test_add_yield_strategy_risk_limit_exceeded(self):
        """Test yield strategy risk limit enforcement"""
        # Add multiple high-risk strategies to exceed limit
        for i in range(3):
            strategy = YieldStrategy(
                strategy_id=i + 1,
                name=f"High Risk Strategy {i + 1}",
                protocol="Test",
                strategy_type=StrategyType.ARBITRAGE,
                assets=["SOL"],
                allocated_amount=200_000_000,  # $200k each
                expected_apy=3000,  # 30%
                current_apy=0,
                risk_level=9,  # High risk
                status=StrategyStatus.ACTIVE,
                total_returns=0,
                daily_returns=0,
                weekly_returns=0,
                monthly_returns=0,
                max_drawdown=0,
                successful_operations=0,
                failed_operations=0,
                created_at=int(time.time()),
                updated_at=int(time.time())
            )
            
            if i < 2:  # First two should succeed
                self.treasury_vault.add_yield_strategy(strategy)
            else:  # Third should fail due to risk limits
                with pytest.raises(ValueError, match="Risk limit exceeded"):
                    self.treasury_vault.add_yield_strategy(strategy)

    def test_add_too_many_strategies(self):
        """Test maximum strategy limit enforcement"""
        # Add maximum number of strategies
        for i in range(20):
            strategy = YieldStrategy(
                strategy_id=i + 1,
                name=f"Strategy {i + 1}",
                protocol="Test",
                strategy_type=StrategyType.LENDING,
                assets=["SOL"],
                allocated_amount=10_000_000,  # $10k each
                expected_apy=800,  # 8%
                current_apy=0,
                risk_level=3,  # Low risk
                status=StrategyStatus.ACTIVE,
                total_returns=0,
                daily_returns=0,
                weekly_returns=0,
                monthly_returns=0,
                max_drawdown=0,
                successful_operations=0,
                failed_operations=0,
                created_at=int(time.time()),
                updated_at=int(time.time())
            )
            self.treasury_vault.add_yield_strategy(strategy)
        
        # 21st strategy should fail
        extra_strategy = YieldStrategy(
            strategy_id=21,
            name="Extra Strategy",
            protocol="Test",
            strategy_type=StrategyType.LENDING,
            assets=["SOL"],
            allocated_amount=10_000_000,
            expected_apy=800,
            current_apy=0,
            risk_level=3,
            status=StrategyStatus.ACTIVE,
            total_returns=0,
            daily_returns=0,
            weekly_returns=0,
            monthly_returns=0,
            max_drawdown=0,
            successful_operations=0,
            failed_operations=0,
            created_at=int(time.time()),
            updated_at=int(time.time())
        )
        
        with pytest.raises(ValueError, match="Too many yield strategies"):
            self.treasury_vault.add_yield_strategy(extra_strategy)

    def test_add_liquidity_pool_success(self):
        """Test successful liquidity pool addition"""
        pool_info = LiquidityPoolInfo(
            pool_id="pool_123",
            dex_protocol="Orca",
            token_a="SOL",
            token_b="USDC",
            liquidity_provided=50_000_000,  # $50k
            pool_share=250,  # 2.5%
            fees_earned=0,
            impermanent_loss=0,
            status=PoolStatus.ACTIVE,
            created_at=int(time.time())
        )
        
        result = self.treasury_vault.add_liquidity_pool(pool_info)
        assert result is True
        assert len(self.treasury_vault.liquidity_pools) == 1

    def test_add_too_many_liquidity_pools(self):
        """Test maximum liquidity pool limit enforcement"""
        # Add maximum number of pools
        for i in range(10):
            pool_info = LiquidityPoolInfo(
                pool_id=f"pool_{i}",
                dex_protocol="Orca",
                token_a="SOL",
                token_b="USDC",
                liquidity_provided=10_000_000,
                pool_share=100,
                fees_earned=0,
                impermanent_loss=0,
                status=PoolStatus.ACTIVE,
                created_at=int(time.time())
            )
            self.treasury_vault.add_liquidity_pool(pool_info)
        
        # 11th pool should fail
        extra_pool = LiquidityPoolInfo(
            pool_id="pool_extra",
            dex_protocol="Orca",
            token_a="SOL",
            token_b="USDC",
            liquidity_provided=10_000_000,
            pool_share=100,
            fees_earned=0,
            impermanent_loss=0,
            status=PoolStatus.ACTIVE,
            created_at=int(time.time())
        )
        
        with pytest.raises(ValueError, match="Too many liquidity pools"):
            self.treasury_vault.add_liquidity_pool(extra_pool)

    def test_rebalancing_trigger(self):
        """Test rebalancing trigger conditions"""
        # Initially no rebalancing needed
        assert not self.treasury_vault.needs_rebalancing()
        
        # Add underperforming strategy
        strategy = YieldStrategy(
            strategy_id=1,
            name="Underperforming Strategy",
            protocol="Test",
            strategy_type=StrategyType.LENDING,
            assets=["SOL"],
            allocated_amount=100_000_000,
            expected_apy=1000,  # 10%
            current_apy=400,  # 4% (6% underperformance)
            risk_level=5,
            status=StrategyStatus.ACTIVE,
            total_returns=0,
            daily_returns=0,
            weekly_returns=0,
            monthly_returns=0,
            max_drawdown=0,
            successful_operations=0,
            failed_operations=0,
            created_at=int(time.time()),
            updated_at=int(time.time())
        )
        
        self.treasury_vault.add_yield_strategy(strategy)
        
        # Now rebalancing should be needed
        assert self.treasury_vault.needs_rebalancing()

    def test_create_treasury_proposal_success(self):
        """Test successful treasury proposal creation"""
        proposal = self.proposal_manager.create_proposal(
            proposer="proposer_pubkey",
            title="Add New Yield Strategy",
            description="Proposal to add a new yield farming strategy on Raydium",
            proposal_type=ProposalType.ADD_STRATEGY,
            voting_duration=604800,  # 7 days
            quorum_threshold=2500,  # 25%
            approval_threshold=5000  # 50%
        )
        
        assert proposal.proposal_id == 1
        assert proposal.title == "Add New Yield Strategy"
        assert proposal.status == ProposalStatus.ACTIVE
        assert proposal.votes_for == 0
        assert proposal.votes_against == 0

    def test_create_proposal_validation(self):
        """Test proposal creation validation"""
        # Test title too long
        with pytest.raises(ValueError, match="Title too long"):
            self.proposal_manager.create_proposal(
                proposer="proposer_pubkey",
                title="A" * 101,  # Too long
                description="Valid description",
                proposal_type=ProposalType.ADD_STRATEGY,
                voting_duration=604800
            )
        
        # Test description too long
        with pytest.raises(ValueError, match="Description too long"):
            self.proposal_manager.create_proposal(
                proposer="proposer_pubkey",
                title="Valid title",
                description="A" * 1001,  # Too long
                proposal_type=ProposalType.ADD_STRATEGY,
                voting_duration=604800
            )
        
        # Test invalid voting duration
        with pytest.raises(ValueError, match="Invalid voting duration"):
            self.proposal_manager.create_proposal(
                proposer="proposer_pubkey",
                title="Valid title",
                description="Valid description",
                proposal_type=ProposalType.ADD_STRATEGY,
                voting_duration=2_592_001  # Too long (> 30 days)
            )

    def test_vote_on_proposal_success(self):
        """Test successful proposal voting"""
        # Create proposal
        proposal = self.proposal_manager.create_proposal(
            proposer="proposer_pubkey",
            title="Test Proposal",
            description="Test proposal for voting",
            proposal_type=ProposalType.RISK_PARAMETERS,
            voting_duration=604800
        )
        
        # Vote on proposal
        result = self.proposal_manager.vote_on_proposal(
            proposal_id=proposal.proposal_id,
            voter="voter1_pubkey",
            vote_for=True,
            voting_power=1000
        )
        
        assert result is True
        updated_proposal = self.proposal_manager.proposals[proposal.proposal_id]
        assert updated_proposal.votes_for == 1000
        assert updated_proposal.votes_against == 0
        assert updated_proposal.total_voting_power == 1000

    def test_vote_validation(self):
        """Test voting validation"""
        # Create proposal
        proposal = self.proposal_manager.create_proposal(
            proposer="proposer_pubkey",
            title="Test Proposal",
            description="Test proposal",
            proposal_type=ProposalType.ADD_STRATEGY,
            voting_duration=604800
        )
        
        # Test voting on non-existent proposal
        with pytest.raises(ValueError, match="Proposal not found"):
            self.proposal_manager.vote_on_proposal(
                proposal_id=999,
                voter="voter_pubkey",
                vote_for=True,
                voting_power=1000
            )
        
        # Test insufficient voting power
        with pytest.raises(ValueError, match="Insufficient voting power"):
            self.proposal_manager.vote_on_proposal(
                proposal_id=proposal.proposal_id,
                voter="voter_pubkey",
                vote_for=True,
                voting_power=0
            )

    def test_proposal_approval_process(self):
        """Test complete proposal approval process"""
        # Create proposal with low thresholds for testing
        proposal = self.proposal_manager.create_proposal(
            proposer="proposer_pubkey",
            title="Test Approval",
            description="Test proposal approval process",
            proposal_type=ProposalType.FEE_CHANGE,
            voting_duration=1,  # 1 second for immediate testing
            quorum_threshold=1000,  # 10%
            approval_threshold=5000  # 50%
        )
        
        # Vote with sufficient power to meet quorum and approval
        self.proposal_manager.vote_on_proposal(
            proposal_id=proposal.proposal_id,
            voter="voter1_pubkey",
            vote_for=True,
            voting_power=6000  # 60% approval
        )
        
        self.proposal_manager.vote_on_proposal(
            proposal_id=proposal.proposal_id,
            voter="voter2_pubkey",
            vote_for=False,
            voting_power=4000  # 40% against
        )
        
        # Wait for voting period to end
        time.sleep(2)
        
        # Vote again to trigger finalization
        self.proposal_manager.vote_on_proposal(
            proposal_id=proposal.proposal_id,
            voter="voter3_pubkey",
            vote_for=True,
            voting_power=1
        )
        
        updated_proposal = self.proposal_manager.proposals[proposal.proposal_id]
        assert updated_proposal.status == ProposalStatus.APPROVED

    def test_risk_parameter_validation(self):
        """Test risk parameter validation"""
        # Test valid risk parameters
        valid_params = RiskParameters(
            max_single_strategy_allocation=2000,  # 20%
            max_high_risk_allocation=1000,  # 10%
            max_daily_loss=200,  # 2%
            max_monthly_loss=800,  # 8%
            min_liquidity_ratio=1500,  # 15%
            max_leverage=150,  # 1.5x
            var_limit=300_000_000,  # $300k
            risk_monitoring_enabled=True
        )
        
        self.treasury_vault.risk_parameters = valid_params
        assert self.treasury_vault.risk_parameters.max_single_strategy_allocation == 2000

    def test_performance_metrics_update(self):
        """Test performance metrics updates"""
        new_metrics = PerformanceMetrics(
            total_returns=50_000_000,  # $50k
            annualized_return=1200,  # 12%
            volatility=800,  # 8%
            sharpe_ratio=150,  # 1.5
            max_drawdown=300,  # 3%
            win_rate=7500,  # 75%
            avg_holding_period=30,  # 30 days
            total_fees_paid=2_000_000,  # $2k
            net_profit=48_000_000,  # $48k
            last_calculated=int(time.time())
        )
        
        self.treasury_vault.performance_metrics = new_metrics
        assert self.treasury_vault.performance_metrics.total_returns == 50_000_000
        assert self.treasury_vault.performance_metrics.annualized_return == 1200

    def test_emergency_controls(self):
        """Test emergency control mechanisms"""
        # Initially not paused
        assert not self.treasury_vault.emergency_pause
        
        # Activate emergency pause
        self.treasury_vault.emergency_pause = True
        assert self.treasury_vault.emergency_pause
        
        # Test that operations are blocked during emergency pause
        # This would be implemented in the actual smart contract logic

def run_treasury_management_tests():
    """Run all treasury management tests"""
    print("🏛️ Running Treasury Management System Tests...")
    
    # Create test instance
    test_suite = TestTreasuryManagement()
    
    # List of test methods
    test_methods = [
        "test_initialize_treasury_vault",
        "test_add_yield_strategy_success",
        "test_add_yield_strategy_risk_validation",
        "test_add_yield_strategy_risk_limit_exceeded",
        "test_add_too_many_strategies",
        "test_add_liquidity_pool_success",
        "test_add_too_many_liquidity_pools",
        "test_rebalancing_trigger",
        "test_create_treasury_proposal_success",
        "test_create_proposal_validation",
        "test_vote_on_proposal_success",
        "test_vote_validation",
        "test_proposal_approval_process",
        "test_risk_parameter_validation",
        "test_performance_metrics_update",
        "test_emergency_controls"
    ]
    
    passed = 0
    failed = 0
    
    for test_method in test_methods:
        try:
            print(f"  Running {test_method}...")
            test_suite.setup_method()
            getattr(test_suite, test_method)()
            print(f"  ✅ {test_method} passed")
            passed += 1
        except Exception as e:
            print(f"  ❌ {test_method} failed: {str(e)}")
            failed += 1
    
    print(f"\n📊 Treasury Management Test Results:")
    print(f"  ✅ Passed: {passed}")
    print(f"  ❌ Failed: {failed}")
    print(f"  📈 Success Rate: {(passed / (passed + failed)) * 100:.1f}%")
    
    if failed == 0:
        print("🎉 All treasury management tests passed!")
        return True
    else:
        print("⚠️ Some treasury management tests failed!")
        return False

if __name__ == "__main__":
    success = run_treasury_management_tests()
    exit(0 if success else 1)