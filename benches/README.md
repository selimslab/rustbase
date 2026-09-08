# Benchmarks

Intentionally empty. Criterion pulls a large dependency tree, so it is left out of the
template and added only when a project actually benchmarks something:

```sh
cargo add --dev criterion --features html_reports
```

Then declare the target in the crate manifest:

```toml
[[bench]]
name = "my_bench"
harness = false
```

Benches live next to the crate they measure (`crates/<name>/benches/`); this top-level
directory is a placeholder from the template layout.
