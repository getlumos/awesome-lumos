# Awesome LUMOS ✨

> Curated collection of awesome LUMOS examples, full-stack applications, tutorials, and community projects for Solana development.

[![Awesome](https://awesome.re/badge.svg)](https://awesome.re)
[![LUMOS](https://img.shields.io/badge/LUMOS-Type--Safe%20Solana-9945FF.svg)](https://github.com/getlumos/lumos)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[LUMOS](https://github.com/getlumos/lumos) is a type-safe schema language and code generator that bridges TypeScript and Rust for Solana development. Write your data structures once, generate production-ready code for both languages with guaranteed Borsh serialization compatibility.

---

## 📚 Contents

- [Official Resources](#-official-resources)
- [Getting Started](#-getting-started)
- [Full-Stack Examples](#-full-stack-examples)
- [Tutorials](#-tutorials)
- [Templates & Starters](#-templates--starters)
- [Community Projects](#-community-projects)
- [Tools & Utilities](#-tools--utilities)
- [Contributing](#-contributing)

---

## 🎯 Official Resources

- [LUMOS Repository](https://github.com/getlumos/lumos) - Core library and CLI
- [VSCode Extension](https://github.com/getlumos/vscode-lumos) - Syntax highlighting and snippets
- [Documentation](https://github.com/getlumos/lumos#readme) - Official documentation
- [Examples](https://github.com/getlumos/lumos/tree/main/examples) - Schema examples in main repo

---

## 🚀 Getting Started

### Installation

```bash
# Install LUMOS CLI
cargo install lumos-cli

# Verify installation
lumos --version

# Install VSCode extension (optional)
# Search for "LUMOS" in VSCode Extensions
```

### Quick Example

```lumos
#[solana]
#[account]
struct UserAccount {
    wallet: PublicKey,
    balance: u64,
    level: u16,
}
```

Generate code:
```bash
lumos generate schema.lumos
```

---

## 🏗️ Full-Stack Examples

> **Coming Soon!** Full-stack Solana applications built with LUMOS.

### Planned Examples

- **NFT Marketplace** - Complete NFT marketplace with listings, bidding, and sales
- **DeFi Staking Platform** - Token staking with rewards calculation
- **DAO Governance** - Proposal creation, voting, and execution
- **Gaming Platform** - On-chain game state and player progression
- **Token Vesting** - Time-locked token release schedules

**Want to contribute?** See [Contributing](#-contributing) section below!

---

## 📖 Tutorials

> **Coming Soon!** Step-by-step tutorials for building Solana applications with LUMOS.

### Planned Tutorials

- [ ] Your First LUMOS Project
- [ ] Building an NFT Minting Program
- [ ] Creating a Token Staking Pool
- [ ] Implementing DAO Governance
- [ ] Advanced Type Patterns with Enums
- [ ] Testing LUMOS-Generated Code

---

## 🎨 Templates & Starters

### Anchor Program Template

```bash
# Coming soon - scaffold a new Anchor program with LUMOS
lumos new --template anchor my-program
```

### Full-Stack DApp Template

```bash
# Coming soon - scaffold a complete DApp (Anchor + React + LUMOS)
lumos new --template dapp my-dapp
```

---

## 🌟 Community Projects

> **Showcase your LUMOS project here!**

Built something awesome with LUMOS? Submit a PR to add it here:

- **Your Project** - Brief description and link

---

## 🛠️ Tools & Utilities

### Official Tools

- [LUMOS CLI](https://github.com/getlumos/lumos) - Code generation tool
- [VSCode Extension](https://github.com/getlumos/vscode-lumos) - Syntax highlighting and snippets

### Community Tools

> **Coming Soon!** Community-contributed tools and utilities.

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Adding Your Project

1. Fork this repository
2. Add your project to the appropriate section
3. Follow the format: `**Project Name** - Brief description ([Demo](link) | [Source](link))`
4. Submit a pull request

### Creating Examples

We're looking for:
- ✨ Full-stack Solana applications using LUMOS
- 📚 Tutorial content and guides
- 🎨 Starter templates and boilerplates
- 🔧 Tools and utilities that enhance LUMOS development

**Requirements for examples:**
- Complete, working code
- README with setup instructions
- Well-documented and commented
- Follows LUMOS best practices

### Example Contribution Template

```
examples/
└── your-project/
    ├── README.md           (Setup, features, architecture)
    ├── schema.lumos        (LUMOS schema definitions)
    ├── programs/           (Anchor/Solana programs)
    ├── app/                (Frontend application)
    └── tests/              (Integration tests)
```

---

## 📋 Contribution Guidelines

1. **Quality Over Quantity** - Each example should be production-quality
2. **Documentation** - Include comprehensive README and inline comments
3. **Testing** - All examples must include tests
4. **License** - Use MIT or Apache 2.0 license
5. **Maintenance** - Be responsive to issues and keep examples updated

---

## 📄 License

This repository is dual-licensed under:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion shall be dual licensed as above, without any additional terms or conditions.

---

## 🙏 Acknowledgments

- **LUMOS Team** - For creating an amazing tool
- **Solana Foundation** - For the incredible blockchain ecosystem
- **Anchor Team** - For the excellent Solana development framework
- **Contributors** - Everyone who contributes examples and improvements

---

<div align="center">

**Built with ❤️ by the LUMOS community**

⭐ Star this repo if you find these examples useful!

[Submit Example](https://github.com/getlumos/awesome-lumos/issues/new) • [Report Issue](https://github.com/getlumos/awesome-lumos/issues) • [Request Tutorial](https://github.com/getlumos/awesome-lumos/issues)

</div>
