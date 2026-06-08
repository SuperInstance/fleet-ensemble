# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-08

### Added
- `Agent` with named demand vectors and scaling/offset operations
- `ConservationBudget` for specifying target sum constraints (including zero-sum)
- `Governor` that projects demand vectors onto the conservation surface via minimum-norm correction
- `Ensemble` for managing multi-agent collections
- `TernaryVector` for discrete {-1, 0, +1} intent with density, balance, and MIDI note mapping
- Budget violation checking and diagnostic projection
- Comprehensive test suite covering zero-sum, custom budgets, and ternary operations
