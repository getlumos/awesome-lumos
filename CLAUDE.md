# CLAUDE.md - Awesome LUMOS

**Repository:** https://github.com/getlumos/awesome-lumos
**Website:** https://lumos-lang.org
**Purpose:** Curated collection of LUMOS examples, tutorials, and community projects

---

## What This Repository Contains

Community-driven showcase of:
- **Full-stack Solana applications** built with LUMOS
- **Step-by-step tutorials** and learning guides
- **Project templates** and starters
- **Tools and utilities** for LUMOS development

**Status:** Initialized, awaiting community contributions

---

## Directory Structure

```
awesome-lumos/
├── examples/          # Full-stack application examples
│   └── .gitkeep
├── tutorials/         # Step-by-step guides
│   └── .gitkeep
├── templates/         # Project starters & boilerplates
│   └── .gitkeep
├── README.md          # Main awesome list
└── CONTRIBUTING.md    # Contribution guidelines
```

---

## Contribution Types

### 1. Full-Stack Examples
Complete, production-ready Solana applications:
- Anchor/Solana programs
- Frontend (React/Next.js)
- LUMOS schema definitions
- Integration tests
- Deployment instructions

**Example structure:**
```
examples/nft-marketplace/
├── README.md
├── schema.lumos
├── programs/
├── app/
└── tests/
```

### 2. Tutorials
Step-by-step learning content:
- Getting started guides
- Feature-specific tutorials
- Best practices
- Migration guides

### 3. Templates
Quick-start boilerplates:
- Anchor program templates
- Full-stack DApp starters
- Testing frameworks
- CI/CD configurations

### 4. Tools & Utilities
Developer tools:
- Code generators
- Testing utilities
- Deployment scripts
- IDE plugins

---

## Quality Standards

### Code Quality
- ✅ Clean, well-commented code
- ✅ Follows Rust/TypeScript best practices
- ✅ Proper error handling
- ✅ Security considerations

### Documentation
- ✅ Comprehensive README
- ✅ Setup and deployment instructions
- ✅ Architecture explanation
- ✅ LUMOS schema documented

### Testing
- ✅ Unit tests for key functions
- ✅ Integration tests
- ✅ All tests passing
- ✅ Coverage > 70% (recommended)

### LUMOS Usage
- ✅ Schema-first design
- ✅ Proper use of generated types
- ✅ Generated code committed
- ✅ Demonstrates best practices

---

## Adding Your Project

### Quick Process
1. Fork this repository
2. Add to appropriate section in README.md
3. Follow format: `**Name** - Description ([Demo](link) | [Source](link))`
4. Submit pull request

### Adding Full Example
1. Create directory: `examples/your-project/`
2. Include all required files (see structure above)
3. Add README with setup instructions
4. Ensure tests pass
5. Submit pull request

---

## Review Criteria

Pull requests reviewed for:
- [ ] Follows project structure
- [ ] Code quality and style
- [ ] Documentation completeness
- [ ] Tests passing
- [ ] No security issues
- [ ] License compatibility (MIT or Apache 2.0)
- [ ] Adds value to the collection

---

## AI Assistant Guidelines

### ✅ DO:
- Survey existing examples before suggesting new ones
- Ensure contributions follow quality standards
- Test all code before submitting
- Update README.md when adding projects
- Maintain consistent formatting

### ❌ DON'T:
- Add low-quality or incomplete examples
- Submit untested code
- Duplicate existing examples
- Skip documentation requirements
- Ignore contribution guidelines

---

## Planned Examples (Help Wanted!)

- NFT Marketplace - Complete marketplace with LUMOS schemas
- DeFi Staking Platform - Token staking with rewards
- DAO Governance - Proposal and voting system
- Gaming Platform - On-chain game state management
- Token Vesting - Time-locked token releases

**Want to contribute?** Pick one and build it! See CONTRIBUTING.md for guidelines.

---

## Related Repositories

- **lumos** - Core library and CLI (required for all examples)
- **vscode-lumos** - VSCode extension for better development experience

---

**Last Updated:** 2025-11-18
**Status:** Awaiting first community contribution
