.PHONY: help setup fmt fmt-check lint test doc-test cov deny typos check clean

help:
	@grep -E '^[a-z-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-10s %s\n", $$1, $$2}'

setup: ## Install tooling and activate the git hooks
# --locked is mandatory: cargo-nextest refuses to build without it.
# typos-cli is pinned because newer releases need a rustc above our pinned toolchain.
	cargo install --locked cargo-deny cargo-nextest cargo-llvm-cov taplo-cli
	cargo install --locked typos-cli --version 1.49.0
	git config core.hooksPath .githooks

fmt: ## Format Rust and TOML
	cargo fmt --all
	taplo fmt

fmt-check: ## Check formatting without writing
	cargo fmt --all --check
	taplo fmt --check

lint: ## Clippy over every target and feature, warnings are errors
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test: ## Run tests with nextest (doctests excluded, see doc-test)
	cargo nextest run --workspace --all-features

doc-test: ## Run doctests -- nextest skips these entirely
	cargo test --doc --workspace

cov: ## Coverage report as lcov.info
	cargo llvm-cov nextest --workspace --lcov --output-path lcov.info

deny: ## Advisories, licenses, bans, sources
	cargo deny check

typos: ## Spellcheck
	typos

check: fmt-check lint test doc-test deny typos ## The full gate

clean:
	cargo clean
