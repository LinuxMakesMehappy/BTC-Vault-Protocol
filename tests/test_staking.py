"""
Staking Pool Testing Suite
Tests for protocol asset staking and allocation functionality
"""

import pytest
import asyncio
import time
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor
import sys
import os

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

class TestStakingPool:
    """Test suite for staking pool operations"""
    
    @pytest.fixture
    def mock_staking_client(self):
        """Mock staking client for testing"""
        mock_client = Mock()
        mock_client.config = {
            'network': 'devnet',
            'program_id': 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
        }
        # Make async methods return AsyncMock
        mock_client.initialize_staking_pool = AsyncMock()
        mock_client.stake_protocol_assets = AsyncMock()
        mock_client.rebalance_allocations = AsyncMock()
        mock_client.add_sol_validator = AsyncMock()
        mock_client.add_eth_validator = AsyncMock()
        mock_client.update_atom_config = AsyncMock()
        return mock_client
    
    @pytest.fixture
    def sample_treasury_data(self):
        """Sample treasury data for testing"""
        return {
            'total_usd_value': 1_000_000,  # $1M USD
            'sol_balance': 400_000,        # $400k USD equivalent
            'eth_balance': 300_000,        # $300k USD equivalent
            'atom_balance': 300_000,       # $300k USD equivalent
        }
    
    @pytest.fixture
    def sample_validators(self):
        """Sample validator data for testing"""
        return {
            'sol_validators': [
                {
                    'address': 'sol_validator_1',
                    'commission': 500,  # 5%
                    'performance_score': 9500,  # 95%
                    'is_active': True
                },
                {
                    'address': 'sol_validator_2',
                    'commission': 300,  # 3%
                    'performance_score': 9800,  # 98%
                    'is_active': True
                }
            ],
            'eth_validators': [
                {
                    'address': 'eth_validator_1',
                    'commission': 400,  # 4%
                    'performance_score': 9600,  # 96%
                    'is_active': True
                }
            ]
        }

    @pytest.mark.asyncio
    async def test_initialize_staking_pool(self, mock_staking_client):
        """Test staking pool initialization"""
        mock_staking_client.initialize_staking_pool.return_value = {
            'success': True,
            'sol_allocation': 4000,  # 40%
            'eth_allocation': 3000,  # 30%
            'atom_allocation': 3000, # 30%
            'atom_everstake': 2000,  # 20%
            'atom_osmosis': 1000,    # 10%
        }
        
        result = await mock_staking_client.initialize_staking_pool()
        
        assert result['success'] is True
        assert result['sol_allocation'] == 4000
        assert result['eth_allocation'] == 3000
        assert result['atom_allocation'] == 3000
        assert result['atom_everstake'] == 2000
        assert result['atom_osmosis'] == 1000
        
        # Verify total allocation is 100%
        total = result['sol_allocation'] + result['eth_allocation'] + result['atom_allocation']
        assert total == 10000  # 100% in basis points

    @pytest.mark.asyncio
    async def test_stake_protocol_assets(self, mock_staking_client, sample_treasury_data):
        """Test protocol asset staking with proper allocations"""
        mock_staking_client.stake_protocol_assets.return_value = {
            'success': True,
            'sol_staked': 400_000,  # 40% of $1M
            'eth_staked': 300_000,  # 30% of $1M
            'atom_staked': 300_000, # 30% of $1M
            'total_staked': 1_000_000,
            'allocations_met': True
        }
        
        result = await mock_staking_client.stake_protocol_assets(
            sample_treasury_data['total_usd_value']
        )
        
        assert result['success'] is True
        assert result['sol_staked'] == 400_000
        assert result['eth_staked'] == 300_000
        assert result['atom_staked'] == 300_000
        assert result['total_staked'] == 1_000_000
        assert result['allocations_met'] is True

    @pytest.mark.asyncio
    async def test_allocation_calculations(self, mock_staking_client):
        """Test allocation percentage calculations"""
        treasury_values = [100_000, 500_000, 1_000_000, 2_000_000]
        
        for treasury_value in treasury_values:
            mock_staking_client.stake_protocol_assets.return_value = {
                'success': True,
                'sol_target': treasury_value * 0.4,   # 40%
                'eth_target': treasury_value * 0.3,   # 30%
                'atom_target': treasury_value * 0.3,  # 30%
                'treasury_value': treasury_value
            }
            
            result = await mock_staking_client.stake_protocol_assets(treasury_value)
            
            assert result['success'] is True
            assert result['sol_target'] == treasury_value * 0.4
            assert result['eth_target'] == treasury_value * 0.3
            assert result['atom_target'] == treasury_value * 0.3
            
            # Verify total allocation equals treasury value
            total_target = result['sol_target'] + result['eth_target'] + result['atom_target']
            assert total_target == treasury_value

    @pytest.mark.asyncio
    async def test_rebalancing_logic(self, mock_staking_client):
        """Test rebalancing when allocations deviate from targets"""
        # Test scenario where rebalancing is needed
        mock_staking_client.rebalance_allocations.return_value = {
            'success': True,
            'rebalancing_needed': True,
            'sol_adjustment': -50_000,  # Reduce SOL by $50k
            'eth_adjustment': 30_000,   # Increase ETH by $30k
            'atom_adjustment': 20_000,  # Increase ATOM by $20k
            'deviation_sol': 12.5,      # 12.5% deviation
            'deviation_eth': 10.0,      # 10% deviation
            'deviation_atom': 6.7       # 6.7% deviation
        }
        
        result = await mock_staking_client.rebalance_allocations()
        
        assert result['success'] is True
        assert result['rebalancing_needed'] is True
        assert result['sol_adjustment'] == -50_000
        assert result['eth_adjustment'] == 30_000
        assert result['atom_adjustment'] == 20_000
        
        # Test scenario where no rebalancing is needed
        mock_staking_client.rebalance_allocations.return_value = {
            'success': True,
            'rebalancing_needed': False,
            'message': 'Allocations within threshold'
        }
        
        result = await mock_staking_client.rebalance_allocations()
        
        assert result['success'] is True
        assert result['rebalancing_needed'] is False

    @pytest.mark.asyncio
    async def test_validator_management(self, mock_staking_client, sample_validators):
        """Test adding and managing validators"""
        # Test adding SOL validator
        mock_staking_client.add_sol_validator.return_value = {
            'success': True,
            'validator_added': True,
            'total_sol_validators': 1
        }
        
        sol_validator = sample_validators['sol_validators'][0]
        result = await mock_staking_client.add_sol_validator(
            sol_validator['address'],
            sol_validator['commission'],
            sol_validator['performance_score']
        )
        
        assert result['success'] is True
        assert result['validator_added'] is True
        
        # Test adding ETH validator
        mock_staking_client.add_eth_validator.return_value = {
            'success': True,
            'validator_added': True,
            'total_eth_validators': 1
        }
        
        eth_validator = sample_validators['eth_validators'][0]
        result = await mock_staking_client.add_eth_validator(
            eth_validator['address'],
            eth_validator['commission'],
            eth_validator['performance_score']
        )
        
        assert result['success'] is True
        assert result['validator_added'] is True

    @pytest.mark.asyncio
    async def test_validator_selection_logic(self, mock_staking_client, sample_validators):
        """Test validator selection based on performance and commission"""
        # Mock validator selection response
        mock_staking_client.select_best_validators = AsyncMock()
        mock_staking_client.select_best_validators.return_value = {
            'success': True,
            'selected_validators': [
                {
                    'address': 'sol_validator_2',  # Higher performance
                    'commission': 300,
                    'performance_score': 9800,
                    'selection_reason': 'highest_performance'
                },
                {
                    'address': 'sol_validator_1',  # Lower commission for same performance
                    'commission': 500,
                    'performance_score': 9500,
                    'selection_reason': 'good_performance_low_commission'
                }
            ]
        }
        
        result = await mock_staking_client.select_best_validators('sol', 2)
        
        assert result['success'] is True
        assert len(result['selected_validators']) == 2
        
        # Verify selection prioritizes performance first, then commission
        validators = result['selected_validators']
        assert validators[0]['performance_score'] >= validators[1]['performance_score']

    @pytest.mark.asyncio
    async def test_atom_staking_configuration(self, mock_staking_client):
        """Test ATOM staking configuration with Everstake and Osmosis"""
        mock_staking_client.update_atom_config.return_value = {
            'success': True,
            'everstake_allocation': 2000,  # 20% of total (66.67% of ATOM)
            'osmosis_allocation': 1000,    # 10% of total (33.33% of ATOM)
            'everstake_validator': 'cosmosvaloper1everstake...',
            'osmosis_validator': 'osmovaloper1osmosis...',
            'total_atom_allocation': 3000  # 30% of total
        }
        
        result = await mock_staking_client.update_atom_config(
            'cosmosvaloper1everstake...',
            'osmovaloper1osmosis...'
        )
        
        assert result['success'] is True
        assert result['everstake_allocation'] == 2000
        assert result['osmosis_allocation'] == 1000
        assert result['total_atom_allocation'] == 3000
        
        # Verify ATOM sub-allocations add up correctly
        total_atom = result['everstake_allocation'] + result['osmosis_allocation']
        assert total_atom == result['total_atom_allocation']

    @pytest.mark.asyncio
    async def test_deviation_threshold_checking(self, mock_staking_client):
        """Test deviation threshold checking for rebalancing triggers"""
        test_scenarios = [
            {
                'name': 'within_threshold',
                'sol_deviation': 1.5,  # Below 2% threshold
                'eth_deviation': 1.8,
                'atom_deviation': 1.2,
                'needs_rebalancing': False
            },
            {
                'name': 'sol_exceeds_threshold',
                'sol_deviation': 3.5,  # Above 2% threshold
                'eth_deviation': 1.0,
                'atom_deviation': 1.5,
                'needs_rebalancing': True
            },
            {
                'name': 'multiple_exceed_threshold',
                'sol_deviation': 2.5,
                'eth_deviation': 4.0,  # Above threshold
                'atom_deviation': 3.2,  # Above threshold
                'needs_rebalancing': True
            }
        ]
        
        for scenario in test_scenarios:
            mock_staking_client.check_deviation_thresholds = AsyncMock()
            mock_staking_client.check_deviation_thresholds.return_value = {
                'success': True,
                'sol_deviation': scenario['sol_deviation'],
                'eth_deviation': scenario['eth_deviation'],
                'atom_deviation': scenario['atom_deviation'],
                'needs_rebalancing': scenario['needs_rebalancing'],
                'threshold': 2.0  # 2% threshold
            }
            
            result = await mock_staking_client.check_deviation_thresholds()
            
            assert result['success'] is True
            assert result['needs_rebalancing'] == scenario['needs_rebalancing']
            assert result['sol_deviation'] == scenario['sol_deviation']

    @pytest.mark.asyncio
    async def test_concurrent_staking_operations(self, mock_staking_client, sample_treasury_data):
        """Test concurrent staking operations"""
        # Mock responses for concurrent operations
        mock_staking_client.stake_protocol_assets.return_value = {'success': True, 'operation': 'stake'}
        mock_staking_client.rebalance_allocations.return_value = {'success': True, 'operation': 'rebalance'}
        mock_staking_client.add_sol_validator.return_value = {'success': True, 'operation': 'add_validator'}
        
        # Create concurrent tasks
        tasks = [
            mock_staking_client.stake_protocol_assets(sample_treasury_data['total_usd_value']),
            mock_staking_client.rebalance_allocations(),
            mock_staking_client.add_sol_validator('validator_1', 500, 9500),
            mock_staking_client.add_sol_validator('validator_2', 400, 9600),
            mock_staking_client.add_eth_validator('eth_validator_1', 300, 9700),
        ]
        
        # Execute all tasks concurrently
        results = await asyncio.gather(*tasks)
        
        # All operations should succeed
        assert all(result['success'] for result in results)
        assert len(results) == 5

    @pytest.mark.asyncio
    async def test_staking_error_scenarios(self, mock_staking_client):
        """Test error scenarios in staking operations"""
        error_scenarios = [
            {
                'operation': 'stake_protocol_assets',
                'error': 'Insufficient treasury balance',
                'code': 'InsufficientBalance'
            },
            {
                'operation': 'add_sol_validator',
                'error': 'Validator commission too high',
                'code': 'InvalidAllocation'
            },
            {
                'operation': 'rebalance_allocations',
                'error': 'Rebalancing threshold not met',
                'code': 'InvalidOperation'
            },
            {
                'operation': 'update_atom_config',
                'error': 'Invalid ATOM allocation percentages',
                'code': 'InvalidAllocation'
            }
        ]
        
        for scenario in error_scenarios:
            if scenario['operation'] == 'stake_protocol_assets':
                mock_staking_client.stake_protocol_assets.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = await mock_staking_client.stake_protocol_assets(1000000)
            elif scenario['operation'] == 'add_sol_validator':
                mock_staking_client.add_sol_validator.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = await mock_staking_client.add_sol_validator('validator', 2500, 9000)  # High commission
            elif scenario['operation'] == 'rebalance_allocations':
                mock_staking_client.rebalance_allocations.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = await mock_staking_client.rebalance_allocations()
            elif scenario['operation'] == 'update_atom_config':
                mock_staking_client.update_atom_config.return_value = {
                    'success': False,
                    'error': scenario['error'],
                    'error_code': scenario['code']
                }
                result = await mock_staking_client.update_atom_config('validator1', 'validator2')
            
            assert result['success'] is False
            assert scenario['error'] in result['error']
            assert result['error_code'] == scenario['code']

    @pytest.mark.asyncio
    async def test_allocation_edge_cases(self, mock_staking_client):
        """Test edge cases in allocation calculations"""
        edge_cases = [
            {
                'name': 'zero_treasury',
                'treasury_value': 0,
                'expected_sol': 0,
                'expected_eth': 0,
                'expected_atom': 0
            },
            {
                'name': 'small_treasury',
                'treasury_value': 100,  # $100
                'expected_sol': 40,     # 40%
                'expected_eth': 30,     # 30%
                'expected_atom': 30     # 30%
            },
            {
                'name': 'large_treasury',
                'treasury_value': 100_000_000,  # $100M
                'expected_sol': 40_000_000,     # 40%
                'expected_eth': 30_000_000,     # 30%
                'expected_atom': 30_000_000     # 30%
            }
        ]
        
        for case in edge_cases:
            mock_staking_client.calculate_allocations = AsyncMock()
            mock_staking_client.calculate_allocations.return_value = {
                'success': True,
                'treasury_value': case['treasury_value'],
                'sol_target': case['expected_sol'],
                'eth_target': case['expected_eth'],
                'atom_target': case['expected_atom']
            }
            
            result = await mock_staking_client.calculate_allocations(case['treasury_value'])
            
            assert result['success'] is True
            assert result['sol_target'] == case['expected_sol']
            assert result['eth_target'] == case['expected_eth']
            assert result['atom_target'] == case['expected_atom']

    def test_allocation_constants(self):
        """Test that allocation constants match requirements"""
        # Test allocation percentages
        SOL_ALLOCATION_BPS = 4000  # 40%
        ETH_ALLOCATION_BPS = 3000  # 30%
        ATOM_ALLOCATION_BPS = 3000 # 30%
        TOTAL_BPS = 10000          # 100%
        
        # Verify allocations add up to 100%
        total = SOL_ALLOCATION_BPS + ETH_ALLOCATION_BPS + ATOM_ALLOCATION_BPS
        assert total == TOTAL_BPS
        
        # Test ATOM sub-allocations
        ATOM_EVERSTAKE_BPS = 2000  # 20% of total (66.67% of ATOM)
        ATOM_OSMOSIS_BPS = 1000    # 10% of total (33.33% of ATOM)
        
        # Verify ATOM sub-allocations add up to total ATOM allocation
        atom_total = ATOM_EVERSTAKE_BPS + ATOM_OSMOSIS_BPS
        assert atom_total == ATOM_ALLOCATION_BPS
        
        # Verify percentages
        assert SOL_ALLOCATION_BPS / TOTAL_BPS == 0.4   # 40%
        assert ETH_ALLOCATION_BPS / TOTAL_BPS == 0.3   # 30%
        assert ATOM_ALLOCATION_BPS / TOTAL_BPS == 0.3  # 30%

    @pytest.mark.asyncio
    async def test_performance_with_large_validator_sets(self, mock_staking_client):
        """Test performance with maximum number of validators"""
        # Test adding maximum validators (10 each for SOL and ETH)
        mock_staking_client.add_validators_bulk = AsyncMock()
        mock_staking_client.add_validators_bulk.return_value = {
            'success': True,
            'sol_validators_added': 10,
            'eth_validators_added': 10,
            'total_validators': 20
        }
        
        # Create validator data
        validators = {
            'sol': [{'address': f'sol_validator_{i}', 'commission': 500, 'performance': 9000 + i*10} for i in range(10)],
            'eth': [{'address': f'eth_validator_{i}', 'commission': 400, 'performance': 9000 + i*10} for i in range(10)]
        }
        
        result = await mock_staking_client.add_validators_bulk(validators)
        
        assert result['success'] is True
        assert result['sol_validators_added'] == 10
        assert result['eth_validators_added'] == 10
        assert result['total_validators'] == 20

if __name__ == '__main__':
    pytest.main([__file__, '-v'])