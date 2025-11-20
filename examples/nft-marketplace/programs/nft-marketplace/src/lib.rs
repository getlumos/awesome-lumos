use anchor_lang::prelude::*;

// Import LUMOS-generated types
mod generated;
use generated::*;

declare_id!("HdYC2wPpQZyPvXZyU8wDVYM2wFoz3KLoPgMWKcqqWAzN");

#[program]
pub mod nft_marketplace {
    use super::*;

    /// Initialize the marketplace with configuration
    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        fee_percentage: u16,
    ) -> Result<()> {
        require!(fee_percentage <= 10000, MarketplaceError::InvalidFeePercentage);

        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.fee_percentage = fee_percentage;
        config.treasury = ctx.accounts.treasury.key();
        config.total_sales = 0;
        config.is_paused = false;

        msg!("Marketplace initialized with {}% fee", fee_percentage / 100);
        Ok(())
    }

    /// List an NFT for sale
    pub fn list_nft(
        ctx: Context<ListNFT>,
        price: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.is_paused, MarketplaceError::MarketplacePaused);
        require!(price > 0, MarketplaceError::InvalidPrice);

        let listing = &mut ctx.accounts.listing;
        let clock = Clock::get()?;

        listing.seller = ctx.accounts.seller.key();
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.price = price;
        listing.listed_at = clock.unix_timestamp;
        listing.status = ListingStatus::Active;
        listing.buyer = None;
        listing.sold_at = None;

        msg!("NFT listed for {} lamports", price);
        Ok(())
    }

    /// Buy a listed NFT
    pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let config = &mut ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.is_paused, MarketplaceError::MarketplacePaused);
        require!(
            matches!(listing.status, ListingStatus::Active),
            MarketplaceError::ListingNotActive
        );

        // Calculate fees
        let price = listing.price;
        let fee = (price as u128 * config.fee_percentage as u128 / 10000) as u64;
        let seller_amount = price - fee;

        // Transfer SOL from buyer to seller
        let buyer_transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.buyer.key,
            &listing.seller,
            seller_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &buyer_transfer_ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Transfer fee to treasury
        if fee > 0 {
            let fee_transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                ctx.accounts.buyer.key,
                &config.treasury,
                fee,
            );
            anchor_lang::solana_program::program::invoke(
                &fee_transfer_ix,
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.treasury_account.clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        // Update listing status
        listing.status = ListingStatus::Sold;
        listing.buyer = Some(ctx.accounts.buyer.key());
        listing.sold_at = Some(clock.unix_timestamp);

        // Update marketplace stats
        config.total_sales += 1;

        msg!("NFT sold for {} lamports (fee: {})", price, fee);
        Ok(())
    }

    /// Cancel a listing
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        require!(
            matches!(listing.status, ListingStatus::Active),
            MarketplaceError::ListingNotActive
        );
        require!(
            listing.seller == ctx.accounts.seller.key(),
            MarketplaceError::Unauthorized
        );

        listing.status = ListingStatus::Cancelled;

        msg!("Listing cancelled");
        Ok(())
    }

    /// Update listing price
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        require!(
            matches!(listing.status, ListingStatus::Active),
            MarketplaceError::ListingNotActive
        );
        require!(
            listing.seller == ctx.accounts.seller.key(),
            MarketplaceError::Unauthorized
        );
        require!(new_price > 0, MarketplaceError::InvalidPrice);

        let old_price = listing.price;
        listing.price = new_price;

        msg!("Price updated from {} to {} lamports", old_price, new_price);
        Ok(())
    }

    /// Pause/unpause marketplace (authority only)
    pub fn set_pause(ctx: Context<SetPause>, paused: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.is_paused = paused;

        msg!("Marketplace {}", if paused { "paused" } else { "unpaused" });
        Ok(())
    }
}

// ============================================================================
// Account Contexts
// ============================================================================

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<MarketplaceConfig>()
    )]
    pub config: Account<'info, MarketplaceConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Treasury account for collecting fees
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + std::mem::size_of::<Listing>()
    )]
    pub listing: Account<'info, Listing>,

    pub config: Account<'info, MarketplaceConfig>,

    #[account(mut)]
    pub seller: Signer<'info>,

    /// CHECK: NFT mint address
    pub nft_mint: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub config: Account<'info, MarketplaceConfig>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Seller account
    #[account(mut)]
    pub seller_account: AccountInfo<'info>,

    /// CHECK: Treasury account
    #[account(mut)]
    pub treasury_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,

    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,

    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub config: Account<'info, MarketplaceConfig>,

    pub authority: Signer<'info>,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid fee percentage (must be <= 100%)")]
    InvalidFeePercentage,

    #[msg("Marketplace is paused")]
    MarketplacePaused,

    #[msg("Invalid price (must be > 0)")]
    InvalidPrice,

    #[msg("Listing is not active")]
    ListingNotActive,

    #[msg("Unauthorized")]
    Unauthorized,
}
