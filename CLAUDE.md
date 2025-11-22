# CLAUDE.md - Awesome LUMOS

> **Ecosystem Context:** See [getlumos/lumos/CLAUDE.md](https://github.com/getlumos/lumos/blob/main/CLAUDE.md) for LUMOS ecosystem overview, cross-repo standards, and shared guidelines.

---

## Examples

| Example | Types | Instructions | Path |
|---------|-------|--------------|------|
| NFT Marketplace | 7 | 9 | `examples/nft-marketplace/` |
| DeFi Staking | 9 | 7 | `examples/defi-staking/` |
| DAO Governance | 12 | 9 | `examples/dao-governance/` |
| Gaming Inventory | 14 | 11 | `examples/gaming-inventory/` |
| Token Vesting | 11 | 6 | `examples/token-vesting/` |

**Total:** 53 types, 42 instructions, 4000+ LOC

---

## Example Structure

```
examples/[name]/
├── schema.lumos      # Source of truth
├── generated.rs/.ts  # Generated code
├── programs/         # Anchor program
└── client/           # TypeScript client
```

---

## Contributing

**Requirements:**
- Complete `.lumos` schema
- Working Anchor program (`anchor build`)
- TypeScript client with tests
- Comprehensive README
- MIT or Apache 2.0 license

---

**Last Updated:** 2025-11-22
**Status:** 5 production-ready examples available
