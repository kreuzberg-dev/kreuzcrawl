mod fixtures;
mod rust;

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use fixtures::load_fixtures;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Generate E2E test suites from crawl fixtures"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate E2E test projects from fixture files
    Generate {
        /// Path to fixtures directory
        #[arg(long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,
        /// Output root directory for generated tests
        #[arg(long, default_value = "e2e")]
        output: Utf8PathBuf,
    },
    /// List all loaded fixtures
    List {
        /// Path to fixtures directory
        #[arg(long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { fixtures, output } => {
            let fixtures = load_fixtures(fixtures.as_path())?;
            rust::generate(&fixtures, output.as_path())?;
            run_cargo_fmt(&output.join("rust"));
        }
        Commands::List { fixtures } => {
            let fixtures = load_fixtures(fixtures.as_path())?;
            for fixture in &fixtures {
                println!(
                    "{:<40} {:<12} {}",
                    fixture.id,
                    fixture.category(),
                    fixture.description
                );
            }
        }
    }
    Ok(())
}

fn run_cargo_fmt(dir: &camino::Utf8Path) {
    let status = std::process::Command::new("cargo")
        .args(["fmt", "--manifest-path"])
        .arg(dir.join("Cargo.toml").as_str())
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(_) => eprintln!("Warning: cargo fmt returned non-zero for {dir}"),
        Err(e) => eprintln!("Warning: failed to run cargo fmt: {e}"),
    }
}
