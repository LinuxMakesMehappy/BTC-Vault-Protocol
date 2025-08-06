#!/usr/bin/env python3
"""
Enhanced State Channel Infrastructure Tests

This module contains comprehensive tests for the enhanced state channel system,
including high-frequency trading, micro-transactions, and dispute resolution.
"""

import pytest
import asyncio
from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum
import json
import time
import random

class ChannelType(Enum):
    PAYMENT = "Payment"
    TRADING = "Trading"
    REWARD = "Reward"
    MULTI_PURPOSE = "MultiPurpose"

class ChannelStatus(Enum):
    INITIALIZING = "Initializing"
    ACTIVE = "Active"
    DISPUTED = "Disputed"
    SETTLING = "Settling"
    CLOSED = "Closed"
    EXPIRED = "Expired"

class ParticipantRole(Enum):
    FULL_PARTICIPANT = "FullParticipant"
    OBSERVER = "Observer"
    OPERATOR = "Operator"
    VALIDATOR = "Validator"

class OperationType(Enum):
    TRANSFER = "Transfer"
    TRADE = "Trade"
    REWARD_DISTRIBUTION = "RewardDistribution"
    BALANCE_UPDATE = "BalanceUpdate"
    CONFIG_UPDATE = "ConfigUpdate"
    BATCH_OPERATION = "BatchOperation"

class HFTOperationType(Enum):
    MARKET_BUY = "MarketBuy"
    MARKET_SELL = "MarketSell"
    LIMIT_BUY = "LimitBuy"
    LIMIT_SELL = "LimitSell"
    CANCEL = "Cancel"
    BATCH = "Batch"

class DisputeType(Enum):
    INVALID_STATE_TRANSITION = "InvalidStateTransition"
    DOUBLE_SPENDING = "DoubleSpending"
    UNAUTHORIZED_OPERATION = "UnauthorizedOperation"
    TIMEOUT_VIOLATION = "TimeoutViolation"
    BALANCE_INCONSISTENCY = "BalanceInconsistency"

class DisputeStatus(Enum):
    OPEN = "Open"
    UNDER_REVIEW = "UnderReview"
    RESOLVED = "Resolved"
    REJECTED = "Rejected"
    TIMED_OUT = "TimedOut"

class ExecutionStatus(Enum):
    PENDING = "Pending"
    COMPLETED = "Completed"
    FAILED = "Failed"
    CANCELLED = "Cancelled"

@dataclass
class ChannelParticipant:
    pubkey: str
    role: ParticipantRole
    weight: int
    is_active: bool
    last_activity: int

@dataclass
class ChannelConfig:
    channel_type: ChannelType
    timeout: int
    dispute_period: int
    challenge_period: int
    min_confirmations: int
    max_batch_size: int
    base_fee: int
    transfer_fee_rate: int
    trade_fee_rate: int
    dispute_fee: int
    max_operation_value: int
    rate_limit: int
    fraud_detection: bool

@dataclass
class ParticipantBalance:
    participant: str
    token_mint: str
    balance: int
    locked_balance: int
    last_updated: int

@dataclass
class HFTOperation:
    id: int
    pair_base: str
    pair_quote: str
    operation_type: HFTOperationType
    amount: int
    price: int
    participant: str
    timestamp: int
    nonce: int

@dataclass
class MicroTransaction:
    id: int
    from_participant: str
    to_participant: str
    token_mint: str
    amount: int
    fee: int
    memo: str
    timestamp: int

@dataclass
class PendingOperation:
    operation_id: int
    operation_type: OperationType
    participants: List[str]
    data: bytes
    required_confirmations: int
    confirmations: List[dict]
    timestamp: int
    expires_at: int

@dataclass
class DisputeInfo:
    dispute_id: int
    challenger: str
    disputed_state: bytes
    evidence: bytes
    dispute_type: DisputeType
    status: DisputeStatus
    challenge_deadline: int
    created_at: int

class MockEnhancedStateChannel:
    """Mock implementation of enhanced state channel for testing"""
    
    def __init__(self):
        self.channel_id = b"test_channel_123456789012345678901234"
        self.participants: List[ChannelParticipant] = []
        self.state_root = b"\x00" * 32
        self.nonce = 0
        self.config = ChannelConfig(
            channel_type=ChannelType.MULTI_PURPOSE,
            timeout=86400,  # 24 hours
            dispute_period=3600,  # 1 hour
            challenge_period=1800,  # 30 minutes
            min_confirmations=2,
            max_batch_size=100,
            base_fee=1000,
            transfer_fee_rate=10,  # 0.1%
            trade_fee_rate=30,  # 0.3%
            dispute_fee=100000,
            max_operation_value=1_000_000_000,  # 1 SOL
            rate_limit=100,  # 100 ops/sec
            fraud_detection=True
        )
        self.status = ChannelStatus.INITIALIZING
        self.balances: List[ParticipantBalance] = []
        self.pending_operations: List[PendingOperation] = []
        self.dispute_info: Optional[DisputeInfo] = None
        self.total_operations = 0
        self.total_volume = 0
        self.created_at = int(time.time())
        self.updated_at = int(time.time())

    def initialize(self, participants: List[ChannelParticipant], config: ChannelConfig) -> bool:
        """Initialize the enhanced state channel"""
        if len(participants) > 10 or len(participants) == 0:
            raise ValueError("Invalid number of participants")
        
        self.participants = participants
        self.config = config
        self.status = ChannelStatus.INITIALIZING
        self.updated_at = int(time.time())
        return True

    def activate(self) -> bool:
        """Activate the channel"""
        if self.status != ChannelStatus.INITIALIZING:
            raise ValueError("Channel not in initializing state")
        
        self.status = ChannelStatus.ACTIVE
        self.updated_at = int(time.time())
        return True

    def is_participant(self, pubkey: str) -> bool:
        """Check if pubkey is an active participant"""
        return any(p.pubkey == pubkey and p.is_active for p in self.participants)

    def get_participant_balance(self, participant: str, token_mint: str) -> int:
        """Get participant balance for specific token"""
        for balance in self.balances:
            if balance.participant == participant and balance.token_mint == token_mint:
                return balance.balance
        return 0

    def update_balance(self, participant: str, token_mint: str, delta: int) -> bool:
        """Update participant balance"""
        for balance in self.balances:
            if balance.participant == participant and balance.token_mint == token_mint:
                if delta < 0 and balance.balance < abs(delta):
                    raise ValueError("Insufficient balance")
                balance.balance += delta
                balance.last_updated = int(time.time())
                return True
        
        # Create new balance entry if doesn't exist
        if delta >= 0:
            self.balances.append(ParticipantBalance(
                participant=participant,
                token_mint=token_mint,
                balance=delta,
                locked_balance=0,
                last_updated=int(time.time())
            ))
            return True
        else:
            raise ValueError("Insufficient balance")

    def process_hft_operation(self, operation: HFTOperation, participant: str) -> bool:
        """Process high-frequency trading operation"""
        if self.status != ChannelStatus.ACTIVE:
            raise ValueError("Channel not active")
        
        if not self.is_participant(participant):
            raise ValueError("Unauthorized participant")
        
        if operation.participant != participant:
            raise ValueError("Operation participant mismatch")
        
        if operation.nonce <= self.nonce:
            raise ValueError("Invalid nonce")
        
        # Check rate limiting (simplified)
        if self.total_operations > 0:
            time_diff = time.time() - self.updated_at
            if time_diff < 1.0 and self.total_operations % self.config.rate_limit == 0:
                raise ValueError("Rate limit exceeded")
        
        # Process operation based on type
        if operation.operation_type in [HFTOperationType.MARKET_BUY, HFTOperationType.MARKET_SELL]:
            self._process_market_order(operation)
        elif operation.operation_type in [HFTOperationType.LIMIT_BUY, HFTOperationType.LIMIT_SELL]:
            self._process_limit_order(operation)
        elif operation.operation_type == HFTOperationType.CANCEL:
            self._process_cancel_order(operation)
        
        self.nonce = operation.nonce
        self.total_operations += 1
        self.total_volume += operation.amount
        self.updated_at = int(time.time())
        
        return True

    def process_micro_transaction(self, transaction: MicroTransaction, participant: str) -> bool:
        """Process micro-transaction"""
        if self.status != ChannelStatus.ACTIVE:
            raise ValueError("Channel not active")
        
        if transaction.from_participant != participant:
            raise ValueError("Unauthorized participant")
        
        if not self.is_participant(transaction.from_participant) or not self.is_participant(transaction.to_participant):
            raise ValueError("Invalid participants")
        
        if transaction.amount <= 0:
            raise ValueError("Invalid amount")
        
        # Check balance
        from_balance = self.get_participant_balance(transaction.from_participant, transaction.token_mint)
        if from_balance < transaction.amount + transaction.fee:
            raise ValueError("Insufficient balance")
        
        # Update balances
        self.update_balance(transaction.from_participant, transaction.token_mint, -(transaction.amount + transaction.fee))
        self.update_balance(transaction.to_participant, transaction.token_mint, transaction.amount)
        
        # Handle fee (simplified - goes to fee recipient)
        if transaction.fee > 0:
            # In real implementation, would go to fee recipient
            pass
        
        self.total_operations += 1
        self.total_volume += transaction.amount
        self.updated_at = int(time.time())
        
        return True

    def add_pending_operation(self, operation: PendingOperation) -> bool:
        """Add operation to pending queue"""
        if len(self.pending_operations) >= 100:
            raise ValueError("Pending operations queue full")
        
        self.pending_operations.append(operation)
        self.updated_at = int(time.time())
        return True

    def confirm_operation(self, operation_id: int, participant: str, signature: bytes) -> bool:
        """Confirm pending operation"""
        operation = None
        for op in self.pending_operations:
            if op.operation_id == operation_id:
                operation = op
                break
        
        if not operation:
            raise ValueError("Operation not found")
        
        if participant not in operation.participants:
            raise ValueError("Unauthorized participant")
        
        # Check if already confirmed
        for confirmation in operation.confirmations:
            if confirmation.get("participant") == participant:
                raise ValueError("Already confirmed")
        
        operation.confirmations.append({
            "participant": participant,
            "signature": signature.hex(),
            "timestamp": int(time.time())
        })
        
        # Execute if enough confirmations
        if len(operation.confirmations) >= operation.required_confirmations:
            self._execute_pending_operation(operation)
            self.pending_operations.remove(operation)
        
        return True

    def initiate_dispute(self, challenger: str, disputed_state: bytes, evidence: bytes, dispute_type: DisputeType) -> bool:
        """Initiate dispute"""
        if not self.is_participant(challenger):
            raise ValueError("Unauthorized challenger")
        
        if self.dispute_info is not None:
            raise ValueError("Dispute already active")
        
        current_time = int(time.time())
        self.dispute_info = DisputeInfo(
            dispute_id=1,
            challenger=challenger,
            disputed_state=disputed_state,
            evidence=evidence,
            dispute_type=dispute_type,
            status=DisputeStatus.OPEN,
            challenge_deadline=current_time + self.config.challenge_period,
            created_at=current_time
        )
        
        self.status = ChannelStatus.DISPUTED
        self.updated_at = current_time
        
        return True

    def resolve_dispute(self, resolution_type: str, winner: Optional[str], penalty: int) -> bool:
        """Resolve active dispute"""
        if self.dispute_info is None:
            raise ValueError("No active dispute")
        
        if self.dispute_info.status not in [DisputeStatus.OPEN, DisputeStatus.UNDER_REVIEW]:
            raise ValueError("Dispute not resolvable")
        
        self.dispute_info.status = DisputeStatus.RESOLVED
        
        # Apply penalty if applicable
        if penalty > 0 and winner:
            # In real implementation, would apply slashing
            pass
        
        self.status = ChannelStatus.ACTIVE
        self.updated_at = int(time.time())
        
        return True

    def close_channel(self) -> bool:
        """Close the channel"""
        if self.status not in [ChannelStatus.ACTIVE, ChannelStatus.DISPUTED]:
            raise ValueError("Cannot close channel in current state")
        
        if len(self.pending_operations) > 0:
            raise ValueError("Cannot close with pending operations")
        
        self.status = ChannelStatus.CLOSED
        self.updated_at = int(time.time())
        
        return True

    def batch_process_hft_operations(self, operations: List[HFTOperation], participant: str) -> List[dict]:
        """Process batch of HFT operations"""
        if len(operations) > self.config.max_batch_size:
            raise ValueError("Batch size exceeds limit")
        
        results = []
        for operation in operations:
            try:
                self.process_hft_operation(operation, participant)
                results.append({
                    "operation_id": operation.id,
                    "status": ExecutionStatus.COMPLETED,
                    "executed_amount": operation.amount,
                    "executed_price": operation.price
                })
            except Exception as e:
                results.append({
                    "operation_id": operation.id,
                    "status": ExecutionStatus.FAILED,
                    "error": str(e)
                })
        
        return results

    # Helper methods
    def _process_market_order(self, operation: HFTOperation):
        """Process market order"""
        # Simplified market order processing
        pass

    def _process_limit_order(self, operation: HFTOperation):
        """Process limit order"""
        # Simplified limit order processing
        pass

    def _process_cancel_order(self, operation: HFTOperation):
        """Process cancel order"""
        # Simplified cancel order processing
        pass

    def _execute_pending_operation(self, operation: PendingOperation):
        """Execute confirmed pending operation"""
        # Simplified operation execution
        self.total_operations += 1

class TestEnhancedStateChannels:
    """Test suite for enhanced state channel system"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.channel = MockEnhancedStateChannel()
        self.participants = [
            ChannelParticipant(
                pubkey="participant_1",
                role=ParticipantRole.FULL_PARTICIPANT,
                weight=100,
                is_active=True,
                last_activity=int(time.time())
            ),
            ChannelParticipant(
                pubkey="participant_2",
                role=ParticipantRole.FULL_PARTICIPANT,
                weight=100,
                is_active=True,
                last_activity=int(time.time())
            ),
            ChannelParticipant(
                pubkey="observer_1",
                role=ParticipantRole.OBSERVER,
                weight=0,
                is_active=True,
                last_activity=int(time.time())
            )
        ]

    def test_channel_initialization(self):
        """Test enhanced state channel initialization"""
        config = ChannelConfig(
            channel_type=ChannelType.TRADING,
            timeout=3600,
            dispute_period=1800,
            challenge_period=900,
            min_confirmations=2,
            max_batch_size=50,
            base_fee=500,
            transfer_fee_rate=5,
            trade_fee_rate=15,
            dispute_fee=50000,
            max_operation_value=500_000_000,
            rate_limit=50,
            fraud_detection=True
        )
        
        result = self.channel.initialize(self.participants, config)
        assert result is True
        assert self.channel.status == ChannelStatus.INITIALIZING
        assert len(self.channel.participants) == 3
        assert self.channel.config.channel_type == ChannelType.TRADING

    def test_channel_activation(self):
        """Test channel activation"""
        self.channel.initialize(self.participants, self.channel.config)
        
        result = self.channel.activate()
        assert result is True
        assert self.channel.status == ChannelStatus.ACTIVE

    def test_participant_validation(self):
        """Test participant validation"""
        self.channel.initialize(self.participants, self.channel.config)
        
        assert self.channel.is_participant("participant_1") is True
        assert self.channel.is_participant("participant_2") is True
        assert self.channel.is_participant("observer_1") is True
        assert self.channel.is_participant("unknown_participant") is False

    def test_balance_management(self):
        """Test participant balance management"""
        self.channel.initialize(self.participants, self.channel.config)
        
        # Initial balance should be 0
        balance = self.channel.get_participant_balance("participant_1", "SOL")
        assert balance == 0
        
        # Update balance
        result = self.channel.update_balance("participant_1", "SOL", 1000000)
        assert result is True
        
        # Check updated balance
        balance = self.channel.get_participant_balance("participant_1", "SOL")
        assert balance == 1000000
        
        # Test insufficient balance
        with pytest.raises(ValueError, match="Insufficient balance"):
            self.channel.update_balance("participant_1", "SOL", -2000000)

    def test_hft_operation_processing(self):
        """Test high-frequency trading operation processing"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Set up initial balance
        self.channel.update_balance("participant_1", "SOL", 10000000)
        
        operation = HFTOperation(
            id=1,
            pair_base="SOL",
            pair_quote="USDC",
            operation_type=HFTOperationType.MARKET_BUY,
            amount=1000000,
            price=50000000,  # $50 in micro-dollars
            participant="participant_1",
            timestamp=int(time.time() * 1000000),  # microseconds
            nonce=1
        )
        
        result = self.channel.process_hft_operation(operation, "participant_1")
        assert result is True
        assert self.channel.nonce == 1
        assert self.channel.total_operations == 1
        assert self.channel.total_volume == 1000000

    def test_hft_operation_validation(self):
        """Test HFT operation validation"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        operation = HFTOperation(
            id=1,
            pair_base="SOL",
            pair_quote="USDC",
            operation_type=HFTOperationType.MARKET_BUY,
            amount=1000000,
            price=50000000,
            participant="participant_1",
            timestamp=int(time.time() * 1000000),
            nonce=1
        )
        
        # Test unauthorized participant
        with pytest.raises(ValueError, match="Unauthorized participant"):
            self.channel.process_hft_operation(operation, "unknown_participant")
        
        # Test participant mismatch
        operation.participant = "participant_2"
        with pytest.raises(ValueError, match="Operation participant mismatch"):
            self.channel.process_hft_operation(operation, "participant_1")

    def test_micro_transaction_processing(self):
        """Test micro-transaction processing"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Set up initial balances
        self.channel.update_balance("participant_1", "SOL", 1000000)
        self.channel.update_balance("participant_2", "SOL", 0)
        
        transaction = MicroTransaction(
            id=1,
            from_participant="participant_1",
            to_participant="participant_2",
            token_mint="SOL",
            amount=100000,
            fee=1000,
            memo="Test micro-transaction",
            timestamp=int(time.time())
        )
        
        result = self.channel.process_micro_transaction(transaction, "participant_1")
        assert result is True
        
        # Check balances after transaction
        from_balance = self.channel.get_participant_balance("participant_1", "SOL")
        to_balance = self.channel.get_participant_balance("participant_2", "SOL")
        
        assert from_balance == 899000  # 1000000 - 100000 - 1000
        assert to_balance == 100000

    def test_micro_transaction_validation(self):
        """Test micro-transaction validation"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        transaction = MicroTransaction(
            id=1,
            from_participant="participant_1",
            to_participant="participant_2",
            token_mint="SOL",
            amount=100000,
            fee=1000,
            memo="Test transaction",
            timestamp=int(time.time())
        )
        
        # Test insufficient balance
        with pytest.raises(ValueError, match="Insufficient balance"):
            self.channel.process_micro_transaction(transaction, "participant_1")
        
        # Test invalid amount
        transaction.amount = 0
        with pytest.raises(ValueError, match="Invalid amount"):
            self.channel.process_micro_transaction(transaction, "participant_1")

    def test_pending_operations(self):
        """Test pending operation management"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        operation = PendingOperation(
            operation_id=1,
            operation_type=OperationType.TRANSFER,
            participants=["participant_1", "participant_2"],
            data=b"transfer_data",
            required_confirmations=2,
            confirmations=[],
            timestamp=int(time.time()),
            expires_at=int(time.time()) + 3600
        )
        
        # Add pending operation
        result = self.channel.add_pending_operation(operation)
        assert result is True
        assert len(self.channel.pending_operations) == 1
        
        # Confirm operation
        signature = b"signature_1" * 4  # 64 bytes
        result = self.channel.confirm_operation(1, "participant_1", signature)
        assert result is True
        
        # Second confirmation should execute the operation
        signature2 = b"signature_2" * 4
        result = self.channel.confirm_operation(1, "participant_2", signature2)
        assert result is True
        assert len(self.channel.pending_operations) == 0  # Should be executed and removed

    def test_dispute_mechanism(self):
        """Test dispute initiation and resolution"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Initiate dispute
        disputed_state = b"invalid_state" * 2  # 32 bytes
        evidence = b"fraud_evidence"
        
        result = self.channel.initiate_dispute(
            "participant_1",
            disputed_state,
            evidence,
            DisputeType.INVALID_STATE_TRANSITION
        )
        
        assert result is True
        assert self.channel.status == ChannelStatus.DISPUTED
        assert self.channel.dispute_info is not None
        assert self.channel.dispute_info.challenger == "participant_1"
        assert self.channel.dispute_info.dispute_type == DisputeType.INVALID_STATE_TRANSITION

    def test_dispute_resolution(self):
        """Test dispute resolution"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Initiate dispute first
        self.channel.initiate_dispute(
            "participant_1",
            b"disputed_state" * 2,
            b"evidence",
            DisputeType.DOUBLE_SPENDING
        )
        
        # Resolve dispute
        result = self.channel.resolve_dispute("ChallengerWins", "participant_1", 10000)
        assert result is True
        assert self.channel.status == ChannelStatus.ACTIVE
        assert self.channel.dispute_info.status == DisputeStatus.RESOLVED

    def test_batch_hft_processing(self):
        """Test batch processing of HFT operations"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Set up balance
        self.channel.update_balance("participant_1", "SOL", 10000000)
        
        # Create batch of operations
        operations = []
        for i in range(5):
            operations.append(HFTOperation(
                id=i + 1,
                pair_base="SOL",
                pair_quote="USDC",
                operation_type=HFTOperationType.MARKET_BUY,
                amount=100000,
                price=50000000,
                participant="participant_1",
                timestamp=int(time.time() * 1000000),
                nonce=i + 1
            ))
        
        results = self.channel.batch_process_hft_operations(operations, "participant_1")
        
        assert len(results) == 5
        assert all(r["status"] == ExecutionStatus.COMPLETED for r in results)
        assert self.channel.total_operations == 5

    def test_batch_size_limit(self):
        """Test batch size limit enforcement"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Create batch exceeding limit
        operations = []
        for i in range(self.channel.config.max_batch_size + 1):
            operations.append(HFTOperation(
                id=i + 1,
                pair_base="SOL",
                pair_quote="USDC",
                operation_type=HFTOperationType.MARKET_BUY,
                amount=100000,
                price=50000000,
                participant="participant_1",
                timestamp=int(time.time() * 1000000),
                nonce=i + 1
            ))
        
        with pytest.raises(ValueError, match="Batch size exceeds limit"):
            self.channel.batch_process_hft_operations(operations, "participant_1")

    def test_channel_closure(self):
        """Test channel closure"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Should be able to close active channel with no pending operations
        result = self.channel.close_channel()
        assert result is True
        assert self.channel.status == ChannelStatus.CLOSED

    def test_channel_closure_with_pending_operations(self):
        """Test channel closure with pending operations"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Add pending operation
        operation = PendingOperation(
            operation_id=1,
            operation_type=OperationType.TRANSFER,
            participants=["participant_1", "participant_2"],
            data=b"data",
            required_confirmations=2,
            confirmations=[],
            timestamp=int(time.time()),
            expires_at=int(time.time()) + 3600
        )
        self.channel.add_pending_operation(operation)
        
        # Should not be able to close with pending operations
        with pytest.raises(ValueError, match="Cannot close with pending operations"):
            self.channel.close_channel()

    def test_invalid_initialization(self):
        """Test invalid channel initialization"""
        # Test too many participants
        too_many_participants = [
            ChannelParticipant(
                pubkey=f"participant_{i}",
                role=ParticipantRole.FULL_PARTICIPANT,
                weight=100,
                is_active=True,
                last_activity=int(time.time())
            ) for i in range(11)
        ]
        
        with pytest.raises(ValueError, match="Invalid number of participants"):
            self.channel.initialize(too_many_participants, self.channel.config)
        
        # Test empty participants
        with pytest.raises(ValueError, match="Invalid number of participants"):
            self.channel.initialize([], self.channel.config)

    def test_operation_nonce_validation(self):
        """Test operation nonce validation"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        operation = HFTOperation(
            id=1,
            pair_base="SOL",
            pair_quote="USDC",
            operation_type=HFTOperationType.MARKET_BUY,
            amount=1000000,
            price=50000000,
            participant="participant_1",
            timestamp=int(time.time() * 1000000),
            nonce=0  # Invalid nonce (should be > current nonce)
        )
        
        with pytest.raises(ValueError, match="Invalid nonce"):
            self.channel.process_hft_operation(operation, "participant_1")

    def test_dispute_validation(self):
        """Test dispute validation"""
        self.channel.initialize(self.participants, self.channel.config)
        self.channel.activate()
        
        # Test unauthorized challenger
        with pytest.raises(ValueError, match="Unauthorized challenger"):
            self.channel.initiate_dispute(
                "unknown_participant",
                b"state" * 8,
                b"evidence",
                DisputeType.INVALID_STATE_TRANSITION
            )
        
        # Initiate valid dispute
        self.channel.initiate_dispute(
            "participant_1",
            b"state" * 8,
            b"evidence",
            DisputeType.INVALID_STATE_TRANSITION
        )
        
        # Test duplicate dispute
        with pytest.raises(ValueError, match="Dispute already active"):
            self.channel.initiate_dispute(
                "participant_2",
                b"state2" * 4,
                b"evidence2",
                DisputeType.DOUBLE_SPENDING
            )

def run_enhanced_state_channel_tests():
    """Run all enhanced state channel tests"""
    print("🔄 Running Enhanced State Channel Infrastructure Tests...")
    
    # Create test instance
    test_suite = TestEnhancedStateChannels()
    
    # List of test methods
    test_methods = [
        "test_channel_initialization",
        "test_channel_activation",
        "test_participant_validation",
        "test_balance_management",
        "test_hft_operation_processing",
        "test_hft_operation_validation",
        "test_micro_transaction_processing",
        "test_micro_transaction_validation",
        "test_pending_operations",
        "test_dispute_mechanism",
        "test_dispute_resolution",
        "test_batch_hft_processing",
        "test_batch_size_limit",
        "test_channel_closure",
        "test_channel_closure_with_pending_operations",
        "test_invalid_initialization",
        "test_operation_nonce_validation",
        "test_dispute_validation"
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
    
    print(f"\n📊 Enhanced State Channel Test Results:")
    print(f"  ✅ Passed: {passed}")
    print(f"  ❌ Failed: {failed}")
    print(f"  📈 Success Rate: {(passed / (passed + failed)) * 100:.1f}%")
    
    if failed == 0:
        print("🎉 All enhanced state channel tests passed!")
        return True
    else:
        print("⚠️ Some enhanced state channel tests failed!")
        return False

if __name__ == "__main__":
    success = run_enhanced_state_channel_tests()
    exit(0 if success else 1)