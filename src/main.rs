use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod commands;
pub mod database;
pub mod git;
pub mod validate;

use commands::*;

#[derive(Parser, Debug)]
#[command(name = "adof")]
#[command(version = "v0.11.0")]
#[command(author = "Abinash S. <fnabinash@gmail.com>")]
#[command(about = "ADOF - An Automatic Dot-files Organizer Friend", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize Adof in your system
    Init,

    /// Manually add files to be tracked by ADOF
    Add,

    /// Remove files from the tracking list
    Remove,

    /// List all the files being tracked by ADOF
    List,

    /// Link a GitHub repository to store your files
    Link {
        /// Link of the GitHub repository
        link: String,
    },

    /// Push local changes to the GitHub
    Push,

    /// Manually check and update files
    Update {
        /// Flag to check for available updates
        #[arg(short, long, default_value = "false")]
        check: bool,
    },

    /// Display logs of the latest changes
    Log {
        /// Show the latest changes up to the specified number
        #[arg(default_value = "0")]
        num: u8,

        /// Flag to fetch commits from GitHub
        #[arg(short, long, default_value = "false")]
        remote: bool,
    },

    // Get an overview of the current status of ADOF
    //Summary,

    /// Deploy files from a GitHub or local repository
    Deploy {
        /// The GitHub repository URL to deploy from
        #[arg(default_value = "")]
        link: String,

        /// Optionally, specify a commit hash
        #[arg(short, long, default_value = "")]
        commit: String,
    },

    /// Unlink the current GitHub repository from ADOF
    Unlink,

    /// Uninstall ADOF from your system
    Uninstall,

    /// Support the development of ADOF
    Sponsor,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init::init().await?;
        }

        Commands::Add => {
            add::add().await?;
        }

        Commands::Remove => {
            remove::remove()?;
        }

        Commands::List => {
            list::list()?;
        }

        Commands::Link { link } => {
            validate::github_repo(link).await?;
            link::link(link)?;
        }

        Commands::Push => {
            push::push()?;
        }

        Commands::Update { check } => {
            update::update(*check)?;
        }

        Commands::Log { num, remote } => {
            validate::log_counts(*num)?;
            log::log(*num, *remote)?;
        }

        // Commands::Summary => {
           // summary::summary();
        // }

        Commands::Deploy { link, commit } => {
            if !link.is_empty() {
                validate::github_repo(link).await?;
            }

            deploy::deploy(link, commit)?;
        }

        Commands::Unlink => {
            unlink::unlink()?;
        }

        Commands::Uninstall => {
            uninstall::uninstall()?;
        }

        Commands::Sponsor => {
            webbrowser::open("https://github.com/sponsors/fnabinash").unwrap();
        }
    };

    Ok(())
}
