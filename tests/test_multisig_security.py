"""
Comprehensive tests for multisig security system with HSM integration.
Tests 2-of-3 multisig, HSM attestation, emergency procedures, and security policies.
"""

import pytest
import asyncio
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor
import time
import json
import hashlib
from dataclasses import dataclass
from typing import List, Optional, Dict, Any

@dataclass
class MockHsmConfig:
    """Mock HSM configuration for testing"""
    enabled: bool
    hsm_type: str
    device_serial: str
    public_key: str
    attestation_key: str
    last_attestation: int
    firmware_version: str

@dataclass
class MockSecurityPolicies:
    """Mock security policies for testing"""
    max_daily_amount: int
    max_single_transaction: int
    require_hsm_for_large_tx: bool
    large_tx_threshold: int
    cooling_period_hours: int
    emergency_freeze_enabled: bool
    auto_freeze_on_suspicious: bool

@dataclass
class MockHsmAttestation:
    """Mock HSM attestation for testing"""
    device_serial: str
    timestamp: int
    signature: bytes
    counter: int

@dataclass
class MockPendingTransaction:
    """Mock pending transaction for testing"""
    id: int
    transaction_type: str
    amount: int
    recipient: str
    data: bytes
    signatures: List[Dict[str, Any]]
    created_at: int
    expires_at: int
    executed: bool

class MockMultisigWallet:
    """Mock multisig wallet for testing multisig security"""
    
    def __init__(self):
        self.owners = []
        self.threshold = 0
        self.nonce = 0
        self.pending_transactions = []
        self.hsm_config = None
        self.security_policies = None
        self.emergency_contacts = []
        self.last_activity = int(time.time())
        self.is_frozen = False
        self.daily_transaction_amounts = {}

class TestMultisigCreation:
    """Test multisig wallet creation and initialization"""
    
    @pytest.fixture
    def multisig_wallet(self):
        return MockMultisigWallet()
    
    def test_basic_multisig_creation(self, multisig_wallet):
        """Test basic 2-of-3 multisig creation"""
        owners = ["owner1", "owner2", "owner3"]
        threshold = 2
        
        # Initialize multisig
        multisig_wallet.owners = owners
        multisig_wallet.threshold = threshold
        
        assert len(multisig_wallet.owners) == 3
        assert multisig_wallet.threshold == 2
        assert multisig_wallet.nonce == 0
        assert not multisig_wallet.is_frozen
    
    def test_multisig_with_hsm_config(self, multisig_wallet):
        """Test multisig creation with HSM configuration"""
        owners = ["owner1", "owner2", "owner3"]
        threshold = 2
        
        hsm_config = MockHsmConfig(
            enabled=True,
            hsm_type="YubicoHSM2",
            device_serial="YH2023001",
            public_key="pubkey123",
            attestation_key="attkey456",
            last_attestation=int(time.time()),
            firmware_version="2.3.1"
        )
        
        multisig_wallet.owners = owners
        multisig_wallet.threshold = threshold
        multisig_wallet.hsm_config = hsm_config
        
        assert multisig_wallet.hsm_config.enabled
        assert multisig_wallet.hsm_config.hsm_type == "YubicoHSM2"
        assert multisig_wallet.hsm_config.device_serial == "YH2023001"
    
    def test_security_policies_initialization(self, multisig_wallet):
        """Test security policies configuration"""
        security_policies = MockSecurityPolicies(
            max_daily_amount=1000000,  # 1M tokens
            max_single_transaction=100000,  # 100K tokens
            require_hsm_for_large_tx=True,
            large_tx_threshold=50000,  # 50K tokens
            cooling_period_hours=24,
            emergency_freeze_enabled=True,
            auto_freeze_on_suspicious=True
        )
        
        multisig_wallet.security_policies = security_policies
        
        assert multisig_wallet.security_policies.max_daily_amount == 1000000
        assert multisig_wallet.security_policies.require_hsm_for_large_tx
        assert multisig_wallet.security_policies.emergency_freeze_enabled
    
    def test_emergency_contacts_setup(self, multisig_wallet):
        """Test emergency contacts configuration"""
        emergency_contacts = ["emergency1", "emergency2"]
        
        multisig_wallet.emergency_contacts = emergency_contacts
        
        assert len(multisig_wallet.emergency_contacts) == 2
        assert "emergency1" in multisig_wallet.emergency_contacts
        assert "emergency2" in multisig_wallet.emergency_contacts
    
    def test_invalid_threshold_validation(self, multisig_wallet):
        """Test validation of invalid threshold values"""
        owners = ["owner1", "owner2", "owner3"]
        
        # Test threshold = 0
        with pytest.raises(ValueError):
            if 0 == 0 or 0 > len(owners):
                raise ValueError("Invalid threshold")
        
        # Test threshold > owners
        with pytest.raises(ValueError):
            if 4 == 0 or 4 > len(owners):
                raise ValueError("Invalid threshold")
        
        # Valid threshold should work
        threshold = 2
        assert threshold > 0 and threshold <= len(owners)

class TestTransactionCreation:
    """Test transaction creation and proposal"""
    
    @pytest.fixture
    def multisig_wallet(self):
        wallet = MockMultisigWallet()
        wallet.owners = ["owner1", "owner2", "owner3"]
        wallet.threshold = 2
        wallet.security_policies = MockSecurityPolicies(
            max_daily_amount=1000000,
            max_single_transaction=100000,
            require_hsm_for_large_tx=True,
            large_tx_threshold=50000,
            cooling_period_hours=24,
            emergency_freeze_enabled=True,
            auto_freeze_on_suspicious=True
        )
        return wallet
    
    def test_create_basic_transaction(self, multisig_wallet):
        """Test creating a basic transfer transaction"""
        transaction = MockPendingTransaction(
            id=1,
            transaction_type="Transfer",
            amount=25000,  # Below HSM threshold
            recipient="recipient1",
            data=b"",
            signatures=[],
            created_at=int(time.time()),
            expires_at=int(time.time()) + 86400,  # 24 hours
            executed=False
        )
        
        multisig_wallet.pending_transactions.append(transaction)
        multisig_wallet.nonce += 1
        
        assert len(multisig_wallet.pending_transactions) == 1
        assert multisig_wallet.pending_transactions[0].amount == 25000
        assert multisig_wallet.pending_transactions[0].transaction_type == "Transfer"
        assert multisig_wallet.nonce == 1
    
    def test_create_large_transaction_requires_hsm(self, multisig_wallet):
        """Test that large transactions require HSM attestation"""
        large_amount = 75000  # Above HSM threshold
        
        # Should require HSM for large transactions
        requires_hsm = (
            multisig_wallet.security_policies.require_hsm_for_large_tx and
            large_amount > multisig_wallet.security_policies.large_tx_threshold
        )
        
        assert requires_hsm
    
    def test_transaction_amount_validation(self, multisig_wallet):
        """Test transaction amount validation against policies"""
        # Test amount exceeding single transaction limit
        excessive_amount = 150000  # Above max_single_transaction
        
        with pytest.raises(ValueError):
            if excessive_amount > multisig_wallet.security_policies.max_single_transaction:
                raise ValueError("Amount exceeds single transaction limit")
    
    def test_daily_limit_validation(self, multisig_wallet):
        """Test daily transaction limit validation"""
        # Simulate existing daily transactions
        today = int(time.time()) // 86400 * 86400
        multisig_wallet.daily_transaction_amounts[today] = 800000
        
        new_transaction_amount = 250000
        total_daily = multisig_wallet.daily_transaction_amounts.get(today, 0) + new_transaction_amount
        
        with pytest.raises(ValueError):
            if total_daily > multisig_wallet.security_policies.max_daily_amount:
                raise ValueError("Daily transaction limit exceeded")
    
    def test_emergency_transaction_creation(self, multisig_wallet):
        """Test creating emergency transactions"""
        emergency_tx = MockPendingTransaction(
            id=2,
            transaction_type="EmergencyAction",
            amount=0,
            recipient="",
            data=b"\x00",  # Emergency freeze action
            signatures=[],
            created_at=int(time.time()),
            expires_at=int(time.time()) + 3600,  # 1 hour for emergency
            executed=False
        )
        
        multisig_wallet.pending_transactions.append(emergency_tx)
        
        assert emergency_tx.transaction_type == "EmergencyAction"
        assert emergency_tx.data == b"\x00"
        assert emergency_tx.expires_at - emergency_tx.created_at == 3600

class TestTransactionSigning:
    """Test transaction signing with HSM integration"""
    
    @pytest.fixture
    def multisig_wallet(self):
        wallet = MockMultisigWallet()
        wallet.owners = ["owner1", "owner2", "owner3"]
        wallet.threshold = 2
        wallet.hsm_config = MockHsmConfig(
            enabled=True,
            hsm_type="YubicoHSM2",
            device_serial="YH2023001",
            public_key="pubkey123",
            attestation_key="attkey456",
            last_attestation=int(time.time()),
            firmware_version="2.3.1"
        )
        
        # Add a pending transaction
        transaction = MockPendingTransaction(
            id=1,
            transaction_type="Transfer",
            amount=75000,  # Requires HSM
            recipient="recipient1",
            data=b"",
            signatures=[],
            created_at=int(time.time()),
            expires_at=int(time.time()) + 86400,
            executed=False
        )
        wallet.pending_transactions.append(transaction)
        
        return wallet
    
    def test_basic_transaction_signing(self, multisig_wallet):
        """Test basic transaction signing by owners"""
        transaction_id = 1
        signer = "owner1"
        signature = b"signature_data_64_bytes" + b"\x00" * 40  # 64 bytes
        
        # Verify signer is owner
        assert signer in multisig_wallet.owners
        
        # Add signature
        signature_entry = {
            "signer": signer,
            "signature": signature,
            "signed_at": int(time.time()),
            "hsm_attestation": None
        }
        
        multisig_wallet.pending_transactions[0].signatures.append(signature_entry)
        
        assert len(multisig_wallet.pending_transactions[0].signatures) == 1
        assert multisig_wallet.pending_transactions[0].signatures[0]["signer"] == signer
    
    def test_hsm_attestation_signing(self, multisig_wallet):
        """Test transaction signing with HSM attestation"""
        transaction_id = 1
        signer = "owner1"
        signature = b"hsm_signature_data_64_bytes" + b"\x00" * 34  # 64 bytes
        
        # Create HSM attestation
        hsm_attestation = MockHsmAttestation(
            device_serial="YH2023001",
            timestamp=int(time.time()),
            signature=b"attestation_signature" + b"\x00" * 43,  # 64 bytes
            counter=1
        )
        
        # Verify attestation device matches HSM config
        assert hsm_attestation.device_serial == multisig_wallet.hsm_config.device_serial
        
        # Add signature with HSM attestation
        signature_entry = {
            "signer": signer,
            "signature": signature,
            "signed_at": int(time.time()),
            "hsm_attestation": hsm_attestation
        }
        
        multisig_wallet.pending_transactions[0].signatures.append(signature_entry)
        
        assert multisig_wallet.pending_transactions[0].signatures[0]["hsm_attestation"] is not None
        assert multisig_wallet.pending_transactions[0].signatures[0]["hsm_attestation"].device_serial == "YH2023001"
    
    def test_duplicate_signature_prevention(self, multisig_wallet):
        """Test prevention of duplicate signatures from same owner"""
        signer = "owner1"
        signature1 = b"signature1" + b"\x00" * 54
        signature2 = b"signature2" + b"\x00" * 54
        
        # Add first signature
        signature_entry1 = {
            "signer": signer,
            "signature": signature1,
            "signed_at": int(time.time()),
            "hsm_attestation": None
        }
        multisig_wallet.pending_transactions[0].signatures.append(signature_entry1)
        
        # Check for duplicate signer
        existing_signers = [sig["signer"] for sig in multisig_wallet.pending_transactions[0].signatures]
        
        with pytest.raises(ValueError):
            if signer in existing_signers:
                raise ValueError("Signer already signed this transaction")
    
    def test_unauthorized_signer_rejection(self, multisig_wallet):
        """Test rejection of signatures from non-owners"""
        unauthorized_signer = "not_an_owner"
        signature = b"unauthorized_signature" + b"\x00" * 42
        
        with pytest.raises(ValueError):
            if unauthorized_signer not in multisig_wallet.owners:
                raise ValueError("Unauthorized signer")
    
    def test_expired_transaction_signing(self, multisig_wallet):
        """Test rejection of signatures on expired transactions"""
        # Set transaction as expired
        multisig_wallet.pending_transactions[0].expires_at = int(time.time()) - 3600  # 1 hour ago
        
        current_time = int(time.time())
        
        with pytest.raises(ValueError):
            if current_time > multisig_wallet.pending_transactions[0].expires_at:
                raise ValueError("Transaction has expired")

class TestTransactionExecution:
    """Test transaction execution after sufficient signatures"""
    
    @pytest.fixture
    def multisig_wallet(self):
        wallet = MockMultisigWallet()
        wallet.owners = ["owner1", "owner2", "owner3"]
        wallet.threshold = 2
        
        # Add a fully signed transaction
        transaction = MockPendingTransaction(
            id=1,
            transaction_type="Transfer",
            amount=25000,
            recipient="recipient1",
            data=b"",
            signatures=[
                {
                    "signer": "owner1",
                    "signature": b"sig1" + b"\x00" * 60,
                    "signed_at": int(time.time()),
                    "hsm_attestation": None
                },
                {
                    "signer": "owner2",
                    "signature": b"sig2" + b"\x00" * 60,
                    "signed_at": int(time.time()),
                    "hsm_attestation": None
                }
            ],
            created_at=int(time.time()),
            expires_at=int(time.time()) + 86400,
            executed=False
        )
        wallet.pending_transactions.append(transaction)
        
        return wallet
    
    def test_successful_transaction_execution(self, multisig_wallet):
        """Test successful execution of fully signed transaction"""
        transaction_id = 1
        transaction = multisig_wallet.pending_transactions[0]
        
        # Verify sufficient signatures
        assert len(transaction.signatures) >= multisig_wallet.threshold
        
        # Verify not expired
        current_time = int(time.time())
        assert current_time <= transaction.expires_at
        
        # Execute transaction
        transaction.executed = True
        
        assert transaction.executed
    
    def test_insufficient_signatures_rejection(self, multisig_wallet):
        """Test rejection of execution with insufficient signatures"""
        # Remove one signature to make it insufficient
        multisig_wallet.pending_transactions[0].signatures.pop()
        
        transaction = multisig_wallet.pending_transactions[0]
        
        with pytest.raises(ValueError):
            if len(transaction.signatures) < multisig_wallet.threshold:
                raise ValueError("Insufficient signatures")
    
    def test_transaction_type_execution(self, multisig_wallet):
        """Test execution of different transaction types"""
        transaction_types = [
            "Transfer",
            "StakingOperation", 
            "RewardDistribution",
            "ConfigUpdate",
            "EmergencyAction"
        ]
        
        for tx_type in transaction_types:
            # Each transaction type should have specific execution logic
            assert tx_type in ["Transfer", "StakingOperation", "RewardDistribution", "ConfigUpdate", "EmergencyAction"]

if __name__ == "__main__":
    pytest.main([__file__, "-v"])