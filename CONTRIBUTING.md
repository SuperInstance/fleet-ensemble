# Contributing to fleet-ensemble

Thank you for your interest in contributing!

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running Examples

```bash
cargo run --example basic
```

## Code Quality

Before submitting a PR:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes with clear commit messages
4. Ensure CI passes (fmt, clippy, test)
5. Open a pull request against `main`

## Architecture

The crate is structured around conservation-constrained coordination:

- **`agent`** — Agent with a demand vector
- **`budget`** — Conservation budget (target sum)
- **`governor`** — Projects demands onto the constraint surface
- **`ensemble`** — Collection of agents
- **`ternary`** — Discrete ternary vectors for symbolic coordination

## Mathematical Background

The projection minimizes the total correction:

```
min Σ ||d_i' - d_i||²   subject to   Σ d_i' = target
```

The solution distributes the deficit equally: `d_i' = d_i - (Σ d_i - target) / N`.
