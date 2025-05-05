# System Patterns _Optional_

This file documents recurring patterns and standards used in the project.
It is optional, but recommended to be updated as the project evolves.
2025-05-05 13:01:10 - Log of updates made.

-

## Coding Patterns

- Rust for backend logic (modular crates: binary and library)
- Solidity for smart contracts (modular contract structure)
- Use of configuration files for reproducible builds (Cargo.toml, flake.nix, foundry.toml)

## Architectural Patterns

- Monorepo structure for unified management of contracts and backend
- Separation of concerns: contracts/, firecrawl-blueprint-bin/, firecrawl-blueprint-lib/
- Memory bank for persistent documentation and architectural context

## Testing Patterns

- Rust integration and end-to-end tests (firecrawl-blueprint-lib/tests/e2e.rs)
- Smart contract testing via Foundry (implied by foundry.toml and contracts/)
