use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use error_stack::Result;

pub mod validate;
pub mod error;

#[derive(Parser, Debug)]
#[command(name = "adof")]
#[command(version = "v0.10.0")]
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

    /// Automatically update files at a set interval
    AutoUpdate {
        /// The frequency of automatic updates in minutes
        #[arg(default_value = "60")]
        min: u64,
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

    /// Get an overview of the current status of ADOF
    Summary,

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

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    name: String,
    args: Vec<String>,
}

fn main() -> Result<(), error::AdofError> {
    let cli = Cli::parse();

    let (command_name, args) = match &cli.command {
        Commands::Init => ("Init", vec![]),

        Commands::Add => ("Add", vec![]),

        Commands::Remove => ("Remove", vec![]),

        Commands::List => ("List", vec![]),

        Commands::Link { link } => {
            validate::github_repo(&link)?;
            ("Link", vec![link.clone()])
        }

        Commands::Push => ("Push", vec![]),

        Commands::Update { check } => ("Update", vec![check.to_string()]),

        Commands::AutoUpdate { min } => {
            validate::auto_update_time(*min)?;
            ("AutoUpdate", vec![min.to_string()])
        }

        Commands::Log { num, remote } => {
            validate::log_counts(*num)?;
            ("Log", vec![num.to_string(), remote.to_string()])
        }

        Commands::Summary => ("Summary", vec![]),

        Commands::Deploy { link, commit } => {
            validate::github_repo(&link)?;
            ("Deploy", vec![link.clone(), commit.clone()])
        }

        Commands::Unlink => ("Unlink", vec![]),

        Commands::Uninstall => ("Uninstall", vec![]),

        Commands::Sponsor => ("Sponsor", vec![]),
    };

    let command = Command {
        name: command_name.to_string(),
        args,
    };

    let json_data = serde_json::to_string(&command).expect("Failed to serialize");

    commands::process_command(&json_data);

    Ok(())
}
