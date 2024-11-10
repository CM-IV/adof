// deploy - copy all the dot files from GitHub and store in the local machine at perfect places
//      - process
//          - when user runs the deploy command first fetch the files and shows a summary of all
//          files like
//              - list all the files
//              - with line count means how many lines each file contains
//              - then the file language like rust, lua, vimrc etc
//              - the file location where it going to save (also give the option to user to change
//              the file location if they want to)
//          - after this if user says yes then proceed and save files where the location it
//          indicated
//          - when a user deploy from other GitHub repo do not link it only deploy, only link when
//          user runs the deploy command
//          - then do a small animation to celebrate🎉

use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod commands;
pub mod database;
pub mod git;

use commands::{
    add, auto_update, deploy, init, link, list, log, push, remove, summary, uninstall, unlink,
    update,
};

#[derive(Parser)]
#[command(name = "adof")]
#[command(version = "v0.10.0")]
#[command(author = "Abinash S. <fnabinash@gmail.com>")]
#[command(about = "ADOF - An Automatic Dot-files Organizer Friend", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Adof in your system
    Init,

    /// Manually add any files you want to keep track of
    Add,

    /// Remove files you does not want to keep track of
    Remove,

    /// List all the files you are keeping track of
    List,

    /// Link to a GitHub repo to store your dot files
    Link {
        /// Link of the GitHub repo
        link: String,
    },

    /// Push the local changes to GitHub
    Push,

    /// Update the changes manually
    Update {
        /// Check for updates
        #[arg(short, long, default_value = "false")]
        check: bool,
    },

    /// Automatically update the changes
    AutoUpdate {
        /// Set how fast you want to auto update
        #[arg(default_value = "60")]
        min: u64,
    },

    /// Got logs of latest changes
    Log {
        /// Get the lastest local changes up to a number
        #[arg(default_value = "0")]
        num: u8,

        /// Get commits from remote repo
        #[arg(short, long, default_value = "false")]
        remote: bool,
    },

    /// Get the overview of your adof
    Summary,

    /// Deploy the dot files to your system
    Deploy {
        /// Enter the link of the github repo
        #[arg(default_value = "")]
        link: String,

        /// Deploy the dot files from a specific commit hash
        #[arg(short, long, default_value = "")]
        commit: String,
    },

    /// Unlink with the GitHub repo
    Unlink,

    /// Uninstall Adof
    Uninstall,

    /// Sponsor Me
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
            link::link(link);
        }

        Commands::Push => {
            push::push()?;
        }

        Commands::Update { check } => {
            update::update(*check)?;
        }

        Commands::AutoUpdate { min } => {
            auto_update::auto_update(*min).await?;
        }

        Commands::Log { num, remote } => {
            log::log(*num, *remote)?;
        }

        Commands::Summary => {
            summary::summary();
        }

        Commands::Deploy { link, commit } => {
            deploy::deploy(link, commit)?;
        }

        Commands::Unlink => {
            unlink::unlink()?;
        }

        Commands::Uninstall => {
            uninstall::uninstall()?;
        }

        Commands::Sponsor => {
            webbrowser::open("https://github.com/sponsors/fnabinash")?;
        }
    }

    Ok(())
}
