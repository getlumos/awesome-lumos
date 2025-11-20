/**
 * DeFi Staking Client
 * TypeScript client using LUMOS-generated types for type-safe interaction
 */

import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  Keypair,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import { AnchorProvider, Program, Idl, BN } from '@coral-xyz/anchor';

// Import LUMOS-generated types
import {
  StakingPool,
  StakeAccount,
  StakingStatus,
  RewardConfig,
  RewardCalculationType,
  UserStakingStats,
  PoolStats,
} from './generated';

// Program ID (matches lib.rs declare_id!)
const PROGRAM_ID = new PublicKey('stk1111111111111111111111111111111111111111');

/**
 * Client for interacting with the DeFi staking program
 */
export class StakingClient {
  constructor(
    private connection: Connection,
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Initialize a new staking pool
   */
  async initializePool(params: {
    authority: Keypair;
    tokenMint: PublicKey;
    vault: PublicKey;
    rewardRate: number;
    minStakeAmount: number;
    minLockDuration: number;
    cooldownPeriod: number;
  }): Promise<PublicKey> {
    const poolKeypair = Keypair.generate();

    await this.program.methods
      .initializePool(
        new BN(params.rewardRate),
        new BN(params.minStakeAmount),
        new BN(params.minLockDuration),
        new BN(params.cooldownPeriod)
      )
      .accounts({
        pool: poolKeypair.publicKey,
        authority: params.authority.publicKey,
        tokenMint: params.tokenMint,
        vault: params.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([poolKeypair, params.authority])
      .rpc();

    console.log(`Pool initialized: ${poolKeypair.publicKey.toBase58()}`);
    return poolKeypair.publicKey;
  }

  /**
   * Stake tokens into a pool
   */
  async stake(params: {
    pool: PublicKey;
    user: Keypair;
    amount: number;
  }): Promise<void> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), params.pool.toBuffer(), params.user.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const poolData = await this.getPool(params.pool);

    await this.program.methods
      .stake(new BN(params.amount))
      .accounts({
        pool: params.pool,
        stakeAccount,
        user: params.user.publicKey,
        vaultAccount: poolData.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.user])
      .rpc();

    console.log(`Staked ${params.amount} tokens`);
  }

  /**
   * Request unstaking (starts cooldown period)
   */
  async requestUnstake(params: {
    pool: PublicKey;
    user: Keypair;
  }): Promise<void> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), params.pool.toBuffer(), params.user.publicKey.toBuffer()],
      PROGRAM_ID
    );

    await this.program.methods
      .requestUnstake()
      .accounts({
        pool: params.pool,
        stakeAccount,
        user: params.user.publicKey,
      })
      .signers([params.user])
      .rpc();

    console.log('Unstake requested, cooldown period started');
  }

  /**
   * Complete unstaking after cooldown
   */
  async unstake(params: {
    pool: PublicKey;
    user: Keypair;
  }): Promise<void> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), params.pool.toBuffer(), params.user.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const poolData = await this.getPool(params.pool);

    await this.program.methods
      .unstake()
      .accounts({
        pool: params.pool,
        stakeAccount,
        user: params.user.publicKey,
        vaultAccount: poolData.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.user])
      .rpc();

    console.log('Unstaked successfully');
  }

  /**
   * Claim accumulated rewards
   */
  async claimRewards(params: {
    pool: PublicKey;
    user: Keypair;
  }): Promise<void> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), params.pool.toBuffer(), params.user.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const poolData = await this.getPool(params.pool);

    await this.program.methods
      .claimRewards()
      .accounts({
        pool: params.pool,
        stakeAccount,
        user: params.user.publicKey,
        vaultAccount: poolData.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.user])
      .rpc();

    console.log('Rewards claimed');
  }

  /**
   * Emergency withdraw with penalty
   */
  async emergencyWithdraw(params: {
    pool: PublicKey;
    user: Keypair;
  }): Promise<void> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), params.pool.toBuffer(), params.user.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const poolData = await this.getPool(params.pool);

    await this.program.methods
      .emergencyWithdraw()
      .accounts({
        pool: params.pool,
        stakeAccount,
        user: params.user.publicKey,
        vaultAccount: poolData.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.user])
      .rpc();

    console.log('Emergency withdrawal completed (10% penalty applied)');
  }

  /**
   * Update pool parameters (admin only)
   */
  async updatePool(params: {
    pool: PublicKey;
    authority: Keypair;
    rewardRate?: number;
    isActive?: boolean;
  }): Promise<void> {
    await this.program.methods
      .updatePool(
        params.rewardRate ? new BN(params.rewardRate) : null,
        params.isActive !== undefined ? params.isActive : null
      )
      .accounts({
        pool: params.pool,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Pool updated');
  }

  // ===== VIEW FUNCTIONS =====

  /**
   * Fetch pool data with type safety from LUMOS-generated types
   */
  async getPool(poolAddress: PublicKey): Promise<StakingPool> {
    const pool = await this.program.account.stakingPool.fetch(poolAddress);

    // TypeScript knows the exact structure thanks to LUMOS!
    return {
      authority: pool.authority,
      tokenMint: pool.tokenMint,
      vault: pool.vault,
      totalStaked: pool.totalStaked.toNumber(),
      totalStakers: pool.totalStakers.toNumber(),
      rewardRate: pool.rewardRate.toNumber(),
      minStakeAmount: pool.minStakeAmount.toNumber(),
      minLockDuration: pool.minLockDuration.toNumber(),
      cooldownPeriod: pool.cooldownPeriod.toNumber(),
      isActive: pool.isActive,
      createdAt: pool.createdAt.toNumber(),
    };
  }

  /**
   * Fetch user stake account
   */
  async getStakeAccount(
    pool: PublicKey,
    user: PublicKey
  ): Promise<StakeAccount | null> {
    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('stake'), pool.toBuffer(), user.toBuffer()],
      PROGRAM_ID
    );

    try {
      const account = await this.program.account.stakeAccount.fetch(stakeAccount);

      return {
        owner: account.owner,
        pool: account.pool,
        amount: account.amount.toNumber(),
        stakedAt: account.stakedAt.toNumber(),
        lastClaimAt: account.lastClaimAt.toNumber(),
        unlockAt: account.unlockAt.toNumber(),
        totalClaimed: account.totalClaimed.toNumber(),
        status: account.status as StakingStatus,
        unstakeRequestedAt: account.unstakeRequestedAt?.toNumber() || undefined,
      };
    } catch (e) {
      return null;
    }
  }

  /**
   * Calculate current rewards for a user
   */
  async calculateRewards(pool: PublicKey, user: PublicKey): Promise<number> {
    const poolData = await this.getPool(pool);
    const stakeData = await this.getStakeAccount(pool, user);

    if (!stakeData || stakeData.amount === 0) {
      return 0;
    }

    const currentTime = Math.floor(Date.now() / 1000);
    const timeStaked = currentTime - stakeData.lastClaimAt;

    // APY calculation: (amount * rate * time) / (100 * 365 * 86400)
    const reward =
      (stakeData.amount * poolData.rewardRate * timeStaked) /
      (100 * 365 * 86400);

    return reward;
  }

  /**
   * Calculate APY for a pool
   */
  calculateAPY(rewardRate: number): number {
    return rewardRate / 100;
  }

  /**
   * Check if user can unstake
   */
  async canUnstake(pool: PublicKey, user: PublicKey): Promise<boolean> {
    const stakeData = await this.getStakeAccount(pool, user);
    if (!stakeData) return false;

    const currentTime = Math.floor(Date.now() / 1000);

    // Check if unlocked
    const isUnlocked = currentTime >= stakeData.unlockAt;

    // Check if in correct status
    const correctStatus =
      stakeData.status === 'Active' || stakeData.status === 'Locked';

    return isUnlocked && correctStatus;
  }

  /**
   * Check if user can complete unstake (after cooldown)
   */
  async canCompleteUnstake(pool: PublicKey, user: PublicKey): Promise<boolean> {
    const poolData = await this.getPool(pool);
    const stakeData = await this.getStakeAccount(pool, user);

    if (!stakeData || stakeData.status !== 'UnstakeRequested') {
      return false;
    }

    const requestedAt = stakeData.unstakeRequestedAt || 0;
    const currentTime = Math.floor(Date.now() / 1000);
    const cooldownEnd = requestedAt + poolData.cooldownPeriod;

    return currentTime >= cooldownEnd;
  }
}

// ===== USAGE EXAMPLE =====

async function example() {
  const connection = new Connection('https://api.devnet.solana.com');
  const wallet = Keypair.generate();

  // Initialize client
  const client = new StakingClient(connection, {} as Program, {} as AnchorProvider);

  // Create a pool
  const pool = await client.initializePool({
    authority: wallet,
    tokenMint: PublicKey.default,
    vault: PublicKey.default,
    rewardRate: 1000, // 10% APY (1000 basis points)
    minStakeAmount: 0.1 * LAMPORTS_PER_SOL,
    minLockDuration: 7 * 24 * 60 * 60, // 7 days
    cooldownPeriod: 24 * 60 * 60, // 1 day
  });

  // Stake tokens
  await client.stake({
    pool,
    user: wallet,
    amount: 10 * LAMPORTS_PER_SOL,
  });

  // Check rewards
  const rewards = await client.calculateRewards(pool, wallet.publicKey);
  console.log(`Current rewards: ${rewards / LAMPORTS_PER_SOL} SOL`);

  // Claim rewards
  await client.claimRewards({
    pool,
    user: wallet,
  });

  // Request unstake
  await client.requestUnstake({
    pool,
    user: wallet,
  });

  // After cooldown period...
  await client.unstake({
    pool,
    user: wallet,
  });
}
