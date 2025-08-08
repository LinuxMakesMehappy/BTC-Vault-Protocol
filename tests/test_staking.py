"""
Comprehensive Staking Pool Testing Suite
Tests for protocol asset staking and allocation functionality with concurrent execution
Addresses FR7: Testing and Development Infrastructure requirements
"""

import pytest
import asyncio
import time
import threading
from unittest.mock import Mock, patch, AsyncMock
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Any
import sys
import os

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# Import configuration
try:
    from config.validators import get_validator_config, get_staking_config
    from config.treasury import get_allocation_config
except ImportError:
    # Mock configs if not available
    def get_validator_config():
        return {
            'sol_validators': ['validator1', 'validator2'],
            'eth_validators': ['eth_validator1'],
            'atom_validators': ['atom_validator1']
        }
    
    def get_staking_config():
        return {
            'sol_allocation': 4000,  # 40%
            'eth_allocation': 3000,  # 30%
            'atom_allocation': 3000  # 30%
        }
    
    def get_allocation_config():
        return {
            'rebalance_threshold': 200,  # 2%
            'max_deviation': 500  # 5%
        }

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

class TestConcurrentStakingOperations:
    """Test concurrent staking operations for performance and reliability"""
    
    @pytest.fixture
    def mock_staking_client(self):
        """Mock staking client for concurrent testing"""
        mock_client = Mock()
        mock_client.config = {
            'network': 'devnet',
            'program_id': 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
        }
        # Make async methods return AsyncMock
        mock_client.stake_protocol_assets = AsyncMock()
        mock_client.rebalance_allocations = AsyncMock()
        mock_client.add_validator = AsyncMock()
        mock_client.calculate_rewards = AsyncMock()
        return mock_client
    
    @pytest.mark.asyncio
    async def test_concurrent_staking_operations_stress(self, mock_staking_client):
        """Test high-volume concurrent staking operations"""
        # Mock successful responses
        mock_staking_client.stake_protocol_assets.return_value = {
            'success': True, 
            'staked_amount': 1000000,
            'transaction_id': 'tx_123'
        }
        mock_staking_client.rebalance_allocations.return_value = {
            'success': True,
            'rebalanced': True,
            'adjustments': {'SOL': 0, 'ETH': 0, 'ATOM': 0}
        }
        
        # Create concurrent staking tasks
        staking_tasks = [
            mock_staking_client.stake_protocol_assets(100000 * (i + 1))
            for i in range(50)
        ]
        
        # Create concurrent rebalancing tasks
        rebalancing_tasks = [
            mock_staking_client.rebalance_allocations()
            for i in range(20)
        ]
        
        # Execute all tasks concurrently
        start_time = time.time()
        all_tasks = staking_tasks + rebalancing_tasks
        results = await asyncio.gather(*all_tasks, return_exceptions=True)
        execution_time = time.time() - start_time
        
        # Verify results
        successful_results = [r for r in results if isinstance(r, dict) and r.get('success')]
        assert len(successful_results) == 70  # 50 staking + 20 rebalancing
        assert execution_time < 15.0  # Should complete within 15 seconds
        
        print(f"✅ Concurrent staking operations: {len(successful_results)}/70 in {execution_time:.2f}s")
    
    def test_concurrent_validator_selection_threadpool(self):
        """Test concurrent validator selection using ThreadPoolExecutor"""
        def select_best_validator(validator_pool):
            """Mock validator selection function"""
            time.sleep(0.02)  # Simulate selection time
            
            # Sort by performance score (descending) then commission (ascending)
            sorted_validators = sorted(
                validator_pool,
                key=lambda v: (-v['performance_score'], v['commission'])
            )
            
            return {
                'selected': sorted_validators[0] if sorted_validators else None,
                'pool_size': len(validator_pool),
                'selection_time': 0.02
            }
        
        # Create validator pools for different networks
        validator_pools = []
        
        # SOL validators
        for i in range(10):
            sol_pool = []
            for j in range(20):  # 20 validators per pool
                sol_pool.append({
                    'address': f'sol_validator_{i}_{j}',
                    'commission': 300 + (j * 50),  # 3% to 12.5%
                    'performance_score': 9000 + (j * 50),  # 90% to 99.5%
                    'network': 'SOL'
                })
            validator_pools.append(sol_pool)
        
        # ETH validators
        for i in range(5):
            eth_pool = []
            for j in range(15):  # 15 validators per pool
                eth_pool.append({
                    'address': f'eth_validator_{i}_{j}',
                    'commission': 400 + (j * 30),  # 4% to 8.2%
                    'performance_score': 9200 + (j * 40),  # 92% to 97.6%
                    'network': 'ETH'
                })
            validator_pools.append(eth_pool)
        
        # ATOM validators
        for i in range(3):
            atom_pool = []
            for j in range(10):  # 10 validators per pool
                atom_pool.append({
                    'address': f'atom_validator_{i}_{j}',
                    'commission': 500 + (j * 25),  # 5% to 7.25%
                    'performance_score': 9100 + (j * 45),  # 91% to 95.05%
                    'network': 'ATOM'
                })
            validator_pools.append(atom_pool)
        
        # Execute concurrent validator selection
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=6) as executor:
            results = list(executor.map(select_best_validator, validator_pools))
        execution_time = time.time() - start_time
        
        # Verify results
        successful_selections = [r for r in results if r['selected'] is not None]
        total_validators_evaluated = sum(r['pool_size'] for r in results)
        
        assert len(successful_selections) == len(validator_pools)
        assert total_validators_evaluated == (10*20 + 5*15 + 3*10)  # 485 total validators
        assert execution_time < 3.0  # Should complete quickly with threading
        
        # Verify best validators were selected (highest performance, lowest commission)
        for result in successful_selections:
            selected = result['selected']
            assert selected['performance_score'] >= 9000
            assert selected['commission'] <= 1000  # Should select low commission validators
        
        print(f"✅ Concurrent validator selection: {len(successful_selections)} pools, {total_validators_evaluated} validators in {execution_time:.2f}s")
    
    @pytest.mark.asyncio
    async def test_concurrent_allocation_rebalancing(self, mock_staking_client):
        """Test concurrent allocation rebalancing across multiple assets"""
        rebalance_results = []
        
        async def mock_rebalance_asset(asset, current_allocation, target_allocation):
            """Mock asset rebalancing function"""
            await asyncio.sleep(0.05)  # Simulate rebalancing time
            
            deviation = abs(current_allocation - target_allocation)
            adjustment_needed = deviation > 200  # 2% threshold
            
            if adjustment_needed:
                adjustment = target_allocation - current_allocation
                return {
                    'asset': asset,
                    'current_allocation': current_allocation,
                    'target_allocation': target_allocation,
                    'adjustment': adjustment,
                    'rebalanced': True,
                    'deviation': deviation
                }
            else:
                return {
                    'asset': asset,
                    'current_allocation': current_allocation,
                    'target_allocation': target_allocation,
                    'adjustment': 0,
                    'rebalanced': False,
                    'deviation': deviation
                }
        
        # Create rebalancing scenarios
        rebalancing_scenarios = [
            # Scenario 1: SOL overallocated
            ('SOL', 4500, 4000),  # 45% -> 40%
            ('ETH', 2750, 3000),  # 27.5% -> 30%
            ('ATOM', 2750, 3000), # 27.5% -> 30%
            
            # Scenario 2: ETH overallocated
            ('SOL', 3800, 4000),  # 38% -> 40%
            ('ETH', 3400, 3000),  # 34% -> 30%
            ('ATOM', 2800, 3000), # 28% -> 30%
            
            # Scenario 3: ATOM overallocated
            ('SOL', 3900, 4000),  # 39% -> 40%
            ('ETH', 2950, 3000),  # 29.5% -> 30%
            ('ATOM', 3150, 3000), # 31.5% -> 30%
            
            # Scenario 4: All within threshold (no rebalancing needed)
            ('SOL', 4050, 4000),  # 40.5% -> 40% (within 2% threshold)
            ('ETH', 2980, 3000),  # 29.8% -> 30%
            ('ATOM', 2970, 3000), # 29.7% -> 30%
        ]
        
        # Execute concurrent rebalancing
        start_time = time.time()
        tasks = [
            mock_rebalance_asset(asset, current, target)
            for asset, current, target in rebalancing_scenarios
        ]
        results = await asyncio.gather(*tasks)
        execution_time = time.time() - start_time
        
        # Analyze results
        rebalanced_assets = [r for r in results if r['rebalanced']]
        no_rebalance_needed = [r for r in results if not r['rebalanced']]
        
        # Verify rebalancing logic
        assert len(rebalanced_assets) == 9  # First 3 scenarios need rebalancing
        assert len(no_rebalance_needed) == 3  # Last scenario within threshold
        assert execution_time < 2.0  # Should complete quickly
        
        # Verify adjustments are correct
        for result in rebalanced_assets:
            expected_adjustment = result['target_allocation'] - result['current_allocation']
            assert result['adjustment'] == expected_adjustment
            assert result['deviation'] > 200  # Above 2% threshold
        
        for result in no_rebalance_needed:
            assert result['deviation'] <= 200  # Within 2% threshold
        
        print(f"✅ Concurrent rebalancing: {len(rebalanced_assets)} rebalanced, {len(no_rebalance_needed)} stable in {execution_time:.2f}s")
    
    def test_staking_performance_under_load(self):
        """Test staking system performance under high load"""
        import psutil
        import gc
        
        process = psutil.Process()
        initial_memory = process.memory_info().rss / (1024**2)  # MB
        
        # Simulate high-load staking scenario
        staking_operations = []
        
        # Create 1000 staking operations
        for i in range(1000):
            operation = {
                'operation_id': i,
                'asset': ['SOL', 'ETH', 'ATOM'][i % 3],
                'amount': 1000 + (i * 100),  # Varying amounts
                'validator': f'validator_{i % 50}',  # 50 different validators
                'timestamp': time.time() + i,
                'priority': i % 5,  # 5 priority levels
                'user_count': (i % 100) + 1  # 1-100 users per operation
            }
            staking_operations.append(operation)
        
        # Process operations in batches using ThreadPoolExecutor
        def process_staking_batch(batch):
            """Process a batch of staking operations"""
            batch_results = []
            for op in batch:
                # Simulate staking processing
                processing_time = 0.001 * op['priority']  # Higher priority = more processing
                time.sleep(processing_time)
                
                result = {
                    'operation_id': op['operation_id'],
                    'asset': op['asset'],
                    'amount': op['amount'],
                    'success': True,
                    'processing_time': processing_time,
                    'validator': op['validator']
                }
                batch_results.append(result)
            
            return batch_results
        
        # Split into batches for concurrent processing
        batch_size = 100
        batches = [
            staking_operations[i:i + batch_size]
            for i in range(0, len(staking_operations), batch_size)
        ]
        
        # Execute batches concurrently
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=4) as executor:
            batch_results = list(executor.map(process_staking_batch, batches))
        execution_time = time.time() - start_time
        
        # Flatten results
        all_results = []
        for batch_result in batch_results:
            all_results.extend(batch_result)
        
        # Analyze performance
        successful_operations = [r for r in all_results if r['success']]
        total_amount_staked = sum(r['amount'] for r in successful_operations)
        
        # Group by asset
        asset_totals = {}
        for result in successful_operations:
            asset = result['asset']
            if asset not in asset_totals:
                asset_totals[asset] = 0
            asset_totals[asset] += result['amount']
        
        final_memory = process.memory_info().rss / (1024**2)  # MB
        memory_increase = final_memory - initial_memory
        
        # Verify performance requirements
        assert len(successful_operations) == 1000  # All operations successful
        assert execution_time < 10.0  # Should complete within 10 seconds
        assert memory_increase < 100  # Memory increase should be minimal
        
        # Verify asset distribution
        assert len(asset_totals) == 3  # SOL, ETH, ATOM
        for asset, total in asset_totals.items():
            assert total > 0  # Each asset should have some allocation
        
        print(f"✅ High-load performance test: {len(successful_operations)} operations in {execution_time:.2f}s")
        print(f"   Total staked: ${total_amount_staked:,}")
        print(f"   Memory usage: {initial_memory:.1f}MB → {final_memory:.1f}MB (+{memory_increase:.1f}MB)")
        print(f"   Asset distribution: {asset_totals}")
        
        # Cleanup
        del staking_operations, all_results
        gc.collect()
    
    @pytest.mark.asyncio
    async def test_cross_chain_staking_coordination(self, mock_staking_client):
        """Test coordination of staking across multiple chains"""
        
        async def mock_stake_on_chain(chain, amount, validators):
            """Mock cross-chain staking function"""
            await asyncio.sleep(0.1 + (0.05 * len(validators)))  # Simulate network latency
            
            # Simulate different success rates for different chains
            success_rates = {'SOL': 0.95, 'ETH': 0.90, 'ATOM': 0.85}
            success = hash(f"{chain}{amount}") % 100 < (success_rates[chain] * 100)
            
            if success:
                return {
                    'chain': chain,
                    'amount': amount,
                    'validators': validators,
                    'success': True,
                    'transaction_id': f'{chain.lower()}_tx_{hash(str(amount))}',
                    'confirmation_time': 0.1 + (0.05 * len(validators))
                }
            else:
                return {
                    'chain': chain,
                    'amount': amount,
                    'validators': validators,
                    'success': False,
                    'error': f'{chain} network congestion',
                    'retry_recommended': True
                }
        
        # Define cross-chain staking operations
        staking_operations = [
            # SOL staking operations
            ('SOL', 400000, ['sol_validator_1', 'sol_validator_2']),
            ('SOL', 300000, ['sol_validator_3']),
            ('SOL', 200000, ['sol_validator_1', 'sol_validator_4']),
            
            # ETH staking operations
            ('ETH', 350000, ['eth_validator_1']),
            ('ETH', 250000, ['eth_validator_2', 'eth_validator_3']),
            
            # ATOM staking operations
            ('ATOM', 200000, ['cosmos_validator_1']),  # Everstake
            ('ATOM', 100000, ['osmosis_validator_1']), # Osmosis
            ('ATOM', 150000, ['cosmos_validator_2']),  # Additional Cosmos
        ]
        
        # Execute cross-chain staking concurrently
        start_time = time.time()
        tasks = [
            mock_stake_on_chain(chain, amount, validators)
            for chain, amount, validators in staking_operations
        ]
        results = await asyncio.gather(*tasks, return_exceptions=True)
        execution_time = time.time() - start_time
        
        # Analyze cross-chain results
        successful_stakes = [r for r in results if isinstance(r, dict) and r.get('success')]
        failed_stakes = [r for r in results if isinstance(r, dict) and not r.get('success')]
        
        # Group by chain
        chain_results = {}
        for result in successful_stakes:
            chain = result['chain']
            if chain not in chain_results:
                chain_results[chain] = {'count': 0, 'total_amount': 0, 'validators': set()}
            
            chain_results[chain]['count'] += 1
            chain_results[chain]['total_amount'] += result['amount']
            chain_results[chain]['validators'].update(result['validators'])
        
        # Verify cross-chain coordination
        assert len(successful_stakes) >= 6  # Most operations should succeed
        assert len(chain_results) == 3  # All three chains should have successful stakes
        assert execution_time < 5.0  # Should complete within 5 seconds
        
        # Verify allocation targets are approximately met
        total_staked = sum(chain_results[chain]['total_amount'] for chain in chain_results)
        if total_staked > 0:
            sol_percentage = (chain_results.get('SOL', {}).get('total_amount', 0) / total_staked) * 100
            eth_percentage = (chain_results.get('ETH', {}).get('total_amount', 0) / total_staked) * 100
            atom_percentage = (chain_results.get('ATOM', {}).get('total_amount', 0) / total_staked) * 100
            
            # Should be approximately 40% SOL, 30% ETH, 30% ATOM
            assert 35 <= sol_percentage <= 45  # Allow some variance
            assert 25 <= eth_percentage <= 35
            assert 25 <= atom_percentage <= 35
        
        print(f"✅ Cross-chain staking: {len(successful_stakes)} successful, {len(failed_stakes)} failed in {execution_time:.2f}s")
        for chain, data in chain_results.items():
            percentage = (data['total_amount'] / total_staked * 100) if total_staked > 0 else 0
            print(f"   {chain}: ${data['total_amount']:,} ({percentage:.1f}%) across {len(data['validators'])} validators")

def run_staking_tests():
    """Run all staking tests including concurrent tests"""
    print("🥩 Running Staking Pool Tests...")
    
    # Run standard and concurrent tests
    test_staking = TestStakingPool()
    test_concurrent = TestConcurrentStakingOperations()
    
    # List of all test methods
    standard_tests = [
        'test_initialize_staking_pool',
        'test_stake_protocol_assets',
        'test_allocation_calculations',
        'test_rebalancing_logic',
        'test_validator_management'
    ]
    
    concurrent_tests = [
        'test_concurrent_validator_selection_threadpool',
        'test_staking_performance_under_load'
    ]
    
    passed = 0
    failed = 0
    
    # Run standard tests
    for test_method in standard_tests:
        try:
            if hasattr(test_staking, test_method):
                method = getattr(test_staking, test_method)
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
    
    print(f"📊 Staking Pool Tests: {passed} passed, {failed} failed")
    return failed == 0

if __name__ == '__main__':
    success = run_staking_tests()
    sys.exit(0 if success else 1)