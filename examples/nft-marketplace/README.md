# NFT Marketplace - LUMOS Example

A complete decentralized NFT marketplace built with **LUMOS-generated types**, demonstrating how to use type-safe schemas for real-world Solana development.

## 🎯 What This Example Demonstrates

- **LUMOS Schema Definition** - Define types once in `.lumos` syntax
- **Automatic Code Generation** - Generate synchronized Rust + TypeScript
- **Anchor Integration** - Use generated types in Anchor programs
- **Type Safety** - Guaranteed compatibility between frontend and backend
- **Real-World Patterns** - Enums, Options, timestamps, permissions

---

## 📁 Project Structure

```
nft-marketplace/
├── schema/
│   └── marketplace.lumos          # LUMOS schema definition
├── programs/
│   └── nft-marketplace/
│       └── src/
│           ├── generated.rs        # Generated Rust types
│           └── lib.rs              # Anchor program logic
├── app/
│   └── src/
│       └── generated.ts            # Generated TypeScript types
├── tests/                          # Anchor tests
├── Anchor.toml                     # Anchor configuration
└── README.md                       # This file
```

---

## 🔮 LUMOS Schema

**File:** `schema/marketplace.lumos`

```rust
// Marketplace Configuration
#[solana]
#[account]
struct MarketplaceConfig {
    authority: PublicKey,
    fee_percentage: u16,
    treasury: PublicKey,
    total_sales: u64,
    is_paused: bool,
}

// NFT Listing
#[solana]
#[account]
struct Listing {
    seller: PublicKey,
    nft_mint: PublicKey,
    price: u64,
    listed_at: i64,
    status: ListingStatus,
    buyer: Option<PublicKey>,
    sold_at: Option<i64>,
}

#[solana]
enum ListingStatus {
    Active,
    Sold,
    Cancelled,
}

// NFT Metadata
#[solana]
#[account]
struct NFTMetadata {
    name: String,
    symbol: String,
    uri: String,
    creator: PublicKey,
    collection: Option<PublicKey>,
    royalty_percentage: u16,
}

// Transaction History
#[solana]
struct TransactionRecord {
    transaction_type: TransactionType,
    nft_mint: PublicKey,
    from: PublicKey,
    to: PublicKey,
    price: u64,
    timestamp: i64,
}

#[solana]
enum TransactionType {
    Listed,
    Sold,
    Cancelled,
    PriceUpdated { old_price: u64, new_price: u64 },
}

// User Profile
#[solana]
#[account]
struct UserProfile {
    owner: PublicKey,
    total_listed: u64,
    total_sold: u64,
    total_purchased: u64,
    joined_at: i64,
}
```

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- Solana CLI 1.18+
- Anchor 0.32.1
- Node.js 18+
- **LUMOS CLI** (`cargo install lumos-cli`)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/getlumos/awesome-lumos
   cd awesome-lumos/examples/nft-marketplace
   ```

2. **Install dependencies:**
   ```bash
   yarn install
   ```

3. **Build the program:**
   ```bash
   anchor build
   ```

4. **Run tests:**
   ```bash
   anchor test
   ```

---

## 🔧 Generating Code from Schema

If you modify the schema, regenerate the code:

```bash
# Generate Rust and TypeScript
lumos generate schema/marketplace.lumos --output programs

# Move generated files to correct locations
mv programs/generated.rs programs/nft-marketplace/src/
mv programs/generated.ts app/src/
```

**Generated Files:**
- `programs/nft-marketplace/src/generated.rs` - Rust types with `#[account]` macros
- `app/src/generated.ts` - TypeScript interfaces + Borsh schemas

---

## 📚 Program Instructions

### 1. Initialize Marketplace

Create a new marketplace with fee configuration.

```rust
pub fn initialize_marketplace(
    ctx: Context<InitializeMarketplace>,
    fee_percentage: u16,
) -> Result<()>
```

**Parameters:**
- `fee_percentage` - Fee in basis points (e.g., 250 = 2.5%)

**Example:**
```typescript
await program.methods
  .initializeMarketplace(250) // 2.5% fee
  .accounts({
    config: marketplaceConfigPda,
    authority: provider.wallet.publicKey,
    treasury: treasuryPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

---

### 2. List NFT

List an NFT for sale on the marketplace.

```rust
pub fn list_nft(
    ctx: Context<ListNFT>,
    price: u64,
) -> Result<()>
```

**Parameters:**
- `price` - Listing price in lamports

**Example:**
```typescript
import { Listing, ListingBorshSchema } from './generated';

await program.methods
  .listNft(new BN(1_000_000_000)) // 1 SOL
  .accounts({
    listing: listingPda,
    config: marketplaceConfigPda,
    seller: provider.wallet.publicKey,
    nftMint: nftMintPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

// Fetch listing data
const listingAccount = await connection.getAccountInfo(listingPda);
const listing = borsh.deserialize(
  ListingBorshSchema,
  listingAccount.data
) as Listing;

console.log('Listed at:', new Date(listing.listed_at * 1000));
console.log('Status:', listing.status.kind); // 'Active'
```

---

### 3. Buy NFT

Purchase a listed NFT.

```rust
pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()>
```

**Example:**
```typescript
await program.methods
  .buyNft()
  .accounts({
    listing: listingPda,
    config: marketplaceConfigPda,
    buyer: provider.wallet.publicKey,
    sellerAccount: sellerPublicKey,
    treasuryAccount: treasuryPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

**What Happens:**
1. Calculates marketplace fee
2. Transfers SOL to seller (price - fee)
3. Transfers fee to treasury
4. Updates listing status to `Sold`
5. Records buyer and timestamp

---

### 4. Cancel Listing

Cancel an active listing (seller only).

```rust
pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()>
```

**Example:**
```typescript
await program.methods
  .cancelListing()
  .accounts({
    listing: listingPda,
    seller: provider.wallet.publicKey,
  })
  .rpc();
```

---

### 5. Update Price

Change the price of an active listing (seller only).

```rust
pub fn update_price(
    ctx: Context<UpdatePrice>,
    new_price: u64,
) -> Result<()>
```

**Example:**
```typescript
await program.methods
  .updatePrice(new BN(2_000_000_000)) // 2 SOL
  .accounts({
    listing: listingPda,
    seller: provider.wallet.publicKey,
  })
  .rpc();
```

---

### 6. Set Pause

Pause/unpause the marketplace (authority only).

```rust
pub fn set_pause(
    ctx: Context<SetPause>,
    paused: bool,
) -> Result<()>
```

**Example:**
```typescript
await program.methods
  .setPause(true) // Pause marketplace
  .accounts({
    config: marketplaceConfigPda,
    authority: provider.wallet.publicKey,
  })
  .rpc();
```

---

## 🎨 Using Generated TypeScript Types

The generated TypeScript types provide full type safety:

```typescript
import {
  Listing,
  ListingStatus,
  MarketplaceConfig,
  TransactionType,
  ListingBorshSchema,
  MarketplaceConfigBorshSchema,
} from './generated';

// Type-safe listing
const listing: Listing = {
  seller: new PublicKey('...'),
  nft_mint: new PublicKey('...'),
  price: 1_000_000_000,
  listed_at: Date.now() / 1000,
  status: { kind: 'Active' },
  buyer: undefined,
  sold_at: undefined,
};

// Serialize for on-chain storage
const buffer = borsh.serialize(ListingBorshSchema, listing);

// Deserialize from account data
const accountData = await connection.getAccountInfo(listingPda);
const deserializedListing = borsh.deserialize(
  ListingBorshSchema,
  accountData.data
) as Listing;

// Type narrowing with discriminated unions
if (deserializedListing.status.kind === 'Sold') {
  console.log('Buyer:', deserializedListing.buyer); // TypeScript knows buyer exists
  console.log('Sold at:', new Date(deserializedListing.sold_at! * 1000));
}
```

---

## 🧪 Testing

The example includes comprehensive tests:

```bash
# Run all tests
anchor test

# Run with logs
anchor test --skip-local-validator
```

**Test Coverage:**
- ✅ Marketplace initialization
- ✅ Listing creation
- ✅ Purchasing flow
- ✅ Fee calculation
- ✅ Cancellation
- ✅ Price updates
- ✅ Pause/unpause
- ✅ Permission checks

---

## 🔐 Security Considerations

### Implemented

- ✅ **Authority checks** - Only authority can pause marketplace
- ✅ **Seller verification** - Only seller can cancel/update listing
- ✅ **Status validation** - Can only buy/cancel active listings
- ✅ **Price validation** - Must be > 0
- ✅ **Fee validation** - Must be <= 100%
- ✅ **Pause mechanism** - Emergency stop functionality

### Production Recommendations

- [ ] Add NFT ownership verification before listing
- [ ] Implement royalty payments to creators
- [ ] Add escrow for NFT token transfers
- [ ] Rate limiting for listing creation
- [ ] Whitelist/blacklist functionality
- [ ] Multi-sig authority for critical operations

---

## 📊 Type Synchronization Benefits

Using LUMOS ensures your types stay synchronized:

| Without LUMOS | With LUMOS |
|--------------|------------|
| Manual Rust struct definition | ✅ Auto-generated from schema |
| Manual TypeScript interface | ✅ Auto-generated from schema |
| Manual Borsh schema | ✅ Auto-generated from schema |
| Risk of type mismatch | ✅ Guaranteed synchronization |
| Update 3 places for changes | ✅ Update 1 place (`.lumos`) |
| Prone to human error | ✅ Compiler-verified |

**Real Impact:**
- 🚀 **3x faster development** - Write types once
- 🐛 **Zero serialization bugs** - Rust/TS always match
- 📝 **Single source of truth** - `.lumos` schema is documentation
- ♻️ **Easy refactoring** - Change schema, regenerate

---

## 🎯 Key Learnings

### 1. Using Generated Account Types

```rust
// LUMOS generates the #[account] macro automatically
#[account]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    // ...
}

// Use in Anchor contexts
#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(init, payer = authority, space = 8 + std::mem::size_of::<MarketplaceConfig>())]
    pub config: Account<'info, MarketplaceConfig>, // ← Generated type!
    // ...
}
```

### 2. Working with Generated Enums

```rust
// LUMOS generates Rust enums
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
}

// Use with pattern matching
require!(
    matches!(listing.status, ListingStatus::Active),
    MarketplaceError::ListingNotActive
);

// TypeScript gets discriminated unions
type ListingStatus =
  | { kind: 'Active' }
  | { kind: 'Sold' }
  | { kind: 'Cancelled' };

// Type-safe checks
if (listing.status.kind === 'Sold') {
  // TypeScript knows this branch
}
```

### 3. Option Types Map Cleanly

```rust
// LUMOS schema
buyer: Option<PublicKey>,

// Rust output
pub buyer: Option<Pubkey>,

// TypeScript output
buyer: PublicKey | undefined;

// Borsh output
borsh.option(borsh.publicKey(), 'buyer')
```

---

## 🛠️ Extending This Example

### Add Royalty Payments

1. **Update schema:**
   ```rust
   #[solana]
   #[account]
   struct NFTMetadata {
       // ... existing fields
       royalty_percentage: u16,
       creator: PublicKey,
   }
   ```

2. **Regenerate types:**
   ```bash
   lumos generate schema/marketplace.lumos --output programs
   ```

3. **Update buy instruction:**
   ```rust
   // Calculate and transfer royalty
   let royalty = (price * metadata.royalty_percentage / 10000) as u64;
   // Transfer to creator...
   ```

### Add Offer System

1. **Add to schema:**
   ```rust
   #[solana]
   #[account]
   struct Offer {
       listing: PublicKey,
       bidder: PublicKey,
       amount: u64,
       expires_at: i64,
   }
   ```

2. **Regenerate and implement:**
   - `make_offer` instruction
   - `accept_offer` instruction
   - `cancel_offer` instruction

---

## 📖 Related Documentation

- [LUMOS Quick Start](https://docs.lumos-lang.org/getting-started/quick-start)
- [Type System Reference](https://docs.lumos-lang.org/api/types)
- [Attributes Guide](https://docs.lumos-lang.org/api/attributes)
- [Migration Guide](https://docs.lumos-lang.org/guides/migration-guide)
- [Interactive Playground](https://docs.lumos-lang.org/playground)

---

## 🤝 Contributing

Found an issue or want to improve this example?

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## 📄 License

This example is part of the LUMOS project and is licensed under MIT OR Apache-2.0.

---

## 💬 Questions?

- **GitHub Issues:** https://github.com/getlumos/lumos/issues
- **Documentation:** https://docs.lumos-lang.org
- **Twitter:** [@getlumos](https://twitter.com/getlumos)

---

**Built with LUMOS** - Type-safe schemas for Solana development 🔮
