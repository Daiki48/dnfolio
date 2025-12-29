mod build;
mod models;
mod ogp;
mod serve;
mod sitemap;
mod structured_data;
mod templates;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dnfolio", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build to static files
    Build,
    /// Starting local develop server
    Serve,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("Building static files ...");
            build::run().await?;
            println!("Build finished!");
        }
        Commands::Serve => {
            println!("Starting development server ...");
            build::run().await?;
            println!("Build finished!");
            serve::run().await?;
        }
    }
    Ok(())
}
