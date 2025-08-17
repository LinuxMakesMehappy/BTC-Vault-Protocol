use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Token, TokenAccount, Transfer},
    stake::{StakePool, StakeAccount},
};
use crate::state::liquidity_engine::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeLiquidityEngine<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + LiquidityEngineState::INIT_SPACE,
        seeds = [b"liquidity_engine"],
        bump
    )]
    pub liquidity_engine: Account<'info, LiquidityEngineState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapViaJupiter<'info> {
    #[account(
        mut,
        seeds = [b"liquidity_engine"],
        bump = liquidity_engine.bump
    )]
    pub liquidity_engine: Account<'info, LiquidityEngineState>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_source_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_destination_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub destination_vault: Account<'info, TokenAccount>,
    
    /// CHECK: Jupiter program will validate this
    pub jupiter_program: UncheckedAccount<'info>,
    
    /// CHECK: Jupiter accounts will be validated by Jupiter program
    pub jupiter_accounts: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct StakeToJSOL<'info> {
    #[account(
        mut,
        seeds = [b"liquidity_engine"],
        bump = liquidity_engine.bump
    )]
    pub liquidity_engine: Account<'info, LiquidityEngineState>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_sol_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_jsol_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,
    
    #[account(mut)]
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub validator_list: AccountInfo<'info>,
    
    #[account(mut)]
    pub reserve_stake: Account<'info, StakeAccount>,
    
    #[account(mut)]
    pub pool_mint: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub pool_token_to: Account<'info, TokenAccount>,
    
    /// CHECK: Sanctum program will validate this
    pub sanctum_program: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub stake_program: Program<'info, anchor_lang::system_program::System>,
}

#[derive(Accounts)]
pub struct InstantUnstakeJSOL<'info> {
    #[account(
        mut,
        seeds = [b"liquidity_engine"],
        bump = liquidity_engine.bump
    )]
    pub liquidity_engine: Account<'info, LiquidityEngineState>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_jsol_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_sol_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub reserve_pool: Account<'info, TokenAccount>,
    
    /// CHECK: Sanctum reserve pool program
    pub sanctum_reserve_program: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CrossChainBridge<'info> {
    #[account(
        mut,
        seeds = [b"liquidity_engine"],
        bump = liquidity_engine.bump
    )]
    pub liquidity_engine: Account<'info, LiquidityEngineState>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub bridge_vault: Account<'info, TokenAccount>,
    
    /// CHECK: Wormhole bridge program
    pub bridge_program: UncheckedAccount<'info>,
    
    /// CHECK: Bridge accounts will be validated by bridge program
    pub bridge_accounts: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeLiquidityEngine<'info> {
    pub fn process(ctx: Context<InitializeLiquidityEngine>) -> Result<()> {
        let liquidity_engine = &mut ctx.accounts.liquidity_engine;
        
        liquidity_engine.authority = ctx.accounts.authority.key();
        liquidity_engine.bump = ctx.bumps.liquidity_engine;
        liquidity_engine.total_volume = 0;
        liquidity_engine.total_fees = 0;
        liquidity_engine.is_paused = false;
        liquidity_engine.jupiter_program = Pubkey::default();
        liquidity_engine.sanctum_program = Pubkey::default();
        liquidity_engine.bridge_program = Pubkey::default();
        liquidity_engine.fee_rate = 30; // 0.3% fee
        liquidity_engine.max_slippage = 100; // 1% max slippage
        
        msg!("Liquidity Engine initialized");
        Ok(())
    }
}

impl<'info> SwapViaJupiter<'info> {
    pub fn process(
        ctx: Context<SwapViaJupiter>,
        amount_in: u64,
        minimum_amount_out: u64,
        jupiter_data: Vec<u8>,
    ) -> Result<()> {
        let liquidity_engine = &mut ctx.accounts.liquidity_engine;
        
        require!(!liquidity_engine.is_paused, VaultError::LiquidityEnginePaused);
        
        // Calculate fees
        let fee_amount = amount_in
            .checked_mul(liquidity_engine.fee_rate as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        
        let swap_amount = amount_in.checked_sub(fee_amount).unwrap();
        
        // Transfer tokens from user to source vault
        let transfer_to_vault = Transfer {
            from: ctx.accounts.user_source_token.to_account_info(),
            to: ctx.accounts.source_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_to_vault,
            ),
            amount_in,
        )?;
        
        // Execute Jupiter swap via CPI
        let jupiter_accounts = vec![
            ctx.accounts.source_vault.to_account_info(),
            ctx.accounts.destination_vault.to_account_info(),
            ctx.accounts.jupiter_accounts.to_account_info(),
        ];
        
        let jupiter_instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.jupiter_program.key(),
            accounts: jupiter_accounts.iter().map(|acc| AccountMeta {
                pubkey: acc.key(),
                is_signer: false,
                is_writable: true,
            }).collect(),
            data: jupiter_data,
        };
        
        anchor_lang::solana_program::program::invoke(
            &jupiter_instruction,
            &jupiter_accounts,
        )?;
        
        // Transfer swapped tokens to user
        let seeds = &[b"liquidity_engine", &[liquidity_engine.bump]];
        let signer = &[&seeds[..]];
        
        let transfer_to_user = Transfer {
            from: ctx.accounts.destination_vault.to_account_info(),
            to: ctx.accounts.user_destination_token.to_account_info(),
            authority: liquidity_engine.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_to_user,
                signer,
            ),
            minimum_amount_out,
        )?;
        
        // Update stats
        liquidity_engine.total_volume = liquidity_engine.total_volume.checked_add(amount_in).unwrap();
        liquidity_engine.total_fees = liquidity_engine.total_fees.checked_add(fee_amount).unwrap();
        
        emit!(SwapExecuted {
            user: ctx.accounts.user.key(),
            amount_in,
            amount_out: minimum_amount_out,
            fee_amount,
        });
        
        Ok(())
    }
}

impl<'info> StakeToJSOL<'info> {
    pub fn process(
        ctx: Context<StakeToJSOL>,
        sol_amount: u64,
    ) -> Result<()> {
        let liquidity_engine = &mut ctx.accounts.liquidity_engine;
        
        require!(!liquidity_engine.is_paused, VaultError::LiquidityEnginePaused);
        
        // Transfer SOL from user
        let transfer_sol = Transfer {
            from: ctx.accounts.user_sol_account.to_account_info(),
            to: ctx.accounts.reserve_stake.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_sol,
            ),
            sol_amount,
        )?;
        
        // Create stake pool deposit instruction via Sanctum
        let sanctum_accounts = vec![
            ctx.accounts.stake_pool.to_account_info(),
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            ctx.accounts.reserve_stake.to_account_info(),
            ctx.accounts.pool_mint.to_account_info(),
            ctx.accounts.pool_token_to.to_account_info(),
            ctx.accounts.validator_list.to_account_info(),
        ];
        
        let deposit_instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.sanctum_program.key(),
            accounts: sanctum_accounts.iter().map(|acc| AccountMeta {
                pubkey: acc.key(),
                is_signer: false,
                is_writable: true,
            }).collect(),
            data: vec![0], // Deposit instruction discriminator
        };
        
        anchor_lang::solana_program::program::invoke(
            &deposit_instruction,
            &sanctum_accounts,
        )?;
        
        emit!(JSOLStaked {
            user: ctx.accounts.user.key(),
            sol_amount,
            jsol_received: sol_amount, // 1:1 initially, varies with rewards
        });
        
        Ok(())
    }
}

impl<'info> InstantUnstakeJSOL<'info> {
    pub fn process(
        ctx: Context<InstantUnstakeJSOL>,
        jsol_amount: u64,
    ) -> Result<()> {
        let liquidity_engine = &mut ctx.accounts.liquidity_engine;
        
        require!(!liquidity_engine.is_paused, VaultError::LiquidityEnginePaused);
        
        // Check reserve pool liquidity
        let reserve_balance = ctx.accounts.reserve_pool.amount;
        require!(reserve_balance >= jsol_amount, VaultError::InsufficientLiquidity);
        
        // Transfer JSOL from user to liquidity pool
        let transfer_jsol = Transfer {
            from: ctx.accounts.user_jsol_account.to_account_info(),
            to: ctx.accounts.liquidity_pool.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_jsol,
            ),
            jsol_amount,
        )?;
        
        // Calculate SOL amount (accounting for staking rewards)
        let exchange_rate = self.get_jsol_to_sol_rate(&ctx.accounts.reserve_pool)?;
        let sol_amount = jsol_amount
            .checked_mul(exchange_rate)
            .unwrap()
            .checked_div(1_000_000_000) // 9 decimals
            .unwrap();
        
        // Transfer SOL from reserve to user instantly
        let seeds = &[b"liquidity_engine", &[liquidity_engine.bump]];
        let signer = &[&seeds[..]];
        
        let transfer_sol = Transfer {
            from: ctx.accounts.reserve_pool.to_account_info(),
            to: ctx.accounts.user_sol_account.to_account_info(),
            authority: liquidity_engine.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_sol,
                signer,
            ),
            sol_amount,
        )?;
        
        emit!(JSOLInstantUnstaked {
            user: ctx.accounts.user.key(),
            jsol_amount,
            sol_received: sol_amount,
            exchange_rate,
        });
        
        Ok(())
    }
    
    fn get_jsol_to_sol_rate(&self, reserve_pool: &Account<TokenAccount>) -> Result<u64> {
        // Simplified exchange rate calculation
        // In production, this would query the actual stake pool state
        Ok(1_050_000_000) // 1.05 SOL per JSOL (5% rewards)
    }
}

impl<'info> CrossChainBridge<'info> {
    pub fn process(
        ctx: Context<CrossChainBridge>,
        amount: u64,
        target_chain: u16,
        recipient: [u8; 32],
    ) -> Result<()> {
        let liquidity_engine = &mut ctx.accounts.liquidity_engine;
        
        require!(!liquidity_engine.is_paused, VaultError::LiquidityEnginePaused);
        
        // Transfer tokens to bridge vault
        let transfer_to_bridge = Transfer {
            from: ctx.accounts.source_token_account.to_account_info(),
            to: ctx.accounts.bridge_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_to_bridge,
            ),
            amount,
        )?;
        
        // Execute cross-chain transfer via bridge
        let bridge_accounts = vec![
            ctx.accounts.bridge_vault.to_account_info(),
            ctx.accounts.bridge_accounts.to_account_info(),
        ];
        
        let bridge_data = [
            &target_chain.to_le_bytes(),
            &recipient,
            &amount.to_le_bytes(),
        ].concat();
        
        let bridge_instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.bridge_program.key(),
            accounts: bridge_accounts.iter().map(|acc| AccountMeta {
                pubkey: acc.key(),
                is_signer: false,
                is_writable: true,
            }).collect(),
            data: bridge_data,
        };
        
        anchor_lang::solana_program::program::invoke(
            &bridge_instruction,
            &bridge_accounts,
        )?;
        
        emit!(CrossChainTransferInitiated {
            user: ctx.accounts.user.key(),
            amount,
            target_chain,
            recipient,
        });
        
        Ok(())
    }
}

// Events
#[event]
pub struct SwapExecuted {
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_amount: u64,
}

#[event]
pub struct JSOLStaked {
    pub user: Pubkey,
    pub sol_amount: u64,
    pub jsol_received: u64,
}

#[event]
pub struct JSOLInstantUnstaked {
    pub user: Pubkey,
    pub jsol_amount: u64,
    pub sol_received: u64,
    pub exchange_rate: u64,
}

#[event]
pub struct CrossChainTransferInitiated {
    pub user: Pubkey,
    pub amount: u64,
    pub target_chain: u16,
    pub recipient: [u8; 32],
}