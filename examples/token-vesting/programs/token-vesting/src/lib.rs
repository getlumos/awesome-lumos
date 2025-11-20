use anchor_lang::prelude::*;

// Import LUMOS-generated types
mod generated;
use generated::*;

declare_id!("vest111111111111111111111111111111111111111");

const SECONDS_PER_DAY: i64 = 86400;
const BASIS_POINTS: u64 = 10000;

#[program]
pub mod token_vesting {
    use super::*;

    /// Create a new vesting pool
    pub fn create_pool(
        ctx: Context<CreatePool>,
        name: String,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;

        pool.authority = ctx.accounts.authority.key();
        pool.token_mint = ctx.accounts.token_mint.key();
        pool.vault = ctx.accounts.vault.key();
        pool.name = name;
        pool.total_schedules = 0;
        pool.total_allocated = 0;
        pool.total_released = 0;
        pool.total_revoked = 0;
        pool.is_active = true;
        pool.created_at = clock.unix_timestamp;

        msg!("Vesting pool created: {}", pool.name);
        Ok(())
    }

    /// Create a vesting schedule for a beneficiary
    pub fn create_schedule(
        ctx: Context<CreateSchedule>,
        vesting_type: VestingType,
        total_amount: u64,
        start_time: i64,
        duration: i64,
        is_revocable: bool,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let schedule = &mut ctx.accounts.schedule;
        let beneficiary_account = &mut ctx.accounts.beneficiary_account;
        let clock = Clock::get()?;

        require!(pool.is_active, VestingError::PoolNotActive);
        require!(total_amount > 0, VestingError::InvalidAmount);
        require!(duration > 0, VestingError::InvalidDuration);

        let end_time = start_time + duration;
        let cliff_time = match &vesting_type {
            VestingType::Cliff { cliff_duration } => Some(start_time + cliff_duration),
            VestingType::CliffLinear { cliff_duration, .. } => Some(start_time + cliff_duration),
            _ => None,
        };

        schedule.pool = pool.key();
        schedule.beneficiary = ctx.accounts.beneficiary.key();
        schedule.vesting_type = vesting_type;
        schedule.total_amount = total_amount;
        schedule.released_amount = 0;
        schedule.start_time = start_time;
        schedule.end_time = end_time;
        schedule.cliff_time = cliff_time;
        schedule.last_release_time = start_time;
        schedule.is_revocable = is_revocable;
        schedule.is_revoked = false;
        schedule.revoked_at = None;
        schedule.created_at = clock.unix_timestamp;

        // Update pool stats
        pool.total_schedules += 1;
        pool.total_allocated += total_amount;

        // Initialize or update beneficiary account
        if beneficiary_account.schedules_count == 0 {
            beneficiary_account.wallet = ctx.accounts.beneficiary.key();
            beneficiary_account.pool = pool.key();
            beneficiary_account.first_vesting_at = start_time;
        }
        beneficiary_account.total_allocated += total_amount;
        beneficiary_account.schedules_count += 1;
        beneficiary_account.last_release_at = 0;
        beneficiary_account.total_released = 0;
        beneficiary_account.total_revoked = 0;

        msg!(
            "Vesting schedule created for {} tokens over {} days",
            total_amount,
            duration / SECONDS_PER_DAY
        );
        Ok(())
    }

    /// Release vested tokens to beneficiary
    pub fn release_tokens(
        ctx: Context<ReleaseTokens>,
    ) -> Result<()> {
        let schedule = &mut ctx.accounts.schedule;
        let pool = &mut ctx.accounts.pool;
        let beneficiary_account = &mut ctx.accounts.beneficiary_account;
        let clock = Clock::get()?;

        require!(!schedule.is_revoked, VestingError::ScheduleRevoked);
        require!(
            schedule.beneficiary == ctx.accounts.beneficiary.key(),
            VestingError::Unauthorized
        );

        let vested_amount = calculate_vested_amount(schedule, clock.unix_timestamp)?;
        let releasable = vested_amount.saturating_sub(schedule.released_amount);

        require!(releasable > 0, VestingError::NoTokensToRelease);

        // Transfer tokens from vault to beneficiary (simplified - assumes SOL)
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= releasable;
        **ctx.accounts.beneficiary.try_borrow_mut_lamports()? += releasable;

        // Update schedule
        schedule.released_amount += releasable;
        schedule.last_release_time = clock.unix_timestamp;

        // Update pool stats
        pool.total_released += releasable;

        // Update beneficiary stats
        beneficiary_account.total_released += releasable;
        beneficiary_account.last_release_at = clock.unix_timestamp;

        msg!("Released {} tokens to beneficiary", releasable);
        Ok(())
    }

    /// Revoke a vesting schedule (authority only)
    pub fn revoke_schedule(
        ctx: Context<RevokeSchedule>,
        reason: String,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let schedule = &mut ctx.accounts.schedule;
        let beneficiary_account = &mut ctx.accounts.beneficiary_account;
        let revocation = &mut ctx.accounts.revocation;
        let clock = Clock::get()?;

        require!(schedule.is_revocable, VestingError::NotRevocable);
        require!(!schedule.is_revoked, VestingError::AlreadyRevoked);
        require!(
            pool.authority == ctx.accounts.authority.key(),
            VestingError::Unauthorized
        );

        let vested_amount = calculate_vested_amount(schedule, clock.unix_timestamp)?;
        let unvested = schedule.total_amount.saturating_sub(vested_amount);

        // Mark as revoked
        schedule.is_revoked = true;
        schedule.revoked_at = Some(clock.unix_timestamp);

        // Record revocation
        revocation.schedule = schedule.key();
        revocation.beneficiary = schedule.beneficiary;
        revocation.amount_vested = vested_amount;
        revocation.amount_revoked = unvested;
        revocation.revoked_by = ctx.accounts.authority.key();
        revocation.revoked_at = clock.unix_timestamp;
        revocation.reason = reason;

        // Update stats
        pool.total_revoked += unvested;
        beneficiary_account.total_revoked += unvested;

        // Return unvested tokens to pool (simplified - assumes SOL)
        if unvested > 0 {
            **ctx.accounts.beneficiary.try_borrow_mut_lamports()? -= unvested;
            **ctx.accounts.vault_account.try_borrow_mut_lamports()? += unvested;
        }

        msg!("Schedule revoked: {} tokens returned", unvested);
        Ok(())
    }

    /// Update beneficiary (authority only)
    pub fn update_beneficiary(
        ctx: Context<UpdateBeneficiary>,
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        let schedule = &mut ctx.accounts.schedule;

        require!(
            pool.authority == ctx.accounts.authority.key(),
            VestingError::Unauthorized
        );
        require!(!schedule.is_revoked, VestingError::ScheduleRevoked);

        let old_beneficiary = schedule.beneficiary;
        schedule.beneficiary = ctx.accounts.new_beneficiary.key();

        msg!(
            "Beneficiary updated from {} to {}",
            old_beneficiary,
            schedule.beneficiary
        );
        Ok(())
    }

    /// Close completed schedule
    pub fn close_schedule(
        ctx: Context<CloseSchedule>,
    ) -> Result<()> {
        let schedule = &ctx.accounts.schedule;
        let clock = Clock::get()?;

        require!(
            schedule.beneficiary == ctx.accounts.beneficiary.key(),
            VestingError::Unauthorized
        );

        // Can only close if fully vested or revoked
        let is_complete = schedule.released_amount == schedule.total_amount;
        let is_past_end = clock.unix_timestamp >= schedule.end_time;

        require!(
            is_complete || schedule.is_revoked || is_past_end,
            VestingError::ScheduleNotComplete
        );

        msg!("Schedule closed");
        Ok(())
    }
}

// ===== HELPER FUNCTIONS =====

/// Calculate vested amount based on schedule type
fn calculate_vested_amount(schedule: &VestingSchedule, current_time: i64) -> Result<u64> {
    if current_time < schedule.start_time {
        return Ok(0);
    }

    if schedule.is_revoked {
        // Already vested amount cannot be revoked
        return Ok(schedule.released_amount);
    }

    match &schedule.vesting_type {
        VestingType::Linear => {
            calculate_linear_vesting(schedule, current_time)
        }
        VestingType::Cliff { cliff_duration } => {
            let cliff_time = schedule.start_time + cliff_duration;
            if current_time < cliff_time {
                Ok(0)
            } else {
                Ok(schedule.total_amount)
            }
        }
        VestingType::CliffLinear {
            cliff_duration,
            cliff_percentage,
        } => {
            let cliff_time = schedule.start_time + cliff_duration;

            if current_time < cliff_time {
                return Ok(0);
            }

            // Calculate cliff amount
            let cliff_amount = (schedule.total_amount as u128 * *cliff_percentage as u128 / BASIS_POINTS as u128) as u64;

            if current_time >= schedule.end_time {
                return Ok(schedule.total_amount);
            }

            // Linear vesting for remaining amount after cliff
            let remaining_amount = schedule.total_amount - cliff_amount;
            let elapsed_since_cliff = current_time - cliff_time;
            let total_vesting_duration = schedule.end_time - cliff_time;

            let linear_vested = if total_vesting_duration > 0 {
                (remaining_amount as u128 * elapsed_since_cliff as u128
                    / total_vesting_duration as u128) as u64
            } else {
                remaining_amount
            };

            Ok(cliff_amount + linear_vested)
        }
        VestingType::Milestone { milestones } => {
            let mut vested = 0u64;
            for milestone in milestones {
                if current_time >= milestone.unlock_time {
                    vested += (schedule.total_amount as u128 * milestone.percentage as u128
                        / BASIS_POINTS as u128) as u64;
                }
            }
            Ok(vested.min(schedule.total_amount))
        }
    }
}

/// Calculate linear vesting amount
fn calculate_linear_vesting(schedule: &VestingSchedule, current_time: i64) -> Result<u64> {
    if current_time >= schedule.end_time {
        return Ok(schedule.total_amount);
    }

    let elapsed = current_time - schedule.start_time;
    let duration = schedule.end_time - schedule.start_time;

    if duration <= 0 {
        return Ok(schedule.total_amount);
    }

    let vested = (schedule.total_amount as u128 * elapsed as u128 / duration as u128) as u64;
    Ok(vested)
}

// ===== ACCOUNT CONTEXTS =====

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<VestingPool>() + 100 // Extra for name
    )]
    pub pool: Account<'info, VestingPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Token mint
    pub token_mint: AccountInfo<'info>,

    /// CHECK: Vault account to hold tokens
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSchedule<'info> {
    #[account(mut)]
    pub pool: Account<'info, VestingPool>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<VestingSchedule>()
    )]
    pub schedule: Account<'info, VestingSchedule>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + std::mem::size_of::<Beneficiary>(),
        seeds = [b"beneficiary", pool.key().as_ref(), beneficiary.key().as_ref()],
        bump
    )]
    pub beneficiary_account: Account<'info, Beneficiary>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Beneficiary wallet
    pub beneficiary: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub pool: Account<'info, VestingPool>,

    #[account(mut)]
    pub schedule: Account<'info, VestingSchedule>,

    #[account(mut)]
    pub beneficiary_account: Account<'info, Beneficiary>,

    #[account(mut)]
    pub beneficiary: Signer<'info>,

    /// CHECK: Vault account
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeSchedule<'info> {
    #[account(mut)]
    pub pool: Account<'info, VestingPool>,

    #[account(mut)]
    pub schedule: Account<'info, VestingSchedule>,

    #[account(mut)]
    pub beneficiary_account: Account<'info, Beneficiary>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<RevocationRecord>() + 200 // Extra for reason
    )]
    pub revocation: Account<'info, RevocationRecord>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Beneficiary account (for returning tokens)
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,

    /// CHECK: Vault account
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBeneficiary<'info> {
    pub pool: Account<'info, VestingPool>,

    #[account(mut)]
    pub schedule: Account<'info, VestingSchedule>,

    pub authority: Signer<'info>,

    /// CHECK: New beneficiary wallet
    pub new_beneficiary: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseSchedule<'info> {
    #[account(mut, close = beneficiary)]
    pub schedule: Account<'info, VestingSchedule>,

    #[account(mut)]
    pub beneficiary: Signer<'info>,
}

// ===== ERRORS =====

#[error_code]
pub enum VestingError {
    #[msg("Vesting pool is not active")]
    PoolNotActive,

    #[msg("Invalid amount (must be > 0)")]
    InvalidAmount,

    #[msg("Invalid duration (must be > 0)")]
    InvalidDuration,

    #[msg("Schedule has been revoked")]
    ScheduleRevoked,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("No tokens available to release")]
    NoTokensToRelease,

    #[msg("Schedule is not revocable")]
    NotRevocable,

    #[msg("Schedule already revoked")]
    AlreadyRevoked,

    #[msg("Schedule is not complete")]
    ScheduleNotComplete,

    #[msg("Calculation overflow")]
    CalculationOverflow,
}
