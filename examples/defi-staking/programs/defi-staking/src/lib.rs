use anchor_lang::prelude::*;

// Import LUMOS-generated types
mod generated;
use generated::*;

declare_id!("stk1111111111111111111111111111111111111111");

#[program]
pub mod defi_staking {
    use super::*;

    /// Initialize a new staking pool with reward configuration
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        reward_rate: u64,
        min_stake_amount: u64,
        min_lock_duration: i64,
        cooldown_period: i64,
    ) -> Result<()> {
        require!(reward_rate > 0 && reward_rate <= 1_000_000, StakingError::InvalidRewardRate);
        require!(min_stake_amount > 0, StakingError::InvalidMinStake);
        require!(min_lock_duration >= 0, StakingError::InvalidLockDuration);
        require!(cooldown_period >= 0, StakingError::InvalidCooldown);

        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;

        pool.authority = ctx.accounts.authority.key();
        pool.token_mint = ctx.accounts.token_mint.key();
        pool.vault = ctx.accounts.vault.key();
        pool.total_staked = 0;
        pool.total_stakers = 0;
        pool.reward_rate = reward_rate;
        pool.min_stake_amount = min_stake_amount;
        pool.min_lock_duration = min_lock_duration;
        pool.cooldown_period = cooldown_period;
        pool.is_active = true;
        pool.created_at = clock.unix_timestamp;

        msg!(
            "Pool initialized with {}% APY, min stake: {}, lock: {}s",
            reward_rate / 100,
            min_stake_amount,
            min_lock_duration
        );

        Ok(())
    }

    /// Stake tokens into the pool
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Validations
        require!(pool.is_active, StakingError::PoolNotActive);
        require!(amount >= pool.min_stake_amount, StakingError::BelowMinStake);

        // Transfer tokens from user to vault (simplified - assumes SOL transfer)
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            &pool.vault,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Initialize or update stake account
        let unlock_time = clock.unix_timestamp + pool.min_lock_duration;

        if stake_account.amount == 0 {
            // First stake
            stake_account.owner = ctx.accounts.user.key();
            stake_account.pool = pool.key();
            stake_account.staked_at = clock.unix_timestamp;
            stake_account.last_claim_at = clock.unix_timestamp;
            stake_account.total_claimed = 0;
            stake_account.status = if pool.min_lock_duration > 0 {
                StakingStatus::Locked
            } else {
                StakingStatus::Active
            };
            stake_account.unstake_requested_at = None;

            pool.total_stakers += 1;
        }

        stake_account.amount += amount;
        stake_account.unlock_at = unlock_time;
        pool.total_staked += amount;

        msg!("Staked {} tokens, unlock at: {}", amount, unlock_time);

        Ok(())
    }

    /// Request unstaking with cooldown period
    pub fn request_unstake(
        ctx: Context<RequestUnstake>,
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Validations
        require!(stake_account.owner == ctx.accounts.user.key(), StakingError::Unauthorized);
        require!(stake_account.amount > 0, StakingError::NoStakedAmount);
        require!(
            clock.unix_timestamp >= stake_account.unlock_at,
            StakingError::StillLocked
        );
        require!(
            matches!(stake_account.status, StakingStatus::Active | StakingStatus::Locked),
            StakingError::InvalidStakingStatus
        );

        stake_account.status = StakingStatus::UnstakeRequested;
        stake_account.unstake_requested_at = Some(clock.unix_timestamp);

        msg!(
            "Unstake requested, can withdraw after: {}",
            clock.unix_timestamp + pool.cooldown_period
        );

        Ok(())
    }

    /// Complete unstaking after cooldown period
    pub fn unstake(
        ctx: Context<Unstake>,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Validations
        require!(stake_account.owner == ctx.accounts.user.key(), StakingError::Unauthorized);
        require!(
            matches!(stake_account.status, StakingStatus::UnstakeRequested),
            StakingError::UnstakeNotRequested
        );

        let requested_at = stake_account.unstake_requested_at.ok_or(StakingError::UnstakeNotRequested)?;
        let cooldown_end = requested_at + pool.cooldown_period;
        require!(
            clock.unix_timestamp >= cooldown_end,
            StakingError::CooldownNotComplete
        );

        let amount = stake_account.amount;

        // Transfer tokens from vault back to user
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.try_borrow_mut_lamports()? += amount;

        // Update state
        pool.total_staked -= amount;
        pool.total_stakers -= 1;
        stake_account.amount = 0;
        stake_account.status = StakingStatus::Unstaked;

        msg!("Unstaked {} tokens", amount);

        Ok(())
    }

    /// Claim accumulated staking rewards
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Validations
        require!(stake_account.owner == ctx.accounts.user.key(), StakingError::Unauthorized);
        require!(stake_account.amount > 0, StakingError::NoStakedAmount);
        require!(
            matches!(stake_account.status, StakingStatus::Active | StakingStatus::Locked),
            StakingError::InvalidStakingStatus
        );

        // Calculate rewards
        let time_staked = clock.unix_timestamp - stake_account.last_claim_at;
        require!(time_staked > 0, StakingError::NoRewardsToClaim);

        // Simplified APY calculation: (amount * rate * time) / (100 * 365 * 86400)
        // rate is in basis points (e.g., 1000 = 10%)
        let reward = (stake_account.amount as u128
            * pool.reward_rate as u128
            * time_staked as u128)
            / (100u128 * 365u128 * 86400u128);

        let reward = reward as u64;
        require!(reward > 0, StakingError::NoRewardsToClaim);

        // Transfer rewards from vault to user
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= reward;
        **ctx.accounts.user.try_borrow_mut_lamports()? += reward;

        // Update state
        stake_account.last_claim_at = clock.unix_timestamp;
        stake_account.total_claimed += reward;

        msg!("Claimed {} tokens in rewards", reward);

        Ok(())
    }

    /// Emergency withdraw with penalty (admin can disable)
    pub fn emergency_withdraw(
        ctx: Context<EmergencyWithdraw>,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let stake_account = &mut ctx.accounts.stake_account;

        // Validations
        require!(stake_account.owner == ctx.accounts.user.key(), StakingError::Unauthorized);
        require!(stake_account.amount > 0, StakingError::NoStakedAmount);

        let amount = stake_account.amount;
        let penalty = amount / 10; // 10% penalty
        let withdraw_amount = amount - penalty;

        // Transfer tokens minus penalty
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= withdraw_amount;
        **ctx.accounts.user.try_borrow_mut_lamports()? += withdraw_amount;

        // Penalty goes to treasury (stays in vault)

        // Update state
        pool.total_staked -= amount;
        pool.total_stakers -= 1;
        stake_account.amount = 0;
        stake_account.status = StakingStatus::Unstaked;

        msg!("Emergency withdraw: {} tokens (penalty: {})", withdraw_amount, penalty);

        Ok(())
    }

    /// Update pool parameters (admin only)
    pub fn update_pool(
        ctx: Context<UpdatePool>,
        reward_rate: Option<u64>,
        is_active: Option<bool>,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(pool.authority == ctx.accounts.authority.key(), StakingError::Unauthorized);

        if let Some(rate) = reward_rate {
            require!(rate > 0 && rate <= 1_000_000, StakingError::InvalidRewardRate);
            pool.reward_rate = rate;
            msg!("Updated reward rate to {}%", rate / 100);
        }

        if let Some(active) = is_active {
            pool.is_active = active;
            msg!("Pool active status: {}", active);
        }

        Ok(())
    }
}

// ===== ACCOUNT CONTEXTS =====

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<StakingPool>()
    )]
    pub pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Token mint for the staking pool
    pub token_mint: AccountInfo<'info>,

    /// CHECK: Vault account to hold staked tokens
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<StakeAccount>(),
        seeds = [b"stake", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault account to hold staked tokens
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestUnstake<'info> {
    pub pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault account holding staked tokens
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    pub pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault account holding reward tokens
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Vault account holding staked tokens
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePool<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,

    pub authority: Signer<'info>,
}

// ===== ERRORS =====

#[error_code]
pub enum StakingError {
    #[msg("Invalid reward rate (must be > 0 and <= 1,000,000 basis points)")]
    InvalidRewardRate,

    #[msg("Invalid minimum stake amount")]
    InvalidMinStake,

    #[msg("Invalid lock duration")]
    InvalidLockDuration,

    #[msg("Invalid cooldown period")]
    InvalidCooldown,

    #[msg("Pool is not active")]
    PoolNotActive,

    #[msg("Amount below minimum stake requirement")]
    BelowMinStake,

    #[msg("Stake is still locked")]
    StillLocked,

    #[msg("No staked amount")]
    NoStakedAmount,

    #[msg("Invalid staking status for this operation")]
    InvalidStakingStatus,

    #[msg("Unstake not requested")]
    UnstakeNotRequested,

    #[msg("Cooldown period not complete")]
    CooldownNotComplete,

    #[msg("No rewards to claim")]
    NoRewardsToClaim,

    #[msg("Unauthorized")]
    Unauthorized,
}
