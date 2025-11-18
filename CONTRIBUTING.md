# Contributing to Awesome LUMOS

Thank you for your interest in contributing to Awesome LUMOS! This guide will help you add your projects, examples, and tutorials to the collection.

## Table of Contents

- [Types of Contributions](#types-of-contributions)
- [Submission Guidelines](#submission-guidelines)
- [Example Project Structure](#example-project-structure)
- [Quality Standards](#quality-standards)
- [Pull Request Process](#pull-request-process)

---

## Types of Contributions

### 1. Full-Stack Examples

Complete, production-ready Solana applications demonstrating real-world use cases:

- NFT marketplaces
- DeFi protocols (staking, lending, DEX)
- DAO governance systems
- Gaming applications
- Social platforms

**Requirements:**
- Working Anchor/Solana program(s)
- Frontend application (React, Next.js, etc.)
- LUMOS schema definitions
- Integration tests
- Deployment instructions

---

### 2. Tutorials

Step-by-step guides for building Solana applications with LUMOS:

- Getting started guides
- Feature-specific tutorials
- Best practices and patterns
- Migration guides from plain Anchor

**Requirements:**
- Clear, well-structured content
- Code snippets with explanations
- Working example code
- Expected outcomes documented

---

### 3. Templates & Starters

Boilerplate projects for quick project initialization:

- Anchor program templates
- Full-stack DApp starters
- Testing frameworks
- CI/CD configurations

**Requirements:**
- Easy to initialize (`npx`, `cargo generate`, etc.)
- Well-documented configuration
- Minimal dependencies
- Production-ready setup

---

### 4. Tools & Utilities

Developer tools that enhance LUMOS workflows:

- Code generators
- Testing utilities
- Deployment scripts
- IDE plugins

**Requirements:**
- Clear purpose and use case
- Installation instructions
- Usage examples
- Maintained and supported

---

## Submission Guidelines

### Adding to README.md

1. **Fork this repository**
2. **Add your contribution** to the appropriate section in README.md
3. **Follow the format:**
   ```markdown
   **Project Name** - Brief description (1-2 sentences). ([Demo](link) | [Source](link))
   ```
4. **Keep alphabetical order** within sections
5. **Submit a pull request**

### Adding Full Example

1. **Create a directory** under `examples/your-project-name/`
2. **Follow the structure** defined below
3. **Include all required files**
4. **Test everything** before submitting
5. **Submit a pull request**

---

## Example Project Structure

```
examples/
└── your-project/
    ├── README.md              # Project documentation
    ├── schema.lumos           # LUMOS type definitions
    ├── programs/              # Anchor/Solana programs
    │   └── your-program/
    │       ├── Cargo.toml
    │       ├── Anchor.toml
    │       └── src/
    │           └── lib.rs     # Import generated LUMOS types
    ├── app/                   # Frontend application
    │   ├── package.json
    │   ├── src/
    │   │   └── types/         # LUMOS generated TypeScript
    │   └── ...
    ├── tests/                 # Integration tests
    │   └── integration.ts
    ├── .gitignore
    └── LICENSE                # MIT or Apache 2.0
```

### Example README.md Template

```markdown
# Your Project Name

Brief description of what this project demonstrates.

## Features

- Feature 1
- Feature 2
- Feature 3

## Architecture

Brief explanation of how LUMOS is used in this project.

## Prerequisites

- Rust 1.70+
- Solana CLI 1.18+
- Anchor 0.30+
- Node.js 18+
- LUMOS CLI

## Setup

\`\`\`bash
# Install dependencies
npm install
cargo build

# Generate LUMOS types
lumos generate schema.lumos

# Run tests
anchor test

# Deploy
anchor deploy
\`\`\`

## LUMOS Schema

Explanation of the schema design and key types.

## Testing

How to run and understand the tests.

## Deployment

Step-by-step deployment instructions.

## License

MIT or Apache 2.0
```

---

## Quality Standards

### Code Quality

- ✅ Clean, readable, well-commented code
- ✅ Follows Rust and TypeScript best practices
- ✅ No unused dependencies
- ✅ Proper error handling
- ✅ Security considerations addressed

### Documentation

- ✅ Comprehensive README
- ✅ Inline code comments for complex logic
- ✅ API documentation where applicable
- ✅ Deployment and testing instructions
- ✅ Architecture diagrams (if complex)

### Testing

- ✅ Unit tests for key functions
- ✅ Integration tests for program instructions
- ✅ Frontend tests (if applicable)
- ✅ All tests passing
- ✅ Test coverage > 70% (recommended)

### LUMOS Usage

- ✅ Schema-first design
- ✅ Proper use of LUMOS types
- ✅ Generated code committed to repo
- ✅ Schema documented and explained
- ✅ Demonstrates LUMOS best practices

---

## Pull Request Process

1. **Fork the repository** and create a new branch
   ```bash
   git checkout -b add-my-project
   ```

2. **Make your changes** following the guidelines above

3. **Test everything** locally
   ```bash
   # For examples
   cd examples/your-project
   lumos generate schema.lumos
   anchor test
   npm test
   ```

4. **Commit with descriptive message**
   ```bash
   git commit -m "Add NFT marketplace example"
   ```

5. **Push to your fork**
   ```bash
   git push origin add-my-project
   ```

6. **Create pull request** with:
   - Clear title describing the contribution
   - Description of what you're adding
   - Screenshots/demos if applicable
   - Checklist confirming quality standards

7. **Respond to feedback** from reviewers

---

## Review Criteria

Pull requests will be reviewed for:

- [ ] Follows project structure
- [ ] Code quality and style
- [ ] Documentation completeness
- [ ] Tests passing
- [ ] No security issues
- [ ] License compatibility
- [ ] Adds value to the collection

---

## Getting Help

- **Questions?** Open an [issue](https://github.com/getlumos/awesome-lumos/issues)
- **Discussion?** Join the conversation in discussions
- **Need help?** Check the [LUMOS documentation](https://github.com/getlumos/lumos)

---

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to learn and build together.

---

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache 2.0.

---

Thank you for contributing to Awesome LUMOS! 🎉
