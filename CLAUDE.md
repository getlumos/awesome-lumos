# CLAUDE.md - Awesome LUMOS

**Repository:** https://github.com/getlumos/awesome-lumos
**Website:** https://lumos-lang.org
**Purpose:** Production-ready LUMOS examples and community projects

---

## Current Examples (5 Complete)

| Example | Types | Instructions | Features | Path |
|---------|-------|--------------|----------|------|
| **NFT Marketplace** | 7 | 9 | Fixed/auction listings, bidding, royalties | `examples/nft-marketplace/` |
| **DeFi Staking** | 9 | 7 | Fixed/tiered/dynamic APY, cooldown, compound | `examples/defi-staking/` |
| **DAO Governance** | 12 | 9 | Proposals, voting, quorum, timelock | `examples/dao-governance/` |
| **Gaming Inventory** | 14 | 11 | Player progression, crafting, 6 rarities | `examples/gaming-inventory/` |
| **Token Vesting** | 11 | 6 | Linear/cliff/milestone vesting, revocation | `examples/token-vesting/` |

**Total:** 53 types, 42 instructions, 4000+ LOC of type-safe Solana code

---

## Structure

```
awesome-lumos/
├── examples/          # 5 complete full-stack examples
│   ├── nft-marketplace/
│   ├── defi-staking/
│   ├── dao-governance/
│   ├── gaming-inventory/
│   └── token-vesting/
├── tutorials/         # Step-by-step guides (open for contributions)
├── templates/         # Project starters (open for contributions)
├── README.md
└── CONTRIBUTING.md
```

Each example includes:
- `.lumos` schema (source of truth)
- Generated Rust + TypeScript
- Working Anchor program
- TypeScript client with helpers
- Comprehensive README

---

## Key Learnings

**Type Synchronization:** All examples demonstrate zero type mismatches between Rust/TypeScript via LUMOS generation.

**Common Patterns:**
- Complex enums with data (VestingType, ProposalType, ItemType)
- Calculation parity (on-chain and client match exactly)
- Context-aware generation (Anchor vs Borsh)
- Option/Vec types correctly mapped

**Development Speed:** 3-4x faster type definition, 100% elimination of type mismatch bugs.

---

## Contributing

### Full Example Requirements
- Complete `.lumos` schema
- Working Anchor program (compiles)
- TypeScript client
- Tests passing
- Comprehensive README
- MIT or Apache 2.0 license

### Review Criteria
Code quality, documentation completeness, tests passing, security, license compatibility, adds value.

---

## AI Assistant Guidelines

**DO:** Survey existing examples, test before submitting, maintain consistent formatting, update README.md.

**DON'T:** Add incomplete examples, submit untested code, duplicate existing examples, skip documentation.

---

## Related Repositories

- **lumos** - Core library and CLI
- **vscode-lumos** - VSCode extension
- **docs-lumos** - Official docs (lumos-lang.org)

---

**Last Updated:** 2025-11-20
**Status:** 5 production-ready examples available
