# Architecture

A clone-and-go Rust template. Every project starts from the same lint gate,
supply-chain checks, and CI instead of re-deriving them.

## Layout

```
Cargo.toml            virtual manifest — workspace config, shared deps, all lints
crates/mars/          the one member crate; lib + thin bin
  src/lib.rs          public API, unit test, proptest
  src/error.rs        thiserror enum + Result alias
  src/main.rs         tracing init, calls the lib, returns anyhow::Result
  tests/              integration test — proves the lib/bin split
.github/workflows/    ci.yml (6 jobs), audit.yml (weekly advisories)
.githooks/pre-commit  fmt + clippy, nothing slower
Makefile              the verbs; CI and humans run the same ones
```

## Decisions

**Workspace with one crate.** A second crate is a line in `members`, not a
restructure. Costs nothing while unused.

**Lints at the workspace root.** One place to edit, inherited via
`[lints] workspace = true` in the crate — which is required, since workspace
lints are *not* picked up implicitly.

**Strict-pragmatic.** `clippy::all` + `pedantic` + restriction lints
(`unwrap_used`, `panic`, `indexing_slicing`, …) at **warn**, `unsafe_code`
forbidden. Warn locally so day-one work never blocks; CI sets `-D warnings`,
so nothing merges dirty. Tests scope-allow `unwrap`/`expect` — that's how a
test asserts.

**Lint groups need `priority = -1`.** `clippy::all`, `pedantic`, and
`rust_2018_idioms` are groups. Without lower priority, Cargo rejects the
manifest when an individual lint from the group is also listed.

**Makefile as the contract.** `make check` runs the exact gate CI runs, in the
same order. No drift between "works locally" and "passes CI".

**`doc-test` is its own target.** `cargo nextest` skips doctests entirely. A
nextest-only pipeline silently stops testing every `///` example.

**Pinned toolchain** (1.91.1, edition 2024, resolver 3). Reproducible builds;
no MSRV job, since the pin *is* the floor.

**Fast hook.** Pre-commit runs fmt + clippy only. Tests belong in CI — a slow
hook gets bypassed, and a bypassed hook checks nothing.

## Deliberately excluded

| Thing | Why | Add it with |
| --- | --- | --- |
| Criterion | Large dep tree for a template that may never benchmark | `cargo add --dev criterion` |
| cargo-fuzz | Only earns its place with a byte-parsing surface | `cargo fuzz init` |
| MSRV job | The toolchain pin already fixes the floor | a matrix entry in `ci.yml` |
| `insta` | Declared in `[workspace.dependencies]`, unused | `insta.workspace = true` |

## Known friction

- **`cargo install` needs `--locked`.** cargo-nextest refuses to build without
  it. `make setup` passes it.
- **typos-cli is version-pinned** (1.49.0). Newer releases require a rustc
  above our pinned toolchain. Bump both together or not at all.
- **cargo-deny warns benignly.** Unused license allowances (kept for future
  deps) and a duplicate `syn` from upstream crates. `multiple-versions` is
  `warn`, not `deny`, precisely because that duplicate isn't ours to fix.
- **cargo-deny's schema shifts between majors.** Re-run `cargo deny check`
  after upgrading and follow what it reports.

## Using this as a base

1. Rename `crates/mars/` and update `name`, plus `members` in the root manifest.
2. Update `[workspace.package]` — `repository`, `authors`, `license`.
3. Replace `lib.rs`; keep `error.rs` as the pattern.
4. `make setup`, then `make check`.
