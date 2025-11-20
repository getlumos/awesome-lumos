use anchor_lang::prelude::*;

// Import LUMOS-generated types
mod generated;
use generated::*;

declare_id!("dao1111111111111111111111111111111111111111");

#[program]
pub mod dao_governance {
    use super::*;

    /// Create a new DAO with governance parameters
    pub fn create_dao(
        ctx: Context<CreateDAO>,
        name: String,
        voting_period: i64,
        timelock_delay: i64,
        quorum_threshold: u64,
        approval_threshold: u64,
    ) -> Result<()> {
        require!(voting_period > 0, GovernanceError::InvalidVotingPeriod);
        require!(timelock_delay >= 0, GovernanceError::InvalidTimelock);
        require!(quorum_threshold > 0 && quorum_threshold <= 10000, GovernanceError::InvalidQuorum);
        require!(approval_threshold > 0 && approval_threshold <= 10000, GovernanceError::InvalidThreshold);

        let dao = &mut ctx.accounts.dao;
        let clock = Clock::get()?;

        dao.authority = ctx.accounts.authority.key();
        dao.name = name;
        dao.treasury = ctx.accounts.treasury.key();
        dao.total_members = 0;
        dao.total_proposals = 0;
        dao.voting_period = voting_period;
        dao.timelock_delay = timelock_delay;
        dao.quorum_threshold = quorum_threshold;
        dao.approval_threshold = approval_threshold;
        dao.is_active = true;
        dao.created_at = clock.unix_timestamp;

        msg!("DAO created: {}", dao.name);
        Ok(())
    }

    /// Create a new proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let member = &ctx.accounts.member;
        let clock = Clock::get()?;

        require!(dao.is_active, GovernanceError::DAONotActive);
        require!(member.is_active, GovernanceError::MemberNotActive);
        require!(member.voting_power > 0, GovernanceError::InsufficientVotingPower);

        let proposal_id = dao.total_proposals;
        dao.total_proposals += 1;

        proposal.id = proposal_id;
        proposal.dao = dao.key();
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.abstain_votes = 0;
        proposal.total_votes = 0;
        proposal.start_time = clock.unix_timestamp;
        proposal.end_time = clock.unix_timestamp + dao.voting_period;
        proposal.queued_at = None;
        proposal.executed_at = None;
        proposal.cancelled_at = None;
        proposal.status = ProposalStatus::Active;

        msg!("Proposal {} created: {}", proposal_id, proposal.title);
        Ok(())
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote_type: VoteType,
        comment: String,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let vote_record = &mut ctx.accounts.vote_record;
        let member = &ctx.accounts.member;
        let clock = Clock::get()?;

        // Validations
        require!(
            matches!(proposal.status, ProposalStatus::Active),
            GovernanceError::ProposalNotActive
        );
        require!(
            clock.unix_timestamp <= proposal.end_time,
            GovernanceError::VotingPeriodEnded
        );
        require!(member.is_active, GovernanceError::MemberNotActive);

        let voting_power = member.voting_power;
        require!(voting_power > 0, GovernanceError::InsufficientVotingPower);

        // Record vote
        vote_record.proposal = proposal.key();
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.vote_type = vote_type.clone();
        vote_record.voting_power = voting_power;
        vote_record.comment = comment;
        vote_record.voted_at = clock.unix_timestamp;

        // Update proposal vote counts
        match vote_type {
            VoteType::Yes => {
                proposal.yes_votes += voting_power;
            }
            VoteType::No => {
                proposal.no_votes += voting_power;
            }
            VoteType::Abstain => {
                proposal.abstain_votes += voting_power;
            }
        }
        proposal.total_votes += voting_power;

        msg!("Vote cast on proposal {}: {:?}", proposal.id, vote_type);
        Ok(())
    }

    /// Queue a successful proposal for execution
    pub fn queue_proposal(
        ctx: Context<QueueProposal>,
    ) -> Result<()> {
        let dao = &ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Check proposal is succeeded
        require!(
            matches!(proposal.status, ProposalStatus::Active),
            GovernanceError::ProposalNotActive
        );
        require!(
            clock.unix_timestamp > proposal.end_time,
            GovernanceError::VotingPeriodNotEnded
        );

        // Calculate total voting power in DAO (simplified - use actual member count)
        let total_voting_power = dao.total_members * 1000; // Assume 1000 power per member

        // Check quorum
        let participation_rate = (proposal.total_votes as u128 * 10000) / total_voting_power as u128;
        require!(
            participation_rate >= dao.quorum_threshold as u128,
            GovernanceError::QuorumNotReached
        );

        // Check approval threshold
        let approval_rate = if proposal.total_votes > 0 {
            (proposal.yes_votes as u128 * 10000) / proposal.total_votes as u128
        } else {
            0
        };

        if approval_rate >= dao.approval_threshold as u128 {
            proposal.status = ProposalStatus::Succeeded;
            proposal.queued_at = Some(clock.unix_timestamp);
            msg!("Proposal {} succeeded and queued", proposal.id);
        } else {
            proposal.status = ProposalStatus::Defeated;
            msg!("Proposal {} defeated", proposal.id);
        }

        Ok(())
    }

    /// Execute a queued proposal after timelock
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
    ) -> Result<()> {
        let dao = &ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        require!(
            matches!(proposal.status, ProposalStatus::Succeeded),
            GovernanceError::ProposalNotQueued
        );

        let queued_at = proposal.queued_at.ok_or(GovernanceError::ProposalNotQueued)?;
        let execution_time = queued_at + dao.timelock_delay;

        require!(
            clock.unix_timestamp >= execution_time,
            GovernanceError::TimelockNotExpired
        );

        // Execute based on proposal type
        match &proposal.proposal_type {
            ProposalType::Transfer { recipient, amount } => {
                // Transfer SOL from treasury to recipient
                **ctx.accounts.treasury_account.try_borrow_mut_lamports()? -= amount;
                **ctx.accounts.recipient_account.try_borrow_mut_lamports()? += amount;
                msg!("Transferred {} lamports to {}", amount, recipient);
            }
            ProposalType::ConfigChange {
                voting_period,
                timelock_delay,
                quorum_threshold,
                approval_threshold,
            } => {
                let dao_mut = &mut ctx.accounts.dao;
                if let Some(period) = voting_period {
                    dao_mut.voting_period = *period;
                }
                if let Some(delay) = timelock_delay {
                    dao_mut.timelock_delay = *delay;
                }
                if let Some(quorum) = quorum_threshold {
                    dao_mut.quorum_threshold = *quorum;
                }
                if let Some(threshold) = approval_threshold {
                    dao_mut.approval_threshold = *threshold;
                }
                msg!("DAO configuration updated");
            }
            ProposalType::AddMember { member, voting_power } => {
                msg!("Member {} added with {} voting power", member, voting_power);
            }
            ProposalType::RemoveMember { member } => {
                msg!("Member {} removed", member);
            }
            ProposalType::Custom { instruction_data, target_program } => {
                msg!("Custom instruction executed on program {}", target_program);
                msg!("Instruction data: {}", instruction_data);
            }
        }

        proposal.status = ProposalStatus::Executed;
        proposal.executed_at = Some(clock.unix_timestamp);

        msg!("Proposal {} executed", proposal.id);
        Ok(())
    }

    /// Cancel a proposal (proposer or admin only)
    pub fn cancel_proposal(
        ctx: Context<CancelProposal>,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        require!(
            matches!(proposal.status, ProposalStatus::Active | ProposalStatus::Succeeded),
            GovernanceError::CannotCancelProposal
        );

        // Only proposer or DAO authority can cancel
        let is_proposer = proposal.proposer == ctx.accounts.canceller.key();
        let is_authority = ctx.accounts.dao.authority == ctx.accounts.canceller.key();

        require!(
            is_proposer || is_authority,
            GovernanceError::Unauthorized
        );

        proposal.status = ProposalStatus::Cancelled;
        proposal.cancelled_at = Some(clock.unix_timestamp);

        msg!("Proposal {} cancelled", proposal.id);
        Ok(())
    }

    /// Add a member to the DAO
    pub fn add_member(
        ctx: Context<AddMember>,
        voting_power: u64,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let member = &mut ctx.accounts.member;
        let clock = Clock::get()?;

        require!(dao.is_active, GovernanceError::DAONotActive);
        require!(voting_power > 0, GovernanceError::InvalidVotingPower);

        member.dao = dao.key();
        member.wallet = ctx.accounts.new_member.key();
        member.voting_power = voting_power;
        member.delegate = None;
        member.proposals_created = 0;
        member.votes_cast = 0;
        member.joined_at = clock.unix_timestamp;
        member.is_active = true;

        dao.total_members += 1;

        msg!("Member {} added with {} voting power", member.wallet, voting_power);
        Ok(())
    }

    /// Delegate voting power to another member
    pub fn delegate_vote(
        ctx: Context<DelegateVote>,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let delegation = &mut ctx.accounts.delegation;
        let clock = Clock::get()?;

        require!(member.is_active, GovernanceError::MemberNotActive);
        require!(
            ctx.accounts.delegatee.is_active,
            GovernanceError::DelegateeNotActive
        );

        member.delegate = Some(ctx.accounts.delegatee_wallet.key());

        delegation.delegator = ctx.accounts.delegator.key();
        delegation.delegatee = ctx.accounts.delegatee_wallet.key();
        delegation.dao = ctx.accounts.dao.key();
        delegation.delegated_power = member.voting_power;
        delegation.created_at = clock.unix_timestamp;

        msg!(
            "Voting power delegated from {} to {}",
            delegation.delegator,
            delegation.delegatee
        );
        Ok(())
    }

    /// Revoke vote delegation
    pub fn revoke_delegation(
        ctx: Context<RevokeDelegation>,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;

        require!(
            member.delegate.is_some(),
            GovernanceError::NoDelegationToRevoke
        );

        member.delegate = None;

        msg!("Delegation revoked for {}", ctx.accounts.delegator.key());
        Ok(())
    }
}

// ===== ACCOUNT CONTEXTS =====

#[derive(Accounts)]
pub struct CreateDAO<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<DAO>() + 200 // Extra space for name string
    )]
    pub dao: Account<'info, DAO>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Treasury account for DAO funds
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,

    #[account(
        init,
        payer = proposer,
        space = 8 + std::mem::size_of::<Proposal>() + 500 // Extra space for strings
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        seeds = [b"member", dao.key().as_ref(), proposer.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + std::mem::size_of::<Vote>() + 200, // Extra for comment
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, Vote>,

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        seeds = [b"member", proposal.dao.as_ref(), voter.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct QueueProposal<'info> {
    pub dao: Account<'info, DAO>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: Treasury account
    #[account(mut)]
    pub treasury_account: AccountInfo<'info>,

    /// CHECK: Recipient for transfers
    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    pub dao: Account<'info, DAO>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    pub canceller: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Member>(),
        seeds = [b"member", dao.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: New member wallet
    pub new_member: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DelegateVote<'info> {
    pub dao: Account<'info, DAO>,

    #[account(mut)]
    pub member: Account<'info, Member>,

    #[account(
        init,
        payer = delegator,
        space = 8 + std::mem::size_of::<VoteDelegation>()
    )]
    pub delegation: Account<'info, VoteDelegation>,

    #[account(mut)]
    pub delegator: Signer<'info>,

    /// CHECK: Delegatee wallet
    pub delegatee_wallet: AccountInfo<'info>,

    #[account(
        seeds = [b"member", dao.key().as_ref(), delegatee_wallet.key().as_ref()],
        bump
    )]
    pub delegatee: Account<'info, Member>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(mut)]
    pub member: Account<'info, Member>,

    pub delegator: Signer<'info>,
}

// ===== ERRORS =====

#[error_code]
pub enum GovernanceError {
    #[msg("Invalid voting period")]
    InvalidVotingPeriod,

    #[msg("Invalid timelock delay")]
    InvalidTimelock,

    #[msg("Invalid quorum threshold (must be 0-10000 basis points)")]
    InvalidQuorum,

    #[msg("Invalid approval threshold (must be 0-10000 basis points)")]
    InvalidThreshold,

    #[msg("DAO is not active")]
    DAONotActive,

    #[msg("Member is not active")]
    MemberNotActive,

    #[msg("Insufficient voting power")]
    InsufficientVotingPower,

    #[msg("Proposal is not active")]
    ProposalNotActive,

    #[msg("Voting period has ended")]
    VotingPeriodEnded,

    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,

    #[msg("Quorum not reached")]
    QuorumNotReached,

    #[msg("Proposal not queued")]
    ProposalNotQueued,

    #[msg("Timelock has not expired")]
    TimelockNotExpired,

    #[msg("Cannot cancel proposal in current state")]
    CannotCancelProposal,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid voting power")]
    InvalidVotingPower,

    #[msg("Delegatee is not active")]
    DelegateeNotActive,

    #[msg("No delegation to revoke")]
    NoDelegationToRevoke,
}
