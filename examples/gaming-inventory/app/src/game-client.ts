/**
 * Gaming Inventory Client
 * TypeScript client using LUMOS-generated types for type-safe game operations
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
  Player,
  GameItem,
  ItemType,
  Rarity,
  ConsumableEffect,
  CraftingRecipe,
  TradeOffer,
  TradeStatus,
  Achievement,
  AchievementType,
  GameStats,
} from './generated';

// Program ID (matches lib.rs declare_id!)
const PROGRAM_ID = new PublicKey('game111111111111111111111111111111111111111');

/**
 * Client for interacting with the gaming inventory program
 */
export class GameClient {
  constructor(
    private connection: Connection,
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Create a new player account
   */
  async createPlayer(params: {
    authority: Keypair;
    username: string;
  }): Promise<PublicKey> {
    const [playerPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), params.authority.publicKey.toBuffer()],
      PROGRAM_ID
    );

    await this.program.methods
      .createPlayer(params.username)
      .accounts({
        player: playerPDA,
        authority: params.authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([params.authority])
      .rpc();

    console.log(`Player created: ${params.username}`);
    return playerPDA;
  }

  /**
   * Mint a new game item
   */
  async mintItem(params: {
    player: PublicKey;
    authority: Keypair;
    name: string;
    itemType: ItemType;
    rarity: Rarity;
    levelRequirement: number;
    power: number;
    defense: number;
  }): Promise<PublicKey> {
    const itemKeypair = Keypair.generate();

    await this.program.methods
      .mintItem(
        params.name,
        params.itemType,
        params.rarity,
        params.levelRequirement,
        params.power,
        params.defense
      )
      .accounts({
        player: params.player,
        item: itemKeypair.publicKey,
        authority: params.authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([itemKeypair, params.authority])
      .rpc();

    console.log(`Item minted: ${params.name}`);
    return itemKeypair.publicKey;
  }

  /**
   * Equip an item
   */
  async equipItem(params: {
    player: PublicKey;
    item: PublicKey;
    authority: Keypair;
  }): Promise<void> {
    await this.program.methods
      .equipItem()
      .accounts({
        player: params.player,
        item: params.item,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Item equipped');
  }

  /**
   * Unequip an item
   */
  async unequipItem(params: {
    player: PublicKey;
    item: PublicKey;
    authority: Keypair;
  }): Promise<void> {
    await this.program.methods
      .unequipItem()
      .accounts({
        player: params.player,
        item: params.item,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Item unequipped');
  }

  /**
   * Craft a new item
   */
  async craftItem(params: {
    player: PublicKey;
    authority: Keypair;
    name: string;
    itemType: ItemType;
    rarity: Rarity;
    power: number;
  }): Promise<PublicKey> {
    const itemKeypair = Keypair.generate();

    await this.program.methods
      .craftItem(params.name, params.itemType, params.rarity, params.power)
      .accounts({
        player: params.player,
        item: itemKeypair.publicKey,
        authority: params.authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([itemKeypair, params.authority])
      .rpc();

    console.log(`Item crafted: ${params.name}`);
    return itemKeypair.publicKey;
  }

  /**
   * Upgrade an item's power
   */
  async upgradeItem(params: {
    player: PublicKey;
    item: PublicKey;
    authority: Keypair;
    powerIncrease: number;
  }): Promise<void> {
    await this.program.methods
      .upgradeItem(params.powerIncrease)
      .accounts({
        player: params.player,
        item: params.item,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log(`Item upgraded (+${params.powerIncrease} power)`);
  }

  /**
   * Consume a consumable item
   */
  async consumeItem(params: {
    player: PublicKey;
    item: PublicKey;
    authority: Keypair;
  }): Promise<void> {
    await this.program.methods
      .consumeItem()
      .accounts({
        player: params.player,
        item: params.item,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Item consumed');
  }

  /**
   * Defeat a monster and earn rewards
   */
  async defeatMonster(params: {
    player: PublicKey;
    authority: Keypair;
    monsterLevel: number;
  }): Promise<void> {
    await this.program.methods
      .defeatMonster(params.monsterLevel)
      .accounts({
        player: params.player,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log(`Monster defeated (Level ${params.monsterLevel})`);
  }

  /**
   * Create a trade offer
   */
  async createTradeOffer(params: {
    player: PublicKey;
    item: PublicKey;
    authority: Keypair;
    requestedGold?: number;
    requestedItemType?: ItemType;
    requestedMinRarity?: Rarity;
    recipient?: PublicKey;
    duration: number;
  }): Promise<PublicKey> {
    const tradeKeypair = Keypair.generate();

    await this.program.methods
      .createTradeOffer(
        params.requestedGold ? new BN(params.requestedGold) : null,
        params.requestedItemType || null,
        params.requestedMinRarity || null,
        params.recipient || null,
        new BN(params.duration)
      )
      .accounts({
        player: params.player,
        item: params.item,
        trade: tradeKeypair.publicKey,
        authority: params.authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([tradeKeypair, params.authority])
      .rpc();

    console.log('Trade offer created');
    return tradeKeypair.publicKey;
  }

  /**
   * Accept a trade offer
   */
  async acceptTrade(params: {
    trade: PublicKey;
    offererPlayer: PublicKey;
    accepterPlayer: PublicKey;
    offeredItem: PublicKey;
    paymentItem: PublicKey;
    authority: Keypair;
    goldPayment?: number;
  }): Promise<void> {
    await this.program.methods
      .acceptTrade(params.goldPayment ? new BN(params.goldPayment) : null)
      .accounts({
        trade: params.trade,
        offererPlayer: params.offererPlayer,
        accepterPlayer: params.accepterPlayer,
        offeredItem: params.offeredItem,
        paymentItem: params.paymentItem,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Trade accepted');
  }

  /**
   * Cancel a trade offer
   */
  async cancelTrade(params: {
    player: PublicKey;
    trade: PublicKey;
    authority: Keypair;
  }): Promise<void> {
    await this.program.methods
      .cancelTrade()
      .accounts({
        player: params.player,
        trade: params.trade,
        authority: params.authority.publicKey,
      })
      .signers([params.authority])
      .rpc();

    console.log('Trade cancelled');
  }

  // ===== VIEW FUNCTIONS =====

  /**
   * Fetch player data with type safety from LUMOS-generated types
   */
  async getPlayer(playerAddress: PublicKey): Promise<Player> {
    const player = await this.program.account.player.fetch(playerAddress);

    return {
      wallet: player.wallet,
      username: player.username,
      level: player.level,
      experience: player.experience.toNumber(),
      gold: player.gold.toNumber(),
      health: player.health,
      mana: player.mana,
      equippedWeapon: player.equippedWeapon || undefined,
      equippedArmor: player.equippedArmor || undefined,
      equippedAccessory: player.equippedAccessory || undefined,
      totalItemsOwned: player.totalItemsOwned.toNumber(),
      totalMonstersDefeated: player.totalMonstersDefeated.toNumber(),
      createdAt: player.createdAt.toNumber(),
      lastLogin: player.lastLogin.toNumber(),
    };
  }

  /**
   * Fetch item data
   */
  async getItem(itemAddress: PublicKey): Promise<GameItem> {
    const item = await this.program.account.gameItem.fetch(itemAddress);

    return {
      id: item.id.toNumber(),
      owner: item.owner,
      name: item.name,
      itemType: item.itemType as ItemType,
      rarity: item.rarity as Rarity,
      levelRequirement: item.levelRequirement,
      power: item.power,
      defense: item.defense,
      durability: item.durability,
      maxDurability: item.maxDurability,
      isEquipped: item.isEquipped,
      isTradeable: item.isTradeable,
      createdAt: item.createdAt.toNumber(),
    };
  }

  /**
   * Fetch trade offer data
   */
  async getTradeOffer(tradeAddress: PublicKey): Promise<TradeOffer> {
    const trade = await this.program.account.tradeOffer.fetch(tradeAddress);

    return {
      id: trade.id.toNumber(),
      offerer: trade.offerer,
      offeredItem: trade.offeredItem,
      requestedGold: trade.requestedGold?.toNumber(),
      requestedItemType: trade.requestedItemType as ItemType | undefined,
      requestedMinRarity: trade.requestedMinRarity as Rarity | undefined,
      recipient: trade.recipient || undefined,
      status: trade.status as TradeStatus,
      createdAt: trade.createdAt.toNumber(),
      expiresAt: trade.expiresAt.toNumber(),
    };
  }

  /**
   * Get player stats summary
   */
  async getPlayerStats(playerAddress: PublicKey): Promise<{
    combatLevel: number;
    totalPower: number;
    totalDefense: number;
    nextLevelXP: number;
    progressToNextLevel: number;
  }> {
    const player = await this.getPlayer(playerAddress);

    let totalPower = 0;
    let totalDefense = 0;

    // Get equipped items
    if (player.equippedWeapon) {
      const weapon = await this.getItem(player.equippedWeapon);
      totalPower += weapon.power;
    }

    if (player.equippedArmor) {
      const armor = await this.getItem(player.equippedArmor);
      totalDefense += armor.defense;
    }

    if (player.equippedAccessory) {
      const accessory = await this.getItem(player.equippedAccessory);
      totalPower += accessory.power;
    }

    const nextLevelXP = player.level * 1000;
    const progressToNextLevel = (player.experience / nextLevelXP) * 100;

    return {
      combatLevel: player.level,
      totalPower,
      totalDefense,
      nextLevelXP,
      progressToNextLevel,
    };
  }

  /**
   * Get rarity multiplier for calculations
   */
  getRarityMultiplier(rarity: Rarity): number {
    switch (rarity) {
      case 'Common':
        return 1.0;
      case 'Uncommon':
        return 1.5;
      case 'Rare':
        return 2.0;
      case 'Epic':
        return 3.0;
      case 'Legendary':
        return 5.0;
      case 'Mythic':
        return 10.0;
      default:
        return 1.0;
    }
  }

  /**
   * Calculate item value
   */
  calculateItemValue(item: GameItem): number {
    const basePower = item.power + item.defense;
    const rarityMultiplier = this.getRarityMultiplier(item.rarity);
    const durabilityFactor = item.durability / item.maxDurability;

    return Math.floor(basePower * rarityMultiplier * durabilityFactor * 10);
  }

  /**
   * Get rarity color for UI
   */
  getRarityColor(rarity: Rarity): string {
    switch (rarity) {
      case 'Common':
        return '#9CA3AF'; // Gray
      case 'Uncommon':
        return '#10B981'; // Green
      case 'Rare':
        return '#3B82F6'; // Blue
      case 'Epic':
        return '#A855F7'; // Purple
      case 'Legendary':
        return '#F59E0B'; // Orange
      case 'Mythic':
        return '#EF4444'; // Red
      default:
        return '#9CA3AF';
    }
  }

  /**
   * Format item type for display
   */
  formatItemType(itemType: ItemType): string {
    if ('Weapon' in itemType) {
      return `Weapon (${itemType.Weapon.damage} DMG, ${itemType.Weapon.attackSpeed} SPD)`;
    } else if ('Armor' in itemType) {
      return `Armor (${itemType.Armor.defenseBonus} DEF, +${itemType.Armor.healthBonus} HP)`;
    } else if ('Accessory' in itemType) {
      return `Accessory (+${itemType.Accessory.manaBonus} MP, +${itemType.Accessory.luckBonus} LCK)`;
    } else if ('Consumable' in itemType) {
      return `Consumable (${itemType.Consumable.uses} uses)`;
    } else if ('Material' in itemType) {
      return `Material (Tier ${itemType.Material.tier})`;
    }
    return 'Unknown';
  }
}

// ===== USAGE EXAMPLE =====

async function example() {
  const connection = new Connection('https://api.devnet.solana.com');
  const wallet = Keypair.generate();

  // Initialize client
  const client = new GameClient(
    connection,
    {} as Program,
    {} as AnchorProvider
  );

  // Create a player
  const player = await client.createPlayer({
    authority: wallet,
    username: 'DragonSlayer',
  });

  // Mint a legendary sword
  const sword = await client.mintItem({
    player,
    authority: wallet,
    name: 'Excalibur',
    itemType: {
      Weapon: {
        damage: 150,
        attackSpeed: 80,
      },
    } as ItemType,
    rarity: 'Legendary',
    levelRequirement: 10,
    power: 150,
    defense: 0,
  });

  // Equip the sword
  await client.equipItem({
    player,
    item: sword,
    authority: wallet,
  });

  // Defeat a monster
  await client.defeatMonster({
    player,
    authority: wallet,
    monsterLevel: 5,
  });

  // Check player stats
  const playerData = await client.getPlayer(player);
  console.log(`Level: ${playerData.level}`);
  console.log(`XP: ${playerData.experience}`);
  console.log(`Gold: ${playerData.gold}`);

  const stats = await client.getPlayerStats(player);
  console.log(`Combat Level: ${stats.combatLevel}`);
  console.log(`Total Power: ${stats.totalPower}`);
  console.log(`Progress to next level: ${stats.progressToNextLevel.toFixed(2)}%`);

  // Craft a new item
  const helmet = await client.craftItem({
    player,
    authority: wallet,
    name: 'Iron Helmet',
    itemType: {
      Armor: {
        defenseBonus: 50,
        healthBonus: 100,
      },
    } as ItemType,
    rarity: 'Rare',
    power: 0,
  });

  // Upgrade the helmet
  await client.upgradeItem({
    player,
    item: helmet,
    authority: wallet,
    powerIncrease: 10,
  });

  // Create a trade offer
  const trade = await client.createTradeOffer({
    player,
    item: helmet,
    authority: wallet,
    requestedGold: 500,
    duration: 7 * 24 * 60 * 60, // 7 days
  });

  console.log('Trade created:', trade.toBase58());
}
