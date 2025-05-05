# Product Context

This file provides a high-level overview of the project and the expected product that will be created. Initially it is based upon projectBrief.md (if provided) and all other available project-related information in the working directory. This file is intended to be updated as the project evolves, and should be used to inform all other modes of the project's goals and context.
2025-05-05 13:00:38 - Log of updates made will be appended as footnotes to the end of this file.

-

## Project Goal

Develop a modular system for blockchain-related workflows, combining Rust-based backend logic with Solidity smart contracts. The project aims to provide a robust, extensible foundation for decentralized applications and protocol integrations.

## Key Features

- Rust monorepo structure with separate binary and library crates
- Solidity smart contracts for on-chain logic
- Automated build and dependency management (Cargo, Foundry, Flake)
- Extensible architecture for protocol and tool integration
- Comprehensive documentation and memory bank for project context

## Overall Architecture

- contracts/: Contains Solidity smart contracts (e.g., HelloBlueprint.sol)
- firecrawl-blueprint-bin/: Rust binary crate (main.rs) for CLI or service entrypoint
- firecrawl-blueprint-lib/: Rust library crate with core logic and tests
- memory-bank/: Project documentation and context management
- Configuration and manifest files for Rust, Foundry, and Nix
- Modular, testable, and extensible codebase organization
