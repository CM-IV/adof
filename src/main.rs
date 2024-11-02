// list - list all the files that .adof is keeping track of
//
// update - manually update
//
// auto_update - to automatically update when there is changes
//      - args
//          - then can time limit like 1 hour or 1 min or 10 min to periodically check for updates
//          and when there is upate sync the changes
//          - also have option to disable the auto_update if they want to
//
// Link - link to a GitHub Repo
//      - only link the repo do not push anything until the push command is not triggered
//
// push - push the changes to GitHub Repo with commit message that include
//      - process
//          - when the user runs the push command first print a summary in a table format with file
//          name, then changes means how many lines added and deleted.
//          - then ask for conformation to push or not
//          - if user says yes then push to GitHub with commit message that contains
//              - date
//              - time
//              - files that are changed with how many lines added and deleted
//
// unlink - unlink the GitHub Repo
//      - process
//          - ask the user for conformation to unlink or not
//
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
//
// uninstall - remove adof from your machine, while uninstalling first check if there is untracked
// or uncommitted changes or not if not then uninstall if yes then ask for permission to forcefully
// uninstall.
//
// log - log the last changes they made
//      - parameters
//          - date - list all the changes happened that day
//          - files - list the changes that happened to that file in particular time
//          - number - list that nth changes happened to that whole .adof folder
//          - also they can give additional parameters like 1 or 2 or all or today to list the
//          changes they made
//
// commit - to list the last commit
//
// Idea - every day create a new branch with name equals to date of that day, and commit all the changes
// thorough out the day with time then on the next day commit the changes with a squash merge with
// all commits props like date time and the files that are changed, also if they can they can
// revert to previous changes made in that day or on the any day they want to

use clap::{Parser, Subcommand};

pub mod commands;
pub mod database;
pub mod git;

use commands::{add, init, remove};

#[derive(Parser)]
#[command(name = "adof")]
#[command(version = "v0.1.1")]
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

    /// Link to a GitHub repo to store your dot files
    Link {
        /// Link of the GitHub repo
        link: String,
    },

    /// Push the local changes to GitHub
    Push,

    /// Got logs of latest changes
    Log,

    /// Get latest commits from GitHub
    Commits {
        /// Get the last nth commits
        #[arg(default_value = "1")]
        number: u8,
    },

    /// Deploy the dot files to your system
    Deploy {
        /// Deploy the dot files from a specific commit hash
        #[arg(default_value = "Latest commit")]
        commit: String,
    },

    /// Unlink with the GitHub repo
    Unlink,

    /// Uninstall Adof
    Uninstall,

    /// Sponsor Me
    Sponsor,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init::init();
        }

        Commands::Add => {
            add::add();
        }

        Commands::Remove => {
            remove::remove();
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

        Commands::Deploy { commit } => {
            println!(
                "Deploying changes to local system. Commit hash: {:?}",
                commit
            );
        }

        Commands::Unlink => {
            println!("Unlinked with GitHub Repo");
        }

        Commands::Uninstall => {
            println!("Uninstalling Adof.");
        }

        Commands::Sponsor => {
            let _ = webbrowser::open("https://github.com/sponsors/fnabinash");
        }
    }
}
