"""
Validator Configuration for Staking
Editable by non-programmers - modify values as needed
"""

# Solana Validators (40% allocation)
SOLANA_VALIDATORS = {
    "primary": {
        "vote_account": "7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2",
        "name": "Solana Foundation",
        "commission": "7%",
        "allocation_percentage": 2000,  # 20% of total (50% of SOL allocation)
        "min_stake_lamports": 1000000000  # 1 SOL minimum
    },
    "secondary": {
        "vote_account": "GE6atKoWiQ2pt3zL7N13pjNHjdLVys8LinG8qeJLcAiL",
        "name": "Everstake",
        "commission": "5%", 
        "allocation_percentage": 1000,  # 10% of total (25% of SOL allocation)
        "min_stake_lamports": 1000000000
    },
    "tertiary": {
        "vote_account": "StepeLdhJ2znRjHcZdjwMWsC4nTRURNKQY8Nca82LJp",
        "name": "Step Finance",
        "commission": "8%",
        "allocation_percentage": 1000,  # 10% of total (25% of SOL allocation)
        "min_stake_lamports": 1000000000
    }
}

# Ethereum Validators (30% allocation) - L2 Liquid Staking
ETHEREUM_VALIDATORS = {
    "arbitrum_lido": {
        "contract_address": "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84",
        "name": "Lido on Arbitrum",
        "apy_estimate": "4.5%",
        "allocation_percentage": 1500,  # 15% of total (50% of ETH allocation)
        "min_stake_eth": "0.01"
    },
    "optimism_rocketpool": {
        "contract_address": "0xae78736Cd615f374D3085123A210448E74Fc6393",
        "name": "Rocket Pool on Optimism", 
        "apy_estimate": "4.2%",
        "allocation_percentage": 1500,  # 15% of total (50% of ETH allocation)
        "min_stake_eth": "0.01"
    }
}

# Cosmos Validators (30% allocation)
COSMOS_VALIDATORS = {
    "everstake_cosmos": {
        "validator_address": "cosmosvaloper1tflk30mq5vgqjdly92kkhhq3raev2hnz6eete3",
        "name": "Everstake",
        "commission": "5%",
        "allocation_percentage": 2000,  # 20% of total (66.7% of ATOM allocation)
        "min_stake_uatom": 1000000  # 1 ATOM minimum
    },
    "cephalopod_cosmos": {
        "validator_address": "cosmosvaloper156gqf9837u7d4c4678yt3rl4ls9c5vuursrrzf",
        "name": "Cephalopod Equipment",
        "commission": "5%",
        "allocation_percentage": 0,  # Backup validator
        "min_stake_uatom": 1000000
    },
    "osmosis_validator": {
        "validator_address": "osmovaloper1clpqr4nrk4khgkxj78fcwwh6dl3uw4epsluffn",
        "name": "Osmosis Foundation",
        "commission": "5%",
        "allocation_percentage": 1000,  # 10% of total (33.3% of ATOM allocation)
        "min_stake_uosmo": 1000000  # 1 OSMO minimum
    }
}

# Staking Configuration
STAKING_CONFIG = {
    "rebalance_frequency_hours": 168,  # Rebalance weekly
    "min_reward_claim_threshold": 0.01,  # Minimum reward to claim
    "auto_compound": True,  # Automatically compound rewards
    "slashing_protection": True,  # Enable slashing protection
    "max_validator_commission": "10%"  # Maximum acceptable commission
}

# Risk Management
RISK_CONFIG = {
    "max_single_validator_percentage": 25,  # Max 25% to single validator
    "diversification_threshold": 3,  # Minimum 3 validators per chain
    "performance_monitoring_days": 30,  # Monitor performance over 30 days
    "auto_unstake_on_jail": True,  # Auto unstake if validator jailed
    "commission_increase_threshold": "2%"  # Alert if commission increases by 2%
}