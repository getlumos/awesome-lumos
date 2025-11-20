/**
 * DAO Governance Client
 * TypeScript client using LUMOS-generated types for type-safe governance operations
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
  DAO,
  Proposal,
  ProposalType,
  ProposalStatus,
  Vote,
  VoteType,
  Member,
  VoteDelegation,
  DAOStats,
  MemberStats,
} from './generated';

// Program ID (matches lib.rs declare_id!)
const PROGRAM_ID = new PublicKey('dao1111111111111111111111111111111111111111');

/**
 * Client for interacting with the DAO governance program
 */
export class GovernanceClient {
  constructor(
    private connection: Connection,
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Create a new DAO
   */
  async createDAO(params: {
    authority: Keypair;
    name: string;
    treasury: PublicKey;
    votingPeriod: number;
    timelockDelay: number;
    quorumThreshold: number;
    approvalThreshold: number;
  }): Promise<PublicKey> {
    const daoKeypair = Keypair.generate();

    await this.program.methods
      .createDao(
        params.name,
        new BN(params.votingPeriod),
        new BN(params.timelockDelay),
        new BN(params.quorumThreshold),
        new BN(params.approvalThreshold)
      )
      .accounts({
        dao: daoKeypair.publicKey,
        authority: params.authority.publicKey,
        treasury: params.treasury,
        systemProgram: SystemProgram.programId,
      })
      .signers([daoKeypair, params.authority])
      .rpc();

    console.log(`DAO created: ${daoKeypair.publicKey.toBase58()}`);
    return daoKeypair.publicKey;
  }

  /**
   * Create a new proposal
   */
  async createProposal(params: {
    dao: PublicKey;
    proposer: Keypair;
    title: string;
    description: string;
    proposalType: ProposalType;
  }): Promise<PublicKey> {
    const proposalKeypair = Keypair.generate();

    const [memberPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('member'), params.dao.toBuffer(), params.proposer.publicKey.toBuffer()],
      PROGRAM_ID
    );

    await this.program.methods
      .createProposal(
        params.title,
        params.description,
        params.proposalType
      )
      .accounts({
        dao: params.dao,
        proposal: proposalKeypair.publicKey,
        proposer: params.proposer.publicKey,
        member: memberPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([proposalKeypair, params.proposer])
      .rpc();

    console.log(`Proposal created: ${proposalKeypair.publicKey.toBase58()}`);
    return proposalKeypair.publicKey;
  }

  /**
   * Cast a vote on a proposal
   */
  async castVote(params: {
    proposal: PublicKey;
    voter: Keypair;
    voteType: VoteType;
    comment: string;
  }): Promise<void> {
    const proposalData = await this.getProposal(params.proposal);

    const [voteRecordPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('vote'),
        params.proposal.toBuffer(),
        params.voter.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );

    const [memberPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('member'),
        proposalData.dao.toBuffer(),
        params.voter.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );

    await this.program.methods
      .castVote(params.voteType, params.comment)
      .accounts({
        proposal: params.proposal,
        voteRecord: voteRecordPDA,
        voter: params.voter.publicKey,
        member: memberPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.voter])
      .rpc();

    console.log(`Vote cast: ${params.voteType}`);
  }

  /**
   * Queue a proposal after voting ends
   */
  async queueProposal(params: {
    dao: PublicKey;
    proposal: PublicKey;
  }): Promise<void> {
    await this.program.methods
      .queueProposal()
      .accounts({
        dao: params.dao,
        proposal: params.proposal,
      })
      .rpc();

    console.log('Proposal queued for execution');
  }

  /**
   * Execute a queued proposal
   */
  async executeProposal(params: {
    dao: PublicKey;
    proposal: PublicKey;
    treasuryAccount: PublicKey;
    recipientAccount: PublicKey;
  }): Promise<void> {
    await this.program.methods
      .executeProposal()
      .accounts({
        dao: params.dao,
        proposal: params.proposal,
        treasuryAccount: params.treasuryAccount,
        recipientAccount: params.recipientAccount,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log('Proposal executed');
  }

  /**
   * Cancel a proposal
   */
  async cancelProposal(params: {
    dao: PublicKey;
    proposal: PublicKey;
    canceller: Keypair;
  }): Promise<void> {
    await this.program.methods
      .cancelProposal()
      .accounts({
        dao: params.dao,
        proposal: params.proposal,
        canceller: params.canceller.publicKey,
      })
      .signers([params.canceller])
      .rpc();

    console.log('Proposal cancelled');
  }

  /**
   * Add a member to the DAO
   */
  async addMember(params: {
    dao: PublicKey;
    authority: Keypair;
    newMember: PublicKey;
    votingPower: number;
  }): Promise<void> {
    const [memberPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('member'), params.dao.toBuffer(), params.newMember.toBuffer()],
      PROGRAM_ID
    );

    await this.program.methods
      .addMember(new BN(params.votingPower))
      .accounts({
        dao: params.dao,
        member: memberPDA,
        authority: params.authority.publicKey,
        newMember: params.newMember,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.authority])
      .rpc();

    console.log(`Member added: ${params.newMember.toBase58()}`);
  }

  /**
   * Delegate voting power
   */
  async delegateVote(params: {
    dao: PublicKey;
    delegator: Keypair;
    delegatee: PublicKey;
  }): Promise<void> {
    const delegationKeypair = Keypair.generate();

    const [memberPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('member'),
        params.dao.toBuffer(),
        params.delegator.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );

    const [delegateePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('member'), params.dao.toBuffer(), params.delegatee.toBuffer()],
      PROGRAM_ID
    );

    await this.program.methods
      .delegateVote()
      .accounts({
        dao: params.dao,
        member: memberPDA,
        delegation: delegationKeypair.publicKey,
        delegator: params.delegator.publicKey,
        delegateeWallet: params.delegatee,
        delegatee: delegateePDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([delegationKeypair, params.delegator])
      .rpc();

    console.log(`Vote delegated to ${params.delegatee.toBase58()}`);
  }

  /**
   * Revoke vote delegation
   */
  async revokeDelegation(params: {
    dao: PublicKey;
    delegator: Keypair;
  }): Promise<void> {
    const [memberPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('member'),
        params.dao.toBuffer(),
        params.delegator.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );

    await this.program.methods
      .revokeDelegation()
      .accounts({
        member: memberPDA,
        delegator: params.delegator.publicKey,
      })
      .signers([params.delegator])
      .rpc();

    console.log('Delegation revoked');
  }

  // ===== VIEW FUNCTIONS =====

  /**
   * Fetch DAO data with type safety from LUMOS-generated types
   */
  async getDAO(daoAddress: PublicKey): Promise<DAO> {
    const dao = await this.program.account.dao.fetch(daoAddress);

    return {
      authority: dao.authority,
      name: dao.name,
      treasury: dao.treasury,
      totalMembers: dao.totalMembers.toNumber(),
      totalProposals: dao.totalProposals.toNumber(),
      votingPeriod: dao.votingPeriod.toNumber(),
      timelockDelay: dao.timelockDelay.toNumber(),
      quorumThreshold: dao.quorumThreshold.toNumber(),
      approvalThreshold: dao.approvalThreshold.toNumber(),
      isActive: dao.isActive,
      createdAt: dao.createdAt.toNumber(),
    };
  }

  /**
   * Fetch proposal data
   */
  async getProposal(proposalAddress: PublicKey): Promise<Proposal> {
    const proposal = await this.program.account.proposal.fetch(proposalAddress);

    return {
      id: proposal.id.toNumber(),
      dao: proposal.dao,
      proposer: proposal.proposer,
      title: proposal.title,
      description: proposal.description,
      proposalType: proposal.proposalType as ProposalType,
      yesVotes: proposal.yesVotes.toNumber(),
      noVotes: proposal.noVotes.toNumber(),
      abstainVotes: proposal.abstainVotes.toNumber(),
      totalVotes: proposal.totalVotes.toNumber(),
      startTime: proposal.startTime.toNumber(),
      endTime: proposal.endTime.toNumber(),
      queuedAt: proposal.queuedAt?.toNumber(),
      executedAt: proposal.executedAt?.toNumber(),
      cancelledAt: proposal.cancelledAt?.toNumber(),
      status: proposal.status as ProposalStatus,
    };
  }

  /**
   * Fetch member data
   */
  async getMember(dao: PublicKey, wallet: PublicKey): Promise<Member | null> {
    const [memberPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('member'), dao.toBuffer(), wallet.toBuffer()],
      PROGRAM_ID
    );

    try {
      const member = await this.program.account.member.fetch(memberPDA);

      return {
        dao: member.dao,
        wallet: member.wallet,
        votingPower: member.votingPower.toNumber(),
        delegate: member.delegate || undefined,
        proposalsCreated: member.proposalsCreated.toNumber(),
        votesCast: member.votesCast.toNumber(),
        joinedAt: member.joinedAt.toNumber(),
        isActive: member.isActive,
      };
    } catch (e) {
      return null;
    }
  }

  /**
   * Check if proposal passed
   */
  async hasProposalPassed(proposal: PublicKey): Promise<boolean> {
    const proposalData = await this.getProposal(proposal);
    const daoData = await this.getDAO(proposalData.dao);

    const currentTime = Math.floor(Date.now() / 1000);

    // Check if voting ended
    if (currentTime <= proposalData.endTime) {
      return false;
    }

    // Calculate participation
    const totalVotingPower = daoData.totalMembers * 1000; // Simplified
    const participationRate = (proposalData.totalVotes * 10000) / totalVotingPower;

    // Check quorum
    if (participationRate < daoData.quorumThreshold) {
      return false;
    }

    // Check approval
    const approvalRate =
      proposalData.totalVotes > 0
        ? (proposalData.yesVotes * 10000) / proposalData.totalVotes
        : 0;

    return approvalRate >= daoData.approvalThreshold;
  }

  /**
   * Check if proposal can be executed
   */
  async canExecuteProposal(proposal: PublicKey): Promise<boolean> {
    const proposalData = await this.getProposal(proposal);

    if (proposalData.status !== 'Succeeded') {
      return false;
    }

    if (!proposalData.queuedAt) {
      return false;
    }

    const daoData = await this.getDAO(proposalData.dao);
    const currentTime = Math.floor(Date.now() / 1000);
    const executionTime = proposalData.queuedAt + daoData.timelockDelay;

    return currentTime >= executionTime;
  }

  /**
   * Get voting results for a proposal
   */
  async getVotingResults(proposal: PublicKey): Promise<{
    yesVotes: number;
    noVotes: number;
    abstainVotes: number;
    totalVotes: number;
    yesPercentage: number;
    noPercentage: number;
    abstainPercentage: number;
    participationRate: number;
  }> {
    const proposalData = await this.getProposal(proposal);
    const daoData = await this.getDAO(proposalData.dao);

    const totalVotingPower = daoData.totalMembers * 1000; // Simplified

    return {
      yesVotes: proposalData.yesVotes,
      noVotes: proposalData.noVotes,
      abstainVotes: proposalData.abstainVotes,
      totalVotes: proposalData.totalVotes,
      yesPercentage:
        proposalData.totalVotes > 0
          ? (proposalData.yesVotes / proposalData.totalVotes) * 100
          : 0,
      noPercentage:
        proposalData.totalVotes > 0
          ? (proposalData.noVotes / proposalData.totalVotes) * 100
          : 0,
      abstainPercentage:
        proposalData.totalVotes > 0
          ? (proposalData.abstainVotes / proposalData.totalVotes) * 100
          : 0,
      participationRate: (proposalData.totalVotes / totalVotingPower) * 100,
    };
  }
}

// ===== USAGE EXAMPLE =====

async function example() {
  const connection = new Connection('https://api.devnet.solana.com');
  const wallet = Keypair.generate();

  // Initialize client
  const client = new GovernanceClient(
    connection,
    {} as Program,
    {} as AnchorProvider
  );

  // Create a DAO
  const dao = await client.createDAO({
    authority: wallet,
    name: 'My DAO',
    treasury: Keypair.generate().publicKey,
    votingPeriod: 7 * 24 * 60 * 60, // 7 days
    timelockDelay: 2 * 24 * 60 * 60, // 2 days
    quorumThreshold: 3000, // 30%
    approvalThreshold: 5100, // 51%
  });

  // Add members
  await client.addMember({
    dao,
    authority: wallet,
    newMember: Keypair.generate().publicKey,
    votingPower: 1000,
  });

  // Create proposal
  const proposal = await client.createProposal({
    dao,
    proposer: wallet,
    title: 'Fund Development',
    description: 'Transfer 10 SOL to development team',
    proposalType: {
      Transfer: {
        recipient: Keypair.generate().publicKey,
        amount: 10 * LAMPORTS_PER_SOL,
      },
    } as ProposalType,
  });

  // Cast vote
  await client.castVote({
    proposal,
    voter: wallet,
    voteType: 'Yes',
    comment: 'Support development!',
  });

  // Check results
  const results = await client.getVotingResults(proposal);
  console.log(`Yes: ${results.yesPercentage}%`);
  console.log(`No: ${results.noPercentage}%`);
  console.log(`Participation: ${results.participationRate}%`);

  // After voting period...
  await client.queueProposal({ dao, proposal });

  // After timelock...
  const canExecute = await client.canExecuteProposal(proposal);
  if (canExecute) {
    await client.executeProposal({
      dao,
      proposal,
      treasuryAccount: Keypair.generate().publicKey,
      recipientAccount: Keypair.generate().publicKey,
    });
  }
}
