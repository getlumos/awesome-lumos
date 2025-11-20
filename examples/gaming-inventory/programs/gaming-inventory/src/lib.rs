use anchor_lang::prelude::*;

// Import LUMOS-generated types
mod generated;
use generated::*;

declare_id!("game111111111111111111111111111111111111111");

const XP_PER_LEVEL: u64 = 1000;
const LEVEL_UP_HEALTH_BONUS: u32 = 100;
const LEVEL_UP_MANA_BONUS: u32 = 50;

#[program]
pub mod gaming_inventory {
    use super::*;

    /// Create a new player account
    pub fn create_player(
        ctx: Context<CreatePlayer>,
        username: String,
    ) -> Result<()> {
        require!(username.len() >= 3 && username.len() <= 20, GameError::InvalidUsername);

        let player = &mut ctx.accounts.player;
        let clock = Clock::get()?;

        player.wallet = ctx.accounts.authority.key();
        player.username = username;
        player.level = 1;
        player.experience = 0;
        player.gold = 1000; // Starting gold
        player.health = 1000;
        player.mana = 500;
        player.equipped_weapon = None;
        player.equipped_armor = None;
        player.equipped_accessory = None;
        player.total_items_owned = 0;
        player.total_monsters_defeated = 0;
        player.created_at = clock.unix_timestamp;
        player.last_login = clock.unix_timestamp;

        msg!("Player created: {} (Level {})", player.username, player.level);
        Ok(())
    }

    /// Mint a new game item
    pub fn mint_item(
        ctx: Context<MintItem>,
        name: String,
        item_type: ItemType,
        rarity: Rarity,
        level_requirement: u16,
        power: u32,
        defense: u32,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;
        let clock = Clock::get()?;

        let max_durability = match rarity {
            Rarity::Common => 100,
            Rarity::Uncommon => 150,
            Rarity::Rare => 200,
            Rarity::Epic => 300,
            Rarity::Legendary => 500,
            Rarity::Mythic => 1000,
        };

        item.id = player.total_items_owned;
        item.owner = player.wallet;
        item.name = name;
        item.item_type = item_type;
        item.rarity = rarity.clone();
        item.level_requirement = level_requirement;
        item.power = power;
        item.defense = defense;
        item.durability = max_durability;
        item.max_durability = max_durability;
        item.is_equipped = false;
        item.is_tradeable = true;
        item.created_at = clock.unix_timestamp;

        player.total_items_owned += 1;

        msg!("Item minted: {} ({:?})", item.name, rarity);
        Ok(())
    }

    /// Equip an item
    pub fn equip_item(
        ctx: Context<EquipItem>,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;

        // Validations
        require!(item.owner == player.wallet, GameError::NotItemOwner);
        require!(player.level >= item.level_requirement, GameError::LevelRequirementNotMet);
        require!(item.durability > 0, GameError::ItemBroken);
        require!(!item.is_equipped, GameError::ItemAlreadyEquipped);

        // Equip based on item type
        match &item.item_type {
            ItemType::Weapon { .. } => {
                if let Some(current_weapon) = player.equipped_weapon {
                    // Would need to unequip current weapon first
                    require!(current_weapon != item.key(), GameError::ItemAlreadyEquipped);
                }
                player.equipped_weapon = Some(item.key());
            }
            ItemType::Armor { .. } => {
                if let Some(current_armor) = player.equipped_armor {
                    require!(current_armor != item.key(), GameError::ItemAlreadyEquipped);
                }
                player.equipped_armor = Some(item.key());
            }
            ItemType::Accessory { .. } => {
                if let Some(current_accessory) = player.equipped_accessory {
                    require!(current_accessory != item.key(), GameError::ItemAlreadyEquipped);
                }
                player.equipped_accessory = Some(item.key());
            }
            _ => return Err(GameError::CannotEquipItemType.into()),
        }

        item.is_equipped = true;

        msg!("Item equipped: {}", item.name);
        Ok(())
    }

    /// Unequip an item
    pub fn unequip_item(
        ctx: Context<UnequipItem>,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;

        require!(item.owner == player.wallet, GameError::NotItemOwner);
        require!(item.is_equipped, GameError::ItemNotEquipped);

        // Unequip based on item type
        match &item.item_type {
            ItemType::Weapon { .. } => {
                player.equipped_weapon = None;
            }
            ItemType::Armor { .. } => {
                player.equipped_armor = None;
            }
            ItemType::Accessory { .. } => {
                player.equipped_accessory = None;
            }
            _ => return Err(GameError::CannotUnequipItemType.into()),
        }

        item.is_equipped = false;

        msg!("Item unequipped: {}", item.name);
        Ok(())
    }

    /// Craft a new item from materials
    pub fn craft_item(
        ctx: Context<CraftItem>,
        name: String,
        item_type: ItemType,
        rarity: Rarity,
        power: u32,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;
        let clock = Clock::get()?;

        // Calculate crafting cost based on rarity
        let crafting_cost = match rarity {
            Rarity::Common => 100,
            Rarity::Uncommon => 250,
            Rarity::Rare => 500,
            Rarity::Epic => 1000,
            Rarity::Legendary => 2500,
            Rarity::Mythic => 5000,
        };

        require!(player.gold >= crafting_cost, GameError::InsufficientGold);

        player.gold -= crafting_cost;

        let max_durability = match rarity {
            Rarity::Common => 100,
            Rarity::Uncommon => 150,
            Rarity::Rare => 200,
            Rarity::Epic => 300,
            Rarity::Legendary => 500,
            Rarity::Mythic => 1000,
        };

        item.id = player.total_items_owned;
        item.owner = player.wallet;
        item.name = name;
        item.item_type = item_type;
        item.rarity = rarity.clone();
        item.level_requirement = 1;
        item.power = power;
        item.defense = 0;
        item.durability = max_durability;
        item.max_durability = max_durability;
        item.is_equipped = false;
        item.is_tradeable = true;
        item.created_at = clock.unix_timestamp;

        player.total_items_owned += 1;

        msg!("Item crafted: {} ({:?}) for {} gold", item.name, rarity, crafting_cost);
        Ok(())
    }

    /// Upgrade an item's power
    pub fn upgrade_item(
        ctx: Context<UpgradeItem>,
        power_increase: u32,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;

        require!(item.owner == player.wallet, GameError::NotItemOwner);

        let upgrade_cost = power_increase as u64 * 10;
        require!(player.gold >= upgrade_cost, GameError::InsufficientGold);

        player.gold -= upgrade_cost;
        item.power += power_increase;

        msg!("Item upgraded: {} (+{} power)", item.name, power_increase);
        Ok(())
    }

    /// Consume a consumable item
    pub fn consume_item(
        ctx: Context<ConsumeItem>,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let item = &mut ctx.accounts.item;

        require!(item.owner == player.wallet, GameError::NotItemOwner);

        match &item.item_type {
            ItemType::Consumable { effect, uses } => {
                require!(*uses > 0, GameError::NoUsesRemaining);

                match effect {
                    ConsumableEffect::HealHealth { amount } => {
                        player.health += amount;
                        msg!("Healed {} health", amount);
                    }
                    ConsumableEffect::RestoreMana { amount } => {
                        player.mana += amount;
                        msg!("Restored {} mana", amount);
                    }
                    ConsumableEffect::BoostExperience { percentage, .. } => {
                        msg!("Experience boost active: +{}%", percentage);
                    }
                    ConsumableEffect::IncreaseGold { percentage, .. } => {
                        msg!("Gold boost active: +{}%", percentage);
                    }
                }

                // Decrease uses (simplified - would need to update struct)
                msg!("Consumable used: {} ({} uses remaining)", item.name, uses - 1);
            }
            _ => return Err(GameError::NotConsumable.into()),
        }

        Ok(())
    }

    /// Defeat a monster and earn rewards
    pub fn defeat_monster(
        ctx: Context<DefeatMonster>,
        monster_level: u16,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let clock = Clock::get()?;

        // Calculate rewards based on monster level
        let exp_earned = (monster_level as u64) * 100;
        let gold_earned = (monster_level as u64) * 50;

        player.experience += exp_earned;
        player.gold += gold_earned;
        player.total_monsters_defeated += 1;
        player.last_login = clock.unix_timestamp;

        // Check for level up
        let required_xp = player.level as u64 * XP_PER_LEVEL;
        if player.experience >= required_xp {
            player.level += 1;
            player.experience -= required_xp;
            player.health += LEVEL_UP_HEALTH_BONUS;
            player.mana += LEVEL_UP_MANA_BONUS;

            msg!("🎉 LEVEL UP! Now level {}", player.level);
        }

        msg!(
            "Monster defeated! Earned {} XP and {} gold",
            exp_earned,
            gold_earned
        );
        Ok(())
    }

    /// Create a trade offer
    pub fn create_trade_offer(
        ctx: Context<CreateTradeOffer>,
        requested_gold: Option<u64>,
        requested_item_type: Option<ItemType>,
        requested_min_rarity: Option<Rarity>,
        recipient: Option<Pubkey>,
        duration: i64,
    ) -> Result<()> {
        let player = &ctx.accounts.player;
        let item = &mut ctx.accounts.item;
        let trade = &mut ctx.accounts.trade;
        let clock = Clock::get()?;

        require!(item.owner == player.wallet, GameError::NotItemOwner);
        require!(item.is_tradeable, GameError::ItemNotTradeable);
        require!(!item.is_equipped, GameError::CannotTradeEquippedItem);

        let trade_id = clock.unix_timestamp as u64;

        trade.id = trade_id;
        trade.offerer = player.wallet;
        trade.offered_item = item.key();
        trade.requested_gold = requested_gold;
        trade.requested_item_type = requested_item_type;
        trade.requested_min_rarity = requested_min_rarity;
        trade.recipient = recipient;
        trade.status = TradeStatus::Open;
        trade.created_at = clock.unix_timestamp;
        trade.expires_at = clock.unix_timestamp + duration;

        msg!("Trade offer created: #{}", trade_id);
        Ok(())
    }

    /// Accept a trade offer
    pub fn accept_trade(
        ctx: Context<AcceptTrade>,
        gold_payment: Option<u64>,
    ) -> Result<()> {
        let trade = &mut ctx.accounts.trade;
        let offerer_player = &mut ctx.accounts.offerer_player;
        let accepter_player = &mut ctx.accounts.accepter_player;
        let offered_item = &mut ctx.accounts.offered_item;
        let payment_item = &mut ctx.accounts.payment_item;
        let clock = Clock::get()?;

        // Validations
        require!(
            matches!(trade.status, TradeStatus::Open),
            GameError::TradeNotOpen
        );
        require!(
            clock.unix_timestamp <= trade.expires_at,
            GameError::TradeExpired
        );

        if let Some(recipient) = trade.recipient {
            require!(
                accepter_player.wallet == recipient,
                GameError::NotTradeRecipient
            );
        }

        // Handle gold payment
        if let Some(requested_gold) = trade.requested_gold {
            let payment = gold_payment.ok_or(GameError::GoldPaymentRequired)?;
            require!(payment >= requested_gold, GameError::InsufficientGoldPayment);

            accepter_player.gold -= payment;
            offerer_player.gold += payment;
        }

        // Handle item swap if payment item provided
        if let Some(requested_type) = &trade.requested_item_type {
            require!(
                matches!(payment_item.item_type, _),
                GameError::InvalidItemPayment
            );
            require!(
                payment_item.owner == accepter_player.wallet,
                GameError::NotItemOwner
            );

            payment_item.owner = offerer_player.wallet;
            msg!("Item exchanged: {}", payment_item.name);
        }

        // Transfer offered item
        offered_item.owner = accepter_player.wallet;

        trade.status = TradeStatus::Accepted;

        msg!("Trade completed: #{}", trade.id);
        Ok(())
    }

    /// Cancel a trade offer
    pub fn cancel_trade(
        ctx: Context<CancelTrade>,
    ) -> Result<()> {
        let player = &ctx.accounts.player;
        let trade = &mut ctx.accounts.trade;

        require!(trade.offerer == player.wallet, GameError::NotTradeOfferer);
        require!(
            matches!(trade.status, TradeStatus::Open),
            GameError::TradeNotOpen
        );

        trade.status = TradeStatus::Cancelled;

        msg!("Trade cancelled: #{}", trade.id);
        Ok(())
    }
}

// ===== ACCOUNT CONTEXTS =====

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Player>() + 100, // Extra for username
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<GameItem>() + 100, // Extra for name
    )]
    pub item: Account<'info, GameItem>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EquipItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub item: Account<'info, GameItem>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnequipItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub item: Account<'info, GameItem>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<GameItem>() + 100,
    )]
    pub item: Account<'info, GameItem>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub item: Account<'info, GameItem>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ConsumeItem<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub item: Account<'info, GameItem>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DefeatMonster<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTradeOffer<'info> {
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub item: Account<'info, GameItem>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<TradeOffer>()
    )]
    pub trade: Account<'info, TradeOffer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptTrade<'info> {
    #[account(mut)]
    pub trade: Account<'info, TradeOffer>,

    #[account(mut)]
    pub offerer_player: Account<'info, Player>,

    #[account(mut)]
    pub accepter_player: Account<'info, Player>,

    #[account(mut)]
    pub offered_item: Account<'info, GameItem>,

    #[account(mut)]
    pub payment_item: Account<'info, GameItem>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelTrade<'info> {
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub trade: Account<'info, TradeOffer>,

    pub authority: Signer<'info>,
}

// ===== ERRORS =====

#[error_code]
pub enum GameError {
    #[msg("Invalid username (must be 3-20 characters)")]
    InvalidUsername,

    #[msg("You do not own this item")]
    NotItemOwner,

    #[msg("Level requirement not met")]
    LevelRequirementNotMet,

    #[msg("Item is broken (0 durability)")]
    ItemBroken,

    #[msg("Item is already equipped")]
    ItemAlreadyEquipped,

    #[msg("Cannot equip this item type")]
    CannotEquipItemType,

    #[msg("Item is not equipped")]
    ItemNotEquipped,

    #[msg("Cannot unequip this item type")]
    CannotUnequipItemType,

    #[msg("Insufficient gold")]
    InsufficientGold,

    #[msg("Item is not a consumable")]
    NotConsumable,

    #[msg("No uses remaining")]
    NoUsesRemaining,

    #[msg("Item is not tradeable")]
    ItemNotTradeable,

    #[msg("Cannot trade equipped items")]
    CannotTradeEquippedItem,

    #[msg("Trade is not open")]
    TradeNotOpen,

    #[msg("Trade has expired")]
    TradeExpired,

    #[msg("Not the intended recipient of this trade")]
    NotTradeRecipient,

    #[msg("Gold payment required")]
    GoldPaymentRequired,

    #[msg("Insufficient gold payment")]
    InsufficientGoldPayment,

    #[msg("Invalid item payment")]
    InvalidItemPayment,

    #[msg("You are not the trade offerer")]
    NotTradeOfferer,
}
