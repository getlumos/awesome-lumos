/**
 * Token Vesting Client
 * TypeScript client using LUMOS-generated types for type-safe vesting operations
 */

import {
  Connection,
  PublicKey,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import { AnchorProvider, Program, BN } from '@coral-xyz/anchor';

// Import LUMOS-generated types
import {
  VestingPool,
  VestingSchedule,
  VestingType,
  Milestone,
  ReleaseRecord,
  Beneficiary,
  RevocationRecord,
  PoolStats,
  BeneficiaryStats,
} from './generated';

// Program ID (matches lib.rs declare_id!)
const PROGRAM_ID = new PublicKey('vest111111111111111111111111111111111111111');

const SECONDS_PER_DAY = 86400;
const BASIS_POINTS = 10000;

/**
 * Client for interacting with the token vesting program
 */
export class VestingClient {
  constructor(
    private connection: Connection,
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Create a new vesting pool
   */
  async createPool(params: {
    authority: Keypair;
    tokenMint: PublicKey;
    vault: PublicKey;
    name: string;
  }): Promise<PublicKey> {
    const poolKeypair = Keypair.generate();

    await this.program.methods
      .createPool(params.name)
      .accounts({
        pool: poolKeypair.publicKey,
        authority: params.authority.publicKey,
        tokenMint: params.tokenMint,
        vault: params.vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([poolKeypair, params.authority])
      .rpc();

    console.log(`Vesting pool created: ${poolKeypair.publicKey.toBase58()}`);
    return poolKeypair.publicKey;
  }

  /**
   * Create a vesting schedule
   */
  async createSchedule(params: {
    pool: PublicKey;
    authority: Keypair;
    beneficiary: PublicKey;
    vestingType: VestingType;
    totalAmount: number;
    startTime: number;
    duration: number;
    isRevocable: boolean;
  }): Promise<PublicKey> {
    const scheduleKeypair = Keypair.generate();

    const [beneficiaryPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('beneficiary'),
        params.pool.toBuffer(),
        params.beneficiary.toBuffer(),
      ],
      PROGRAM_ID
    );

    await this.program.methods
      .createSchedule(
        params.vestingType,
        new BN(params.totalAmount),
        new BN(params.startTime),
        new BN(params.duration),
        params.isRevocable
      )
      .accounts({
        pool: params.pool,
        schedule: scheduleKeypair.publicKey,
        beneficiaryAccount: beneficiaryPDA,
        authority: params.authority.publicKey,
        beneficiary: params.beneficiary,
        systemProgram: SystemProgram.programId,
      })
      .signers([scheduleKeypair, params.authority])
      .rpc();

    console.log(`Vesting schedule created: ${scheduleKeypair.publicKey.toBase58()}`);
    return scheduleKeypair.publicKey;
  }

  /**
   * Release vested tokens
   */
  async releaseTokens(params: {
    pool: PublicKey;
    schedule: PublicKey;
    beneficiary: Keypair;
    vaultAccount: PublicKey;
  }): Promise<void> {
    const [beneficiaryPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('beneficiary'),
        params.pool.toBuffer(),
        params.beneficiary.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );

    await this.program.methods
      .releaseTokens()
      .accounts({
        pool: params.pool,
        schedule: params.schedule,
        beneficiaryAccount: beneficiaryPDA,
        beneficiary: params.beneficiary.publicKey,
        vaultAccount: params.vaultAccount,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.beneficiary])
      .rpc();

    console.log('Tokens released');
  }

  /**
   * Revoke a vesting schedule
   */
  async revokeSchedule(params: {
    pool: PublicKey;
    schedule: PublicKey;
    authority: Keypair;
    beneficiary: PublicKey;
    vaultAccount: PublicKey;
    reason: string;
  }): Promise<PublicKey> {
    const revocationKeypair = Keypair.generate();

    const [beneficiaryPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('beneficiary'),
        params.pool.toBuffer(),
        params.beneficiary.toBuffer(),
      ],
      PROGRAM_ID
    );

    await this.program.methods
      .revokeSchedule(params.reason)
      .accounts({
        pool: params.pool,
        schedule: params.schedule,
        beneficiaryAccount: beneficiaryPDA,
        revocation: revocationKeypair.publicKey,
        authority: params.authority.publicKey,
        beneficiary: params.beneficiary,
        vaultAccount: params.vaultAccount,
        systemProgram: SystemProgram.programId,
      })
      .signers([revocationKeypair, params.authority])
      .rpc();

    console.log('Schedule revoked');
    return revocationKeypair.publicKey;
  }

  /**
   * Update beneficiary of a schedule
   */
  async updateBeneficiary(params: {
    pool: PublicKey;
    schedule: PublicKey;
    authority: Keypair;
    newBeneficiary: PublicKey;
  }): Promise<void> {
    await this.program.methods
      .updateBeneficiary()
      .accounts({
        pool: params.pool,
        schedule: params.schedule,
        authority: params.authority.publicKey,
        newBeneficiary: params.newBeneficiary,
      })
      .signers([params.authority])
      .rpc();

    console.log('Beneficiary updated');
  }

  /**
   * Close a completed schedule
   */
  async closeSchedule(params: {
    schedule: PublicKey;
    beneficiary: Keypair;
  }): Promise<void> {
    await this.program.methods
      .closeSchedule()
      .accounts({
        schedule: params.schedule,
        beneficiary: params.beneficiary.publicKey,
      })
      .signers([params.beneficiary])
      .rpc();

    console.log('Schedule closed');
  }

  // ===== VIEW FUNCTIONS =====

  /**
   * Fetch vesting pool data
   */
  async getPool(poolAddress: PublicKey): Promise<VestingPool> {
    const pool = await this.program.account.vestingPool.fetch(poolAddress);

    return {
      authority: pool.authority,
      tokenMint: pool.tokenMint,
      vault: pool.vault,
      name: pool.name,
      totalSchedules: pool.totalSchedules.toNumber(),
      totalAllocated: pool.totalAllocated.toNumber(),
      totalReleased: pool.totalReleased.toNumber(),
      totalRevoked: pool.totalRevoked.toNumber(),
      isActive: pool.isActive,
      createdAt: pool.createdAt.toNumber(),
    };
  }

  /**
   * Fetch vesting schedule data
   */
  async getSchedule(scheduleAddress: PublicKey): Promise<VestingSchedule> {
    const schedule = await this.program.account.vestingSchedule.fetch(scheduleAddress);

    return {
      pool: schedule.pool,
      beneficiary: schedule.beneficiary,
      vestingType: schedule.vestingType as VestingType,
      totalAmount: schedule.totalAmount.toNumber(),
      releasedAmount: schedule.releasedAmount.toNumber(),
      startTime: schedule.startTime.toNumber(),
      endTime: schedule.endTime.toNumber(),
      cliffTime: schedule.cliffTime?.toNumber(),
      lastReleaseTime: schedule.lastReleaseTime.toNumber(),
      isRevocable: schedule.isRevocable,
      isRevoked: schedule.isRevoked,
      revokedAt: schedule.revokedAt?.toNumber(),
      createdAt: schedule.createdAt.toNumber(),
    };
  }

  /**
   * Fetch beneficiary data
   */
  async getBeneficiary(
    pool: PublicKey,
    wallet: PublicKey
  ): Promise<Beneficiary | null> {
    const [beneficiaryPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('beneficiary'), pool.toBuffer(), wallet.toBuffer()],
      PROGRAM_ID
    );

    try {
      const beneficiary = await this.program.account.beneficiary.fetch(beneficiaryPDA);

      return {
        wallet: beneficiary.wallet,
        pool: beneficiary.pool,
        totalAllocated: beneficiary.totalAllocated.toNumber(),
        totalReleased: beneficiary.totalReleased.toNumber(),
        totalRevoked: beneficiary.totalRevoked.toNumber(),
        schedulesCount: beneficiary.schedulesCount.toNumber(),
        firstVestingAt: beneficiary.firstVestingAt.toNumber(),
        lastReleaseAt: beneficiary.lastReleaseAt.toNumber(),
      };
    } catch (e) {
      return null;
    }
  }

  // ===== CALCULATION HELPERS =====

  /**
   * Calculate vested amount for a schedule at given time
   */
  calculateVestedAmount(schedule: VestingSchedule, currentTime: number): number {
    if (currentTime < schedule.startTime) {
      return 0;
    }

    if (schedule.isRevoked) {
      return schedule.releasedAmount;
    }

    const vestingType = schedule.vestingType;

    if ('Linear' in vestingType) {
      return this.calculateLinearVesting(schedule, currentTime);
    } else if ('Cliff' in vestingType) {
      const cliffDuration = vestingType.Cliff.cliffDuration;
      const cliffTime = schedule.startTime + cliffDuration;
      return currentTime < cliffTime ? 0 : schedule.totalAmount;
    } else if ('CliffLinear' in vestingType) {
      const { cliffDuration, cliffPercentage } = vestingType.CliffLinear;
      return this.calculateCliffLinearVesting(
        schedule,
        currentTime,
        cliffDuration,
        cliffPercentage
      );
    } else if ('Milestone' in vestingType) {
      const milestones = vestingType.Milestone.milestones;
      return this.calculateMilestoneVesting(schedule, currentTime, milestones);
    }

    return 0;
  }

  /**
   * Calculate linear vesting
   */
  private calculateLinearVesting(
    schedule: VestingSchedule,
    currentTime: number
  ): number {
    if (currentTime >= schedule.endTime) {
      return schedule.totalAmount;
    }

    const elapsed = currentTime - schedule.startTime;
    const duration = schedule.endTime - schedule.startTime;

    if (duration <= 0) {
      return schedule.totalAmount;
    }

    return Math.floor((schedule.totalAmount * elapsed) / duration);
  }

  /**
   * Calculate cliff + linear vesting
   */
  private calculateCliffLinearVesting(
    schedule: VestingSchedule,
    currentTime: number,
    cliffDuration: number,
    cliffPercentage: number
  ): number {
    const cliffTime = schedule.startTime + cliffDuration;

    if (currentTime < cliffTime) {
      return 0;
    }

    const cliffAmount = Math.floor(
      (schedule.totalAmount * cliffPercentage) / BASIS_POINTS
    );

    if (currentTime >= schedule.endTime) {
      return schedule.totalAmount;
    }

    const remainingAmount = schedule.totalAmount - cliffAmount;
    const elapsedSinceCliff = currentTime - cliffTime;
    const totalVestingDuration = schedule.endTime - cliffTime;

    const linearVested =
      totalVestingDuration > 0
        ? Math.floor((remainingAmount * elapsedSinceCliff) / totalVestingDuration)
        : remainingAmount;

    return cliffAmount + linearVested;
  }

  /**
   * Calculate milestone vesting
   */
  private calculateMilestoneVesting(
    schedule: VestingSchedule,
    currentTime: number,
    milestones: Milestone[]
  ): number {
    let vested = 0;
    for (const milestone of milestones) {
      if (currentTime >= milestone.unlockTime) {
        vested += Math.floor((schedule.totalAmount * milestone.percentage) / BASIS_POINTS);
      }
    }
    return Math.min(vested, schedule.totalAmount);
  }

  /**
   * Calculate releasable amount
   */
  calculateReleasableAmount(
    schedule: VestingSchedule,
    currentTime: number
  ): number {
    const vested = this.calculateVestedAmount(schedule, currentTime);
    return Math.max(0, vested - schedule.releasedAmount);
  }

  /**
   * Get vesting progress percentage
   */
  getVestingProgress(schedule: VestingSchedule, currentTime: number): number {
    if (schedule.totalAmount === 0) return 0;

    const vested = this.calculateVestedAmount(schedule, currentTime);
    return (vested / schedule.totalAmount) * 100;
  }

  /**
   * Get days until fully vested
   */
  getDaysUntilFullyVested(schedule: VestingSchedule, currentTime: number): number {
    if (currentTime >= schedule.endTime) {
      return 0;
    }

    const remaining = schedule.endTime - currentTime;
    return Math.ceil(remaining / SECONDS_PER_DAY);
  }

  /**
   * Get days until cliff ends
   */
  getDaysUntilCliffEnd(schedule: VestingSchedule, currentTime: number): number | null {
    if (!schedule.cliffTime) {
      return null;
    }

    if (currentTime >= schedule.cliffTime) {
      return 0;
    }

    const remaining = schedule.cliffTime - currentTime;
    return Math.ceil(remaining / SECONDS_PER_DAY);
  }

  /**
   * Format vesting type for display
   */
  formatVestingType(vestingType: VestingType): string {
    if ('Linear' in vestingType) {
      return 'Linear';
    } else if ('Cliff' in vestingType) {
      const days = Math.floor(vestingType.Cliff.cliffDuration / SECONDS_PER_DAY);
      return `Cliff (${days} days)`;
    } else if ('CliffLinear' in vestingType) {
      const days = Math.floor(vestingType.CliffLinear.cliffDuration / SECONDS_PER_DAY);
      const percentage = vestingType.CliffLinear.cliffPercentage / 100;
      return `Cliff + Linear (${days} days, ${percentage}%)`;
    } else if ('Milestone' in vestingType) {
      const count = vestingType.Milestone.milestones.length;
      return `Milestone (${count} milestones)`;
    }
    return 'Unknown';
  }

  /**
   * Get schedule status
   */
  getScheduleStatus(schedule: VestingSchedule, currentTime: number): string {
    if (schedule.isRevoked) {
      return 'Revoked';
    }

    if (schedule.releasedAmount === schedule.totalAmount) {
      return 'Completed';
    }

    if (currentTime < schedule.startTime) {
      return 'Not Started';
    }

    if (schedule.cliffTime && currentTime < schedule.cliffTime) {
      return 'Cliff Period';
    }

    if (currentTime >= schedule.endTime) {
      return 'Fully Vested';
    }

    return 'Vesting';
  }
}

// ===== USAGE EXAMPLE =====

async function example() {
  const connection = new Connection('https://api.devnet.solana.com');
  const authority = Keypair.generate();
  const beneficiary = Keypair.generate();

  // Initialize client
  const client = new VestingClient(
    connection,
    {} as Program,
    {} as AnchorProvider
  );

  // Create a vesting pool
  const pool = await client.createPool({
    authority,
    tokenMint: Keypair.generate().publicKey,
    vault: Keypair.generate().publicKey,
    name: 'Team Vesting Pool',
  });

  // Linear vesting: 1,000,000 tokens over 4 years
  const linearSchedule = await client.createSchedule({
    pool,
    authority,
    beneficiary: beneficiary.publicKey,
    vestingType: { Linear: {} } as VestingType,
    totalAmount: 1_000_000 * LAMPORTS_PER_SOL,
    startTime: Math.floor(Date.now() / 1000),
    duration: 4 * 365 * SECONDS_PER_DAY, // 4 years
    isRevocable: true,
  });

  // Cliff vesting: 500,000 tokens after 1 year cliff
  const cliffSchedule = await client.createSchedule({
    pool,
    authority,
    beneficiary: beneficiary.publicKey,
    vestingType: {
      Cliff: {
        cliffDuration: 365 * SECONDS_PER_DAY, // 1 year
      },
    } as VestingType,
    totalAmount: 500_000 * LAMPORTS_PER_SOL,
    startTime: Math.floor(Date.now() / 1000),
    duration: 365 * SECONDS_PER_DAY,
    isRevocable: false,
  });

  // Cliff + Linear: 25% after 1 year, then linear over 3 years
  const cliffLinearSchedule = await client.createSchedule({
    pool,
    authority,
    beneficiary: beneficiary.publicKey,
    vestingType: {
      CliffLinear: {
        cliffDuration: 365 * SECONDS_PER_DAY,
        cliffPercentage: 2500, // 25%
      },
    } as VestingType,
    totalAmount: 2_000_000 * LAMPORTS_PER_SOL,
    startTime: Math.floor(Date.now() / 1000),
    duration: 4 * 365 * SECONDS_PER_DAY,
    isRevocable: true,
  });

  // Check schedule details
  const schedule = await client.getSchedule(linearSchedule);
  const currentTime = Math.floor(Date.now() / 1000);

  console.log(`Total Amount: ${schedule.totalAmount}`);
  console.log(`Released: ${schedule.releasedAmount}`);
  console.log(`Type: ${client.formatVestingType(schedule.vestingType)}`);
  console.log(`Status: ${client.getScheduleStatus(schedule, currentTime)}`);

  // Calculate vesting info
  const vested = client.calculateVestedAmount(schedule, currentTime);
  const releasable = client.calculateReleasableAmount(schedule, currentTime);
  const progress = client.getVestingProgress(schedule, currentTime);
  const daysLeft = client.getDaysUntilFullyVested(schedule, currentTime);

  console.log(`Vested: ${vested}`);
  console.log(`Releasable: ${releasable}`);
  console.log(`Progress: ${progress.toFixed(2)}%`);
  console.log(`Days until fully vested: ${daysLeft}`);

  // Release tokens
  if (releasable > 0) {
    await client.releaseTokens({
      pool,
      schedule: linearSchedule,
      beneficiary,
      vaultAccount: Keypair.generate().publicKey,
    });
  }
}
