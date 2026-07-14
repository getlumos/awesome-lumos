<!-- Satellite context file — extends the global hub (~/.claude/CLAUDE.md | ~/.pi/agent/AGENTS.md). Host-neutral; project-specific only. Do not duplicate hub standards here. -->

# Awesome LUMOS

> Curated examples and templates for the LUMOS schema language.

## Repository Overview

Awesome LUMOS ships production-ready examples of LUMOS schemas with generated
Rust/TypeScript code, Anchor programs, and client libraries. Each example is a
self-contained reference for a common on-chain data pattern.

**Ecosystem context:** See [getlumos/lumos](https://github.com/getlumos/lumos)
for the LUMOS ecosystem overview, cross-repo standards, and shared guidelines.

## Examples

| Example | Types | Instructions | Path |
|---------|-------|--------------|------|
| NFT Marketplace | 7 | 9 | `examples/nft-marketplace/` |
| DeFi Staking | 9 | 7 | `examples/defi-staking/` |
| DAO Governance | 12 | 9 | `examples/dao-governance/` |
| Gaming Inventory | 14 | 11 | `examples/gaming-inventory/` |
| Token Vesting | 11 | 6 | `examples/token-vesting/` |

**Total:** 53 types, 42 instructions, 4000+ LOC

## Example Structure

```
examples/[name]/
├── schema.lumos      # Source of truth
├── generated.rs/.ts  # Generated code
├── programs/         # Anchor program
└── client/           # TypeScript client
```

## Contributing

**Requirements:**
- Complete `.lumos` schema
- Working Anchor program (`anchor build`)
- TypeScript client with tests
- Comprehensive README
- MIT or Apache 2.0 license

See `CONTRIBUTING.md` and `CI_STATUS.md` for full details.