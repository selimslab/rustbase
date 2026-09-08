//! Thin binary wrapper around the `mars` library.

// A CLI legitimately writes to stdout.
#![allow(clippy::print_stdout)]

use anyhow::Context as _;
use mars::Greeting;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "world".to_owned());
    tracing::debug!(%name, "building greeting");

    let greeting = Greeting::new(&name).with_context(|| format!("invalid name: {name:?}"))?;
    println!("{}", greeting.render());

    Ok(())
}
