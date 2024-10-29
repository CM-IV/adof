// Init - init the tool and find all the dot files and copy to the .adof folder
//
// Link - link to a github Repo
//
// push - push the changes to github Repo with commit message that include
//          - date
//          - time
//          - files that are changed
//
// unlink - unlink the github Repo
//
// deploy - copy all the dot files from github and store in the local machine at perfect places
//
// uninstall - remove adof from your machine, while uninstalling first check if there is untracked
// or uncommitted changes or not if not then uninstall if yes then ask for permission to forcefully
// uninstall.
//
// log - log the last changes thay made
//      - parameters
//          - date - list all the changes happend that day
//          - files - list the changes that happend to that file in perticular time
//          - number - list that nth changes happened to that whole .adof folder
//          - also they can give additional parameters like 1 or 2 or all or today to list the
//          changes they made
//
// commit - to list the last commit
//
// Idea - every day create a new branch on the date of that day, and commit all the changes
// thorough out the day with time then on the next day commit the changes with a squash merge with
// all commits props like date time and the files that are changed, also if they can they can
// revert to previous changes made in that day or on the any day they want to

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "adof")]
#[command(about = "ADOF - An Atomatic Dot-files Organizer Friend", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the latest version
    Version,

    /// Initialize Adof in your system
    Init,

    /// Link to a GitHub repo to store your dotfiles
    Link {
        /// Link of the GitHub repo
        link: String
    },

    /// Push the local changes to GitHub
    Push,

    /// Got logs of latest changes
    Log,

    /// Get latest commits from GitHUb
    Commits {
        /// Get the last nth commits
        #[arg(default_value = "1")]
        number: u8,
    },

    /// Unlink with the GitHub repo
    Unlink,

    /// Deploy the dot files to your system
    Deploy {
        /// Deploy the dot files from a specific commit hash
        #[arg(default_value = "Latest commit")]
        commit: String,
    },

    /// Uninstall Adof
    Uninstall,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Version => {
            println!("adof v0.1.0");
        }

        Commands::Init => {
            println!("Initializing adof.");
        }

        Commands::Link { link } => {
            println!("Linking with GitHub Repo. Link: {:?}", link);
        }
        
        Commands::Push => {
            println!("Pushing changes to GitHub");
        }

        Commands::Log => {
            println!("Printing last changes.");
        }

        Commands::Commits { number } => {
            println!("Printing last {} commits", number);
        }

        Commands::Unlink => {
            println!("Unlinked with GitHub Repo");
        }

        Commands::Deploy { commit } => {
            println!("Deploying changes to local system. Commit hash: {:?}", commit);
        }

        Commands::Uninstall => {
            println!("Uninstalling Adof.");
        }
    }
}
