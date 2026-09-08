# mars

A clone-and-go Rust template: strict lints, supply-chain checks, and CI wired up from
commit one, so no project has to re-derive them.

## Setup

```sh
make setup   # installs cargo-deny, nextest, llvm-cov, taplo, typos; points git at .githooks
make check   # the full gate
```

## Commands

| Command          | What it does                                                  |
| ---------------- | ------------------------------------------------------------- |
| `make fmt`       | `cargo fmt --all` plus `taplo fmt` for TOML                    |
| `make fmt-check` | Formatting check, no writes                                    |
| `make lint`      | Clippy over all targets and features, warnings are errors      |
| `make test`      | `cargo nextest run --workspace --all-features`                 |
| `make doc-test`  | `cargo test --doc` — nextest skips doctests entirely           |
| `make cov`       | Coverage to `lcov.info` via `cargo llvm-cov`                   |
| `make deny`      | Advisories, licenses, bans, sources                            |
| `make typos`     | Spellcheck                                                     |
| `make check`     | All of the above, in the order CI runs them                    |

Cargo aliases mirror the short ones: `cargo lint`, `cargo xtest`, `cargo cov`.

## What's included

- **Workspace** with one member crate (`crates/mars`), `resolver = "3"`, edition 2024.
- **Lints**: `clippy::all` + `pedantic` + curated restriction lints (`unwrap_used`,
  `panic`, `indexing_slicing`, …) at `warn`; `unsafe_code = "forbid"`. Set at the
  workspace level and inherited via `[lints] workspace = true` in the crate.
- **Toolchain** pinned to 1.91.1 in `rust-toolchain.toml`.
- **Testing**: nextest, doctests, `proptest` for property tests, an integration test.
- **Supply chain**: `deny.toml` with a license allowlist, plus a weekly advisories run.
- **CI**: fmt, clippy, test matrix (Ubuntu + macOS), coverage, typos, deny.
- **Hook**: `.githooks/pre-commit` runs fmt + clippy only, to stay fast.

## Using this as a base

1. Rename the crate: `crates/mars/` → your name, and update `name` in its `Cargo.toml`
   plus `members` in the root `Cargo.toml`.
2. Update `[workspace.package]` metadata — `repository`, `authors`, `license`.
3. Replace `crates/<name>/src/lib.rs` with real code; keep `error.rs` as the pattern.
4. Add project jargon to `_typos.toml` when the spellchecker gets in the way.

### Benchmarks

`benches/` is intentionally empty. Criterion pulls a large dependency tree, so add it
only when you actually benchmark:

```sh
cargo add --dev criterion --features html_reports
```

Then declare `[[bench]] name = "…"  harness = false` in the crate manifest.

### Fuzzing

Not included. If a project grows a byte-parsing surface, run `cargo fuzz init`.

## License

MIT OR Apache-2.0.
