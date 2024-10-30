// Init - init the tool and find all the dot files and copy to the .adof folder
//      - process
//          - when running the init command it will open a window like fzf with left side containing
//          the files name and path and right side shows the preview with synatx highlight.
//          - it does not pull every single file, rather it only pulls the files that it finds as
//          dot files or config files then you can use your `tab` to select files and use `arrows`
//          to move between files after selecting all files hit `enter` to complete the
//          initialization process
//
// Add - after initialization, users can easily search and add any files they want to track
//      - process
//          - when running `add` command it opens a window like fzf in left side file with path and
//          in right sight the file contents with syntax highlights then you can use `tab` to
//          select files and `enter` to complete the process
//
// Link - link to a github Repo
//      - only link the repo do not push anything untill the push command is not triggeered
//
// push - push the changes to github Repo with commit message that include
//      - process
//          - when the user runs the push command first print a summary in a table format with file
//          name, then changes means how many lines added and deleted.
//          - then ask for comformation to push or not
//          - if user says yes then push to github with commit message that contains
//              - date
//              - time
//              - files that are changed with how many lines added and deleted
//
// unlink - unlink the github Repo
//      - process
//          - ask the user for conformation to unlink or not
//
// deploy - copy all the dot files from github and store in the local machine at perfect places
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
//          user runs the deploy commmand
//          - then do a small animation to celebrate🎉
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
// Idea - every day create a new branch with name equals to date of that day, and commit all the changes
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

    /// Manually add any files you want to keep track of
    Add,

    /// Link to a GitHub repo to store your dotfiles
    Link {
        /// Link of the GitHub repo
        link: String,
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

        Commands::Add => {
            println!("Manually adding new files.");
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
    }
}
