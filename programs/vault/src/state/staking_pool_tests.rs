#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    fn create_test_staking_pool() -> StakingPool {
        let mut pool = StakingPool {
            total_staked: 0,
            total_treasury_value: 0,
            sol_allocation: AssetAllocation {
                target_percentage: 0,
                current_amount: 0,
                target_amount: 0,
                last_rebalance: 0,
                deviation_threshold: 0,
            },
            eth_allocation: AssetAllocation {
                target_percentage: 0,
                current_amount: 0,
                target_amount: 0,
                last_rebalance: 0,
                deviation_threshold: 0,
            },
            atom_allocation: AssetAllocation {
                target_percentage: 0,
                current_amount: 0,
                target_amount: 0,
                last_rebalance: 0,
                deviation_threshold: 0,
            },
            sol_staked: 0,
            eth_staked: 0,
            atom_staked: 0,
            sol_validators: Vec::new(),
            eth_validators: Vec::new(),
            atom_config: AtomStakingConfig {
                everstake_allocation: 0,
                osmosis_allocation: 0,
                everstake_validator: String::new(),
                osmosis_validator: String::new(),
            },
            rewards_accumulated: 0,
            rewards_distributed: 0,
            last_reward_calculation: 0,
            last_rebalance: 0,
            rebalance_threshold: 0,
            auto_rebalance_enabled: false,
            last_update: 0,
            bump: 0,
        };
        
        pool.initialize(1).unwrap();
        pool
    }

    #[test]
    fn test_staking_pool_initialization() {
        let pool = create_test_staking_pool();
        
        // Check allocation percentages
        assert_eq!(pool.sol_allocation.target_percentage, StakingPool::SOL_ALLOCATION_BPS);
        assert_eq!(pool.eth_allocation.target_percentage, StakingPool::ETH_ALLOCATION_BPS);
        assert_eq!(pool.atom_allocation.target_percentage, StakingPool::ATOM_ALLOCATION_BPS);
        
        // Check ATOM sub-allocations
        assert_eq!(pool.atom_config.everstake_allocation, StakingPool::ATOM_EVERSTAKE_BPS);
        assert_eq!(pool.atom_config.osmosis_allocation, StakingPool::ATOM_OSMOSIS_BPS);
        
        // Check total adds up to 100%
        let total = pool.sol_allocation.target_percentage 
            + pool.eth_allocation.target_percentage 
            + pool.atom_allocation.target_percentage;
        assert_eq!(total, StakingPool::TOTAL_BPS);
        
        // Check auto-rebalance is enabled
        assert!(pool.auto_rebalance_enabled);
    }

    #[test]
    fn test_calculate_target_allocations() {
        let mut pool = create_test_staking_pool();
        let treasury_value = 1_000_000u64; // $1M USD
        
        pool.calculate_target_allocations(treasury_value).unwrap();
        
        // Check target amounts
        assert_eq!(pool.sol_allocation.target_amount, 400_000); // 40% of $1M
        assert_eq!(pool.eth_allocation.target_amount, 300_000); // 30% of $1M
        assert_eq!(pool.atom_allocation.target_amount, 300_000); // 30% of $1M
        
        // Check total treasury value is set
        assert_eq!(pool.total_treasury_value, treasury_value);
    }

    #[test]
    fn test_allocation_deviation_calculation() {
        let pool = create_test_staking_pool();
        
        // Test no deviation
        let deviation = pool.calculate_allocation_deviation(100, 100).unwrap();
        assert_eq!(deviation, 0);
        
        // Test 10% over-allocation
        let deviation = pool.calculate_allocation_deviation(110, 100).unwrap();
        assert_eq!(deviation, 1000); // 10% in basis points
        
        // Test 20% under-allocation
        let deviation = pool.calculate_allocation_deviation(80, 100).unwrap();
        assert_eq!(deviation, 2000); // 20% in basis points
        
        // Test zero target (should return 0)
        let deviation = pool.calculate_allocation_deviation(50, 0).unwrap();
        assert_eq!(deviation, 0);
    }

    #[test]
    fn test_needs_rebalancing() {
        let mut pool = create_test_staking_pool();
        pool.calculate_target_allocations(1_000_000).unwrap();
        
        // Set current amounts equal to targets - no rebalancing needed
        pool.sol_allocation.current_amount = 400_000;
        pool.eth_allocation.current_amount = 300_000;
        pool.atom_allocation.current_amount = 300_000;
        
        assert!(!pool.needs_rebalancing().unwrap());
        
        // Create significant deviation in SOL (over 2% threshold)
        pool.sol_allocation.current_amount = 450_000; // 12.5% over target
        
        assert!(pool.needs_rebalancing().unwrap());
        
        // Reset SOL and create deviation in ETH
        pool.sol_allocation.current_amount = 400_000;
        pool.eth_allocation.current_amount = 270_000; // 10% under target
        
        assert!(pool.needs_rebalancing().unwrap());
    }

    #[test]
    fn test_rebalancing_requirements() {
        let mut pool = create_test_staking_pool();
        pool.calculate_target_allocations(1_000_000).unwrap();
        
        // Set current amounts with deviations
        pool.sol_allocation.current_amount = 450_000; // 50k over
        pool.eth_allocation.current_amount = 270_000; // 30k under
        pool.atom_allocation.current_amount = 280_000; // 20k under
        
        let (sol_diff, eth_diff, atom_diff) = pool.get_rebalancing_requirements().unwrap();
        
        assert_eq!(sol_diff, -50_000); // Need to reduce SOL by 50k
        assert_eq!(eth_diff, 30_000);  // Need to increase ETH by 30k
        assert_eq!(atom_diff, 20_000); // Need to increase ATOM by 20k
    }

    #[test]
    fn test_validator_management() {
        let mut pool = create_test_staking_pool();
        
        // Test adding SOL validator
        let sol_validator = ValidatorInfo {
            address: "sol_validator_1".to_string(),
            commission: 500, // 5%
            stake_amount: 0,
            performance_score: 9500, // 95%
            is_active: true,
        };
        
        pool.add_sol_validator(sol_validator).unwrap();
        assert_eq!(pool.sol_validators.len(), 1);
        
        // Test adding ETH validator
        let eth_validator = ValidatorInfo {
            address: "eth_validator_1".to_string(),
            commission: 300, // 3%
            stake_amount: 0,
            performance_score: 9800, // 98%
            is_active: true,
        };
        
        pool.add_eth_validator(eth_validator).unwrap();
        assert_eq!(pool.eth_validators.len(), 1);
        
        // Test validator with too high commission (should fail)
        let bad_validator = ValidatorInfo {
            address: "bad_validator".to_string(),
            commission: 2500, // 25% - too high for SOL
            stake_amount: 0,
            performance_score: 5000,
            is_active: true,
        };
        
        assert!(pool.add_sol_validator(bad_validator).is_err());
    }

    #[test]
    fn test_validator_selection() {
        let mut pool = create_test_staking_pool();
        
        // Add multiple SOL validators with different performance/commission
        let validators = vec![
            ValidatorInfo {
                address: "validator_1".to_string(),
                commission: 500,  // 5%
                stake_amount: 0,
                performance_score: 9500, // 95%
                is_active: true,
            },
            ValidatorInfo {
                address: "validator_2".to_string(),
                commission: 300,  // 3%
                stake_amount: 0,
                performance_score: 9500, // 95% (same performance, lower commission)
                is_active: true,
            },
            ValidatorInfo {
                address: "validator_3".to_string(),
                commission: 400,  // 4%
                stake_amount: 0,
                performance_score: 9800, // 98% (higher performance)
                is_active: true,
            },
            ValidatorInfo {
                address: "validator_4".to_string(),
                commission: 600,  // 6%
                stake_amount: 0,
                performance_score: 9000, // 90%
                is_active: false, // Inactive
            },
        ];
        
        for validator in validators {
            pool.add_sol_validator(validator).unwrap();
        }
        
        // Select best 2 validators
        let best_validators = pool.select_best_sol_validators(2);
        
        assert_eq!(best_validators.len(), 2);
        // Should select validator_3 first (highest performance)
        assert_eq!(best_validators[0].address, "validator_3");
        // Should select validator_2 second (same performance as validator_1 but lower commission)
        assert_eq!(best_validators[1].address, "validator_2");
    }

    #[test]
    fn test_atom_config_validation() {
        let mut pool = create_test_staking_pool();
        
        // Test valid ATOM config
        let valid_config = AtomStakingConfig {
            everstake_allocation: 2000, // 20%
            osmosis_allocation: 1000,   // 10%
            everstake_validator: "everstake_validator".to_string(),
            osmosis_validator: "osmosis_validator".to_string(),
        };
        
        pool.update_atom_config(valid_config).unwrap();
        
        // Test invalid ATOM config (doesn't add up to 30%)
        let invalid_config = AtomStakingConfig {
            everstake_allocation: 1500, // 15%
            osmosis_allocation: 1000,   // 10% (total 25%, not 30%)
            everstake_validator: "everstake_validator".to_string(),
            osmosis_validator: "osmosis_validator".to_string(),
        };
        
        assert!(pool.update_atom_config(invalid_config).is_err());
    }

    #[test]
    fn test_allocation_validation() {
        let pool = create_test_staking_pool();
        
        // Should pass validation (allocations add up to 100%)
        assert!(pool.validate_allocations().is_ok());
        
        // Test with invalid allocations by creating a new pool
        let mut invalid_pool = create_test_staking_pool();
        invalid_pool.sol_allocation.target_percentage = 5000; // 50%
        // ETH and ATOM still 30% each = total 110%
        
        assert!(invalid_pool.validate_allocations().is_err());
    }

    #[test]
    fn test_update_current_allocations() {
        let mut pool = create_test_staking_pool();
        
        pool.update_current_allocations(400_000, 300_000, 300_000).unwrap();
        
        assert_eq!(pool.sol_allocation.current_amount, 400_000);
        assert_eq!(pool.eth_allocation.current_amount, 300_000);
        assert_eq!(pool.atom_allocation.current_amount, 300_000);
        assert_eq!(pool.total_staked, 1_000_000);
    }

    #[test]
    fn test_mark_rebalanced() {
        let mut pool = create_test_staking_pool();
        
        pool.mark_rebalanced().unwrap();
        
        // All rebalance timestamps should be updated
        assert!(pool.last_rebalance > 0);
        assert!(pool.sol_allocation.last_rebalance > 0);
        assert!(pool.eth_allocation.last_rebalance > 0);
        assert!(pool.atom_allocation.last_rebalance > 0);
    }

    #[test]
    fn test_allocation_percentages() {
        // Test that the allocation percentages match requirements
        assert_eq!(StakingPool::SOL_ALLOCATION_BPS, 4000);  // 40%
        assert_eq!(StakingPool::ETH_ALLOCATION_BPS, 3000);  // 30%
        assert_eq!(StakingPool::ATOM_ALLOCATION_BPS, 3000); // 30%
        
        // Test ATOM sub-allocations
        assert_eq!(StakingPool::ATOM_EVERSTAKE_BPS, 2000); // 20% of total
        assert_eq!(StakingPool::ATOM_OSMOSIS_BPS, 1000);   // 10% of total
        
        // Test that ATOM sub-allocations add up to total ATOM allocation
        assert_eq!(
            StakingPool::ATOM_EVERSTAKE_BPS + StakingPool::ATOM_OSMOSIS_BPS,
            StakingPool::ATOM_ALLOCATION_BPS
        );
    }

    #[test]
    fn test_edge_cases() {
        let mut pool = create_test_staking_pool();
        
        // Test with zero treasury value
        pool.calculate_target_allocations(0).unwrap();
        assert_eq!(pool.sol_allocation.target_amount, 0);
        assert_eq!(pool.eth_allocation.target_amount, 0);
        assert_eq!(pool.atom_allocation.target_amount, 0);
        
        // Test rebalancing with zero treasury (should not need rebalancing)
        assert!(!pool.needs_rebalancing().unwrap());
        
        // Test adding maximum validators
        for i in 0..10 {
            let validator = ValidatorInfo {
                address: format!("validator_{}", i),
                commission: 500,
                stake_amount: 0,
                performance_score: 9000,
                is_active: true,
            };
            pool.add_sol_validator(validator).unwrap();
        }
        
        // Adding 11th validator should fail
        let extra_validator = ValidatorInfo {
            address: "validator_11".to_string(),
            commission: 500,
            stake_amount: 0,
            performance_score: 9000,
            is_active: true,
        };
        
        assert!(pool.add_sol_validator(extra_validator).is_err());
    }
}
