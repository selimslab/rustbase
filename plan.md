# Base Rust Template

## Context

`/Users/selimozturk/Documents/base/mars` is an empty git repo with placeholder directories
(`src/`, `tests/`, `benches/`, `.cargo/`, `.githooks/`, `.github/workflows/`) and no files.

Goal: a clone-and-go Rust template with strong static analysis, so every new project starts
with the same lint gate, supply-chain checks, and CI instead of re-deriving them each time.

Decisions already settled:

- **Layout**: Cargo workspace with one member crate (scales to multi-crate without restructuring)
- **Strictness**: strict-pragmatic — `clippy::all` + `pedantic` + curated restriction lints
- **Tooling**: cargo-deny, nextest, llvm-cov, taplo, typos, **proptest** (no cargo-fuzz)
- **CI**: GitHub Actions + local pre-commit hook
- **MSRV**: pin stable 1.91.1 in `rust-toolchain.toml`, no separate MSRV job
- **Deps**: thiserror + anyhow + tracing

Local toolchain is rustc/cargo 1.91.1; none of the cargo subcommands above are installed yet.

## Files to create

### Step 0 — `plan.md`

Copy this plan to `/Users/selimozturk/Documents/base/mars/plan.md` so it lives with the repo
and travels with every clone. Do this first, before any scaffolding.

### Workspace root

**`Cargo.toml`** — virtual manifest, no root package.

- `[workspace]` `resolver = "3"`, `members = ["crates/mars"]`
- `[workspace.package]` — shared `edition = "2024"`, `rust-version = "1.91"`, license, repository
- `[workspace.dependencies]` — pin each once: `thiserror`, `anyhow`, `tracing`,
  `tracing-subscriber` (features `env-filter`, `fmt`), and dev: `proptest`, `insta`
- `[workspace.lints.rust]` —
  `unsafe_code = "forbid"`, `missing_docs = "warn"`, `rust_2018_idioms = "warn"`,
  `unreachable_pub = "warn"`, `missing_debug_implementations = "warn"`,
  `unused_qualifications = "warn"`
- `[workspace.lints.clippy]` —
  `all = { level = "warn", priority = -1 }`, `pedantic = { level = "warn", priority = -1 }`,
  then curated restriction lints at `warn` (not deny, so they don't block day-one work):
  `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`,
  `indexing_slicing`, `float_cmp`, `mem_forget`, `dbg_macro`, `print_stdout`
  (`print_stdout` allowed in the bin crate — see below)
- `[profile.release]` — `lto = "thin"`, `codegen-units = 1`, `panic = "abort"`, `strip = "symbols"`
- `[profile.dev.package."*"]` — `opt-level = 2` so dependencies build optimized in dev

Priority note: `priority = -1` on the group lints is required, otherwise Cargo rejects the
manifest when an individual lint from that group is also listed.

**`rust-toolchain.toml`** — `channel = "1.91.1"`, `components = ["rustfmt", "clippy", "rust-src"]`,
`profile = "minimal"`.

**`rustfmt.toml`** — stable options only (unstable ones are silently ignored on stable):
`edition = "2024"`, `max_width = 100`, `use_field_init_shorthand = true`,
`use_try_shorthand = true`, `newline_style = "Unix"`.
Add nightly-only niceties (`group_imports`, `imports_granularity`, `wrap_comments`) as
commented lines with a note that they need `cargo +nightly fmt`.

**`.cargo/config.toml`** —
- `[alias]`: `lint = "clippy --workspace --all-targets --all-features"`,
  `xtest = "nextest run --workspace"`, `cov = "llvm-cov nextest --workspace"`
- `[target.'cfg(all())'] rustflags` left empty by default; include a commented
  `-Dwarnings` line explaining CI sets it via env instead
- On macOS, a commented block for the `lld`/`sold` faster-linker setup

**`deny.toml`** — cargo-deny config, four sections:
- `[advisories]` — `yanked = "deny"`; RustSec DB defaults
- `[licenses]` — `allow = ["MIT", "Apache-2.0", "Apache-2.0 WITH LLVM-exception", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-3.0", "Zlib"]`, `confidence-threshold = 0.9`
- `[bans]` — `multiple-versions = "warn"`, `wildcards = "deny"`
- `[sources]` — `unknown-registry = "deny"`, allow only crates.io

Note: cargo-deny's config schema shifts between major versions. Run `cargo deny check` once
after writing and adjust to whatever the installed version reports.

**`taplo.toml`** — TOML formatting: aligned entries off, `indent_string` two spaces,
`reorder_keys = false` (reordering `Cargo.toml` keys is disruptive).

**`_typos.toml`** — empty `[default.extend-words]` table as the place to whitelist
project-specific jargon.

**`.gitignore`** — `/target`, `**/*.rs.bk`, `*.profraw`, `/coverage`, `.DS_Store`, `.idea/`, `.vscode/`.

**`Makefile`** — thin wrapper so the same verbs work locally and in CI:
- `fmt` / `fmt-check` — `cargo fmt --all` (+ `taplo fmt`)
- `lint` — `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `test` — `cargo nextest run --workspace --all-features`
- `doc-test` — `cargo test --doc --workspace` (nextest does not run doctests)
- `cov` — `cargo llvm-cov nextest --workspace --lcov --output-path lcov.info`
- `deny` — `cargo deny check`
- `typos` — `typos`
- `check` — the full gate: fmt-check + lint + test + doc-test + deny + typos
- `setup` — `cargo install cargo-deny cargo-nextest cargo-llvm-cov taplo-cli typos-cli`
  and `git config core.hooksPath .githooks`

`doc-test` as a separate target matters: `cargo nextest` skips doctests entirely, so a
nextest-only pipeline silently stops testing every `///` example.

### Member crate — `crates/mars/`

**`crates/mars/Cargo.toml`** — package inheriting `version.workspace = true` etc.;
`[lints] workspace = true` (required — workspace lints are **not** inherited implicitly);
deps via `.workspace = true`. Declares both a lib and a bin target.

**`crates/mars/src/lib.rs`** — crate-level docs (`//!`), one small example type with a
documented public API and a doctest, plus a `#[cfg(test)] mod tests` holding:
- one ordinary unit test
- one `proptest!` block demonstrating a roundtrip invariant

**`crates/mars/src/error.rs`** — `thiserror`-derived `Error` enum plus
`pub type Result<T> = core::result::Result<T, Error>`.

**`crates/mars/src/main.rs`** — thin binary: init `tracing_subscriber` with `EnvFilter`,
call into the lib, return `anyhow::Result<()>`. Carries
`#![allow(clippy::print_stdout)]` since a CLI legitimately prints.

**`crates/mars/tests/integration.rs`** — integration test importing the crate by name,
proving the lib/bin split works.

**`crates/mars/benches/`** — deferred. Criterion pulls a large dependency tree for a
template that may never benchmark; leave the top-level `benches/` dir empty with a note in
the README on adding Criterion when needed.

### CI and hooks

**`.github/workflows/ci.yml`** — jobs, all with `Swatinem/rust-cache@v2`:
1. `fmt` — `cargo fmt --all --check` + `taplo fmt --check`
2. `clippy` — `--workspace --all-targets --all-features -- -D warnings`
3. `test` — matrix over `ubuntu-latest` / `macos-latest`, nextest + doctests
4. `coverage` — `cargo llvm-cov` producing `lcov.info` as an artifact
5. `typos`
6. `deny` — `EmbarkStudios/cargo-deny-action@v2`
Concurrency group cancels superseded runs on the same ref.

**`.github/workflows/audit.yml`** — scheduled weekly `cargo deny check advisories`, so a
newly disclosed CVE surfaces without waiting for the next push.

**`.github/dependabot.yml`** — weekly updates for `cargo` and `github-actions`.

**`.githooks/pre-commit`** — executable shell script, fast path only:
`cargo fmt --all --check` then `cargo clippy --workspace --all-targets -- -D warnings`.
No tests — keep the commit loop under a couple of seconds. Activated by
`git config core.hooksPath .githooks` (run by `make setup`).

**`README.md`** — what the template includes, `make setup` as the first step, the command
table, and a short "using this as a base" section: rename the crate, update
`[workspace.package]` metadata, and `cargo fuzz init` if a project ever grows a
byte-parsing surface.

## Verification

Run in order from the repo root:

1. `make setup` — installs the five cargo tools and points git at `.githooks`
2. `cargo build --workspace` — manifest and lint syntax are valid
   (a bad `priority` value or unknown lint name fails here)
3. `make check` — the whole gate green: fmt, clippy, nextest, doctests, deny, typos
4. `cargo llvm-cov nextest --workspace --summary-only` — coverage instrumentation works
5. Confirm the lints actually bite: temporarily add `let x: Option<u8> = None; x.unwrap();`
   to `lib.rs` and check `cargo clippy` warns `clippy::unwrap_used`; revert
6. Confirm the hook fires: stage a badly formatted file, `git commit`, expect rejection
7. Push a branch and confirm every GitHub Actions job passes
