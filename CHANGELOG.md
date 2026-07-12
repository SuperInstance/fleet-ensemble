# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- In-source test suite did not compile: tests treated `Governor::project`'s
  `Result` return as a bare `Vec`, so `cargo test` failed with 4 errors and the
  suite never ran. All call sites now unwrap the `Result`.
- `ternary_vector_operations` test asserted the wrong density (0.625 instead of
  0.75 for `[1,0,-1,1,0,-1,1,1]`); the suite had never run to catch it.
- `Ensemble::total_demand` could panic (index out of bounds) when agents had
  differing dimensions; it now sizes to the widest agent and never panics.
- README Quick Start used `zero_sum(2)` with 3-dimensional agents, which would
  panic with `DimensionMismatch`; corrected to `zero_sum(3)`.
- `cargo fmt -- --check` and `cargo clippy` failures across `src/` and examples.

### Added
- `TernaryVector::try_new` — fallible constructor returning `Error::InvalidTernary`
  (the error variant previously existed but was never constructed).
- Tests for the `EmptyEnsemble` and `DimensionMismatch` error paths,
  `project_with_diagnostics`, `Agent` helpers, budget `violation`/`is_satisfied`,
  `TernaryVector::to_demand`, and empty-vector edge cases.
- `.gitignore` for `target/`.

### Changed
- README now documents the real scope (single-process, synchronous, in-memory)
  and lists what is implemented and tested.

## [0.1.0] - 2026-06-08

### Added
- `Agent` with named demand vectors and scaling/offset operations
- `ConservationBudget` for specifying target sum constraints (including zero-sum)
- `Governor` that projects demand vectors onto the conservation surface via minimum-norm correction
- `Ensemble` for managing multi-agent collections
- `TernaryVector` for discrete {-1, 0, +1} intent with density, balance, and MIDI note mapping
- Budget violation checking and diagnostic projection
- Comprehensive test suite covering zero-sum, custom budgets, and ternary operations
