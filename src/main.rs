use std::{path::{Path, PathBuf}, collections::HashMap};
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use dashmap::DashMap;

mod api;

pub enum Direction {
    Left,
    Right,
    Down,
    Up
}

fn setup() -> Result<(), Report> {
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Completed setup");

    let games: DashMap<String, Game> = DashMap::new();

    Ok(())
}

