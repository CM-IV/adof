pub fn what_is(command: &str) {
    match command {
        "" => whatis(),
        "init" => init(),
        "add" => add(),
        "remove" => remove(),
        "link" => link(),
        "unlink" => unlink(),
        "push" => push(),
        "update" => update(),
        "deploy" => deploy(),
        "uninstall" => uninstall(),
        "log" => log(),
        "list" => list(),
        "sponsor" => sponsor(),
        "future" => future(),
        _ => other(),
    }
}

fn init() {
    let output = r"
adof init Command - Detailed Documentation

Flags: Currently, init does not accept specific flags, but it may in future updates 
to allow additional filtering or pattern customization directly in the command.

Parameters: No positional parameters are required. The init command works based on predefined patterns 
(editable by modifying patterns.rs) and does not require file paths or specific filenames as arguments.

Detailed Steps of init

1. Pattern Matching:

- The command begins by identifying files based on patterns defined in patterns.rs.
- Patterns can be customized by the user to adjust what types of files are considered for tracking.
- Examples of default patterns could include files like .bashrc, .zshrc, .vimrc, etc., 
depending on the configurations commonly found in users’ home directories.

2. Interactive Selection:

- Once matching files are found, they are displayed in fzf, a command-line fuzzy finder.

In fzf, users can:
- See a list of matched file paths on the left
- View the file contents on the right, aiding in easy identification of which files to track.
- Users can use the tab key to select multiple files and the enter key to confirm selection and proceed.

3. Backup Creation:

- After selection, adof init backs up each selected file by copying it into a .adof/ directory.
- This .adof/ directory is created at the root of the user's home directory or 
within the repository where adof is initialized.
- The original structure and format of each file are preserved within .adof/.

4. Automatic Git Staging and Committing:

- adof stages and commits each of the selected files in .adof/ using Git.
- Commit messages are automatically generated, 
including details such as the day, date, time, and the number of files added or modified.
- The goal is to provide clear version control from the outset, 
making it easier to track changes over time.

5.README.md Generation:

- In the .adof/ directory, adof automatically generates a README.md file.

This file provides:
- An overview of the files being tracked.
- Instructions for using adof commands to manage these files.
- Details on how others can copy these configurations if 
the repository is shared publicly (e.g., on GitHub).


Example Workflow

Assuming you are starting with a fresh installation of adof and 
want to begin tracking specific dotfiles, you would run:

adof init

adof scans the system for files matching predefined patterns.
In the fzf interface, you use tab to select files (e.g., .bashrc, .vimrc) and 
press enter to confirm.

These files are then:
- Copied to .adof/
- Staged and committed with an autogenerated message
- A README.md is created within .adof/, documenting the repository’s purpose and usage.

Potential Use Cases for init

First-Time Setup: Ideal for users starting with adof, 
init provides a seamless way to begin managing configuration files from the start.

Collaboration Ready: By backing up files and committing them with descriptive messages, 
adof makes it easy for users to share and collaborate on configurations with minimal setup.

Note: init is a foundational command and cannot be skipped. Once initialized, 
users can leverage the full suite of adof commands to continue adding, removing, and 
managing their configuration files.
        ";

    println!("{}", output);
}

fn add() {
    let output = r"
adof add Command - Detailed Documentation

Flags: The add command currently does not accept any specific flags but 
may support additional options in future updates for further customization.

Parameters: No positional parameters are required for the add command. 
It works interactively without requiring file paths or filenames as arguments.

Detailed Steps of add

1. Interactive File Selection:

- The add command initiates an interactive file selection process that 
scans the system for files.
- Unlike init, which works based on predefined patterns, 
add provides access to all files on the system.
- The fzf interface allows users to quickly search through files and 
choose which ones they want to add for tracking.

In fzf, users can:
- See a list of all files available on the system, displayed in the left panel.
- Preview the contents of each selected file in the right panel, 
helping users confirm which files they want to track.
- Use the tab key to select multiple files and the enter key to finalize their selections.

2. Adding Files to Tracking:

- Once selections are made, adof copies the chosen files into the `.adof/` directory.
- This directory serves as a backup location where tracked files are managed.

3. Automatic Git Staging and Committing:

- After the files are added to the `.adof/` directory, adof stages and 
commits them using Git.
- The commit messages are automatically generated and 
include relevant information such as the day, date, time, 
the number of files added, and file names.
- Enhanced commit messages ensure that each change is well-documented for easy tracking.

4. Integration with init Command:

- The add command works seamlessly with the init command. 
While init provides an initial setup based on patterns, 
add offers flexibility for files that may not match those patterns.
- Users can run init to set up initial tracking and 
later use add to incorporate any additional files they wish to manage.

Example Workflow

If you have initialized adof with the init command but 
later realize there are additional configuration files 
you want to manage, you would run:

adof add

adof opens the fzf interface, allowing you to select additional files for tracking.
Selected files are then:
- Copied to the `.adof/` directory
- Staged and committed with an autogenerated message that 
includes detailed information about the added files.

Potential Use Cases for add

Custom Configurations: For configurations that do not match the initial patterns defined in init, 
add provides a straightforward way to track custom files.

Iterative Setup: Users can gradually expand the list of tracked files by adding new files as 
their needs evolve, without altering the initial setup.

Collaboration Ready: Like init, add also generates commit messages that 
make it easy for users to share, collaborate, and keep track of changes to 
individual configuration files.

Note: add is intended to complement the init command, 
allowing users to extend their configuration management 
without having to redefine initial patterns.
        ";

    println!("{}", output);
}

fn remove() {
    let output = r"
adof remove Command - Detailed Documentation

Flags: The remove command does not currently accept flags.

Parameters: No positional parameters are required. remove works interactively 
without needing filenames as arguments.

Detailed Steps of remove

1. Interactive File Removal:

- Running the remove command opens an interactive `fzf` window 
displaying all files currently tracked in the `.adof` directory.
- Users can view and select multiple files to remove using the tab key, 
confirming their selection by pressing enter.

2. Removing Files from Tracking:

- Once the selection is made, adof removes the chosen files from the `.adof` directory, 
effectively stopping them from being tracked by adof.
- These files are no longer staged or committed in future operations, 
and no backups are maintained in `.adof`.

3. Automatic Git Staging and Committing:

- After removing the files, adof automatically stages and commits the changes.
- The commit message includes information about which files were removed, 
enabling easy tracking of changes 
and ensuring that the repository history remains clear.

Example Workflow

If you accidentally added files that you don’t want to manage with adof, 
you would run:

adof remove

In the fzf interface:
- Select the files you wish to stop tracking.
- Confirm the selection by pressing enter.
- The selected files are removed from `.adof` and staged and 
committed with an appropriate message.

Potential Use Cases for remove

Accidental Additions: Users can quickly untrack files 
that were mistakenly added with the `init` or `add` commands.

Periodic Cleanup: Ideal for users who want to reduce the number of tracked files, 
particularly for unused or deprecated configurations.

Note: The remove command makes it easy to manage the list of tracked files, 
enabling users to maintain a clean and relevant backup directory in `.adof`.
    ";

    println!("{}", output);
}

fn link() {
    let output = r"
adof link Command - Detailed Documentation

Flags: The link command does not currently accept flags.

Parameters:
- <repo-url>: A required parameter that takes a GitHub repository URL to 
link the local `.adof` directory to a remote repository.

Detailed Steps of link

1. Linking the Local Repository:

- The `link` command takes a GitHub URL as input, enabling users to 
link their local `.adof` repository to the specified remote repository.
- This setup allows for easy sharing and collaboration by enabling push 
and pull operations between the local and remote repositories.

2. Auto-Generating a README.md:

- Once the link is established, adof automatically generates a `README.md` file 
within the `.adof` directory.
- This README includes instructions for others to copy these configurations 
to their systems if they wish to replicate the setup.
- The README file contains essential usage information, 
ensuring that anyone accessing the repository knows how to deploy the configurations easily.

3. Setting Up for Git Operations:

- After linking, the command configures Git credentials and 
remote settings to streamline future operations, 
such as pushing updates from the local `.adof` directory to the linked remote repository.

Example Workflow

If you want to share your dotfiles with followers or 
collaborators on GitHub, you would run:

adof link <repo-url>

After linking:
- A README.md is generated in the `.adof` directory with usage instructions.
- You can use `adof push` to send updates to the remote repository.

Potential Use Cases for link

Tech Influencers & Content Creators: Ideal for users who frequently share configuration 
setups with a large audience. 
By linking the local directory to a GitHub repository, 
followers can easily clone and deploy these configurations.

Collaborative Environments: Teams working with standardized environments can link to a central repository, 
enabling easier sharing and updates to configuration files.

Note: The link command simplifies collaboration, making it easier for users to manage configuration files 
in a public or private repository.
    ";

    println!("{}", output);
}

fn unlink() {
    let output = r"
adof unlink Command - Detailed Documentation

Flags: The unlink command does not accept any flags.

Parameters: No parameters are required.

Detailed Steps of unlink

1. Disconnecting the Local Repository from Remote:

- Running the `unlink` command removes the remote 
association of the local `.adof` directory.
- This operation unlinks the GitHub repository URL configured with 
the `link` command, meaning that subsequent pushes 
or pulls will no longer affect the remote repository.

2. Preserving Local Changes:

- The unlink command only removes the remote link but 
does not delete any local files or history in the `.adof` directory.
- Users can continue to manage and modify tracked files locally, 
even without the remote association.

Example Workflow

If you decide to stop sharing your `.adof` directory with 
a remote repository, you would run:

adof unlink

After unlinking:
- No changes are pushed to or pulled from the previously linked GitHub repository.
- All local changes in `.adof` are preserved.

Potential Use Cases for unlink

Transition to Private Work: Users can unlink from a public repository when 
transitioning configuration files 
to a private environment or personal use only.

Decoupling Remote Sharing: Useful for those who initially shared their 
configurations but no longer wish to 
maintain a public or collaborative repository.

Note: unlink provides flexibility by allowing users to remove 
remote links without disrupting their local configuration.
    ";

    println!("{}", output);
}

fn push() {
    let output = r"
adof push Command - Detailed Documentation

Flags: The push command does not accept any flags.

Parameters: No positional parameters are required.

Detailed Steps of push

1. Automatic Git Push:

- The `push` command automates the process of pushing changes 
in the `.adof` directory to the linked remote repository.
- This command is designed to save users the hassle of navigating to 
the `.adof` directory and manually running `git push`.

2. Authentication:

- If Git credentials have already been set up for the linked repository, 
`adof` will automatically use these credentials to complete the push.
- If credentials are not set, `adof` will prompt the user to enter them, 
ensuring secure access to the remote repository.

3. Push Confirmation and Feedback:

- After the command completes, adof provides feedback indicating whether 
the push was successful or if any errors were encountered.

Example Workflow

Once you have made changes to your tracked files 
and want to push them to the remote repository, simply run:

adof push

After pushing:
- All staged changes in `.adof` are uploaded to the linked remote repository, 
keeping your remote and local repositories in sync.

Potential Use Cases for push

Routine Backups: Users can regularly run `adof push` 
to keep their dotfiles updated on a remote repository, 
ensuring they have a backup available if needed.

Collaboration and Sharing: If the `.adof` directory is linked to a shared repository, 
this command allows users to push their latest changes, 
making it easier for collaborators to access updated configurations.

Note: push streamlines the process of updating a remote repository, 
allowing users to focus on file management 
rather than manual Git operations.
    ";

    println!("{}", output);
}

fn update() {
    let output = r"
adof update Command - Detailed Documentation

Flags:
- `--check` or `-c`: Use this flag to check for file changes without 
updating them in the `.adof` directory.

Parameters: No positional parameters are required.

Detailed Steps of update

1. Change Detection:

- Running `adof update` causes the tool to 
scan all tracked files for any changes.
- If any tracked files have been modified, `adof` stages and 
commits these changes in the `.adof` directory.

2. Staging and Committing Changes:

- For each modified file, `adof` stages the change and 
generates a commit with an automatically crafted message.
- The commit message includes details such as the day, date, time, and 
number of changes made, helping users maintain a comprehensive change history.

3. Using the Check Flag:

- If the user runs `adof update --check` or `adof update -c`, 
adof only lists files with changes but does not stage or commit them.
- This is useful when users want to see which files have been modified 
without immediately updating the `.adof` repository.

Example Workflow

If you want to update your `.adof` repository with the 
latest changes in your tracked files, run:

adof update

Alternatively, to view changes without updating, run:

adof update --check

After updating:
- All modified files are staged and committed in the 
`.adof` repository, preserving a record of changes.

Potential Use Cases for update

Regular Configuration Sync: Useful for users who 
frequently change their dotfiles and wish to keep their backups up to date.

Selective Tracking: With the `--check` flag, 
users can monitor changes without immediately committing, 
providing flexibility in tracking configurations.

Note: update offers a reliable way to keep your `.adof` 
repository synchronized with your latest configurations, 
helping maintain a clear history of modifications.
    ";

    println!("{}", output);
}

fn deploy() {
    let output = r"
adof deploy Command - Detailed Documentation

Flags:
- `-c` or `--commit`: Specify a specific commit hash to deploy 
from the linked or local repository.

Parameters:
- <repo-url> (optional): GitHub URL of a repository from which 
configurations will be deployed.

Detailed Steps of deploy

1. Deploying Configurations from a Remote Repository:

- If a GitHub URL is provided, `adof deploy` clones or 
fetches the specified repository.
- By default, `adof deploy <repo-url>` will pull the latest commit's 
configurations from the remote repository.

2. Deploying Configurations from a Specific Commit:

- To deploy configurations from a particular commit, 
use the `-c` flag followed by the commit hash.
- This command applies configurations from that specific version, 
which can be helpful if users want to revert to a previous setup.

3. Deploying Local Commits:

- Running `adof deploy -c <commit-id>` without specifying a URL allows users to 
deploy configurations from a local commit in `.adof`.
- This is useful if users want to roll back to 
a particular configuration state in their local repository.

Example Workflow

To deploy the latest configurations from a remote repository, run:

adof deploy <repo-url>

To deploy configurations from a specific commit in the remote or 
local repository, run:

adof deploy -c <commit-id>

After deployment:
- The specified configurations are applied, 
and affected files are updated based on the chosen commit or repository.

Potential Use Cases for deploy

Environment Setup: Ideal for users setting up new systems or environments, 
enabling quick configuration replication.

Version Control: By deploying specific commits, 
users can roll back to previous configurations, 
providing flexibility in setup management.

Note: deploy simplifies the process of configuring new systems or
restoring previous configurations, 
enabling seamless and flexible environment management.
    ";

    println!("{}", output);
}

fn uninstall() {
    let output = r"
adof uninstall Command - Detailed Documentation

Flags: The uninstall command does not accept any flags.

Parameters: No positional parameters are required.

Detailed Steps of uninstall

1. Complete Removal of adof:

- Running `adof uninstall` removes all adof files, directories, 
and settings from the system.
- This includes the `.adof` directory, tracked files, Git history, 
and any configuration files created by adof.

2. Preserving the System State:

- The uninstall command ensures that no configuration files are left behind, 
allowing users to fully remove adof without affecting other system files.
- The user's original configuration files remain untouched outside of `.adof`.

Example Workflow

If you wish to completely remove adof from your system, simply run:

adof uninstall

After uninstalling:
- All adof data and configurations are deleted, 
leaving the system in its pre-installation state.

Potential Use Cases for uninstall

System Cleanup: Users who no longer need adof can use the command to 
fully remove it from their systems.

Switching Tools: Those migrating to different configuration management 
tools may want to remove adof completely.

Note: uninstall provides a straightforward way to remove all traces of adof, 
ensuring a clean system without leftover files or settings.
    ";

    println!("{}", output);
}

fn log() {
    let output = r"
adof log Command - Detailed Documentation

Flags:
- `-r` or `--remote`: Displays commits from the linked remote repository 
instead of the local `.adof` history.

Parameters:
- <number>: An optional parameter specifying the number of commits to display, 
up to a maximum of 100.

Detailed Steps of log

1. Displaying Local Commit History:

- Running `adof log` shows the last 5 local commits by default.
- Users can specify a number (up to 100) to view additional commit history.

2. Displaying Remote Commit History:

- With the `-r` or `--remote` flag, `adof` retrieves commit history from 
the remote repository.
- If the local and remote repositories are not yet linked, 
this flag defaults to showing the last 5 local commits.

3. Viewing Specific Commit Counts:

- When combined with a number (e.g., `adof log 10`), 
`adof` shows the last specified number of commits, 
enabling detailed review of configuration changes over time.

Example Workflow

To display the last 5 commits from the local `.adof` history, run:

adof log

To display the last 10 commits from the remote repository, run:

adof log -r 10

After reviewing:
- The user can gain a clear understanding of changes over time, 
helping track the evolution of configurations.

Potential Use Cases for log

Change Tracking: Users can regularly view their commit history to monitor changes 
and track configuration updates.

Troubleshooting: By reviewing the commit history, 
users can identify recent changes that may have caused issues.

Note: log provides valuable insight into configuration history, 
enhancing transparency and version control within adof.
    ";

    println!("{}", output);
}

fn list() {
    let output = r"
adof list Command - Detailed Documentation

Flags: The list command does not accept any flags.

Parameters: No positional parameters are required.

Detailed Steps of list

1. Displaying Tracked Files:

- Running `adof list` generates a detailed list of all files currently tracked by adof.
- This list is displayed in the terminal, 
providing a convenient overview of managed configurations.

2. Interactive FZF Selection (Optional):

- If desired, the user can configure `adof` to launch the `fzf` fuzzy finder, 
enabling an interactive view of tracked files for easier navigation.
- Users can press `Tab` to select specific files for inspection.

Example Workflow

To view all files managed by adof, simply run:

adof list

After listing:
- Users gain a clear overview of managed configurations, 
helping ensure all critical files are tracked.

Potential Use Cases for list

Configuration Review: The list command is ideal for users wishing to 
verify which files are currently backed up by adof.

Documentation Support: This overview helps users document which 
configurations are managed within `.adof`.

Note: list provides users with a centralized view of tracked files,
making it easier to monitor managed configurations.
    ";

    println!("{}", output);
}

fn sponsor() {
    let output = r"
adof sponsor Command - Detailed Documentation

Flags: The sponsor command does not accept any flags.

Parameters: No positional parameters are required.

Detailed Overview of sponsor

1. Supporting Adof's Development:

- Running `adof sponsor` displays information on 
how users can contribute to adof’s development via sponsorship.
- Details include sponsorship tiers, rewards, 
and how funds help expand adof’s functionality.

2. Sponsorship Options:

- Users can choose from multiple sponsorship tiers, 
each offering unique benefits and contributing to new features.
- Examples of sponsorship benefits include priority support, 
early access to beta features, and public recognition as a supporter.

3. Direct Access to Sponsorship Links:

- `adof sponsor` provides direct links to adof’s GitHub sponsorship page, 
making it easy for users to learn more and support adof directly.

Example Workflow

To view adof sponsorship options, simply run:

adof sponsor

After running:
- Users will have clear information on how they can support adof 
and enjoy unique benefits in return.

Potential Use Cases for sponsor

Community Contribution: The sponsor command provides users with 
an easy way to contribute to adof’s growth.

Feature Access: Sponsors can enjoy early or beta access to certain features, 
enhancing their experience with adof.

Note: sponsor helps foster community support, 
empowering users to contribute toward new features and improvements.
    ";

    println!("{}", output);
}

fn future() {
    let output = r"
adof future Command - Upcoming Features Overview

Overview:

The `future` command provides a glimpse into the exciting new features and 
improvements planned for adof.
These upcoming functionalities are designed to make adof more versatile, 
powerful, and user-friendly.

Detailed Overview of Planned Features:

1. Auto-Update Feature:

- Adof will soon support an `auto-update` command, 
allowing users to set an interval for automatic syncing.
- This feature is designed to periodically check for and 
commit changes in tracked files, 
ensuring configurations are always up-to-date.

2. Profile Management:

- Users will be able to create multiple profiles within a single adof repository, 
each representing different setups.
- This is useful for users who want to 
manage different configurations for various environments, 
such as work, personal, or public profiles.

3. File Encryption:

- Adof will include file encryption options, 
allowing users to securely track sensitive configurations.
- This feature is ideal for protecting credentials or private files, 
ensuring sensitive information remains secure.

4. Portable Mode for Cloud Environments:

- A lightweight, resource-efficient mode will be developed to 
optimize adof for cloud-based usage.
- This feature will allow adof to work seamlessly in cloud environments, 
providing a convenient way to deploy configurations across devices.

5. Enhanced Git Features:

- More advanced Git operations, such as pull requests, 
diffing, and conflict resolution, will be integrated into adof.
- These enhancements will make it easier to 
manage version control directly within adof, 
improving workflow efficiency.

6. Advanced Diffing and Change Visualization:

- Adof will support in-depth file comparison and visualization of changes, 
enabling users to see differences between commits.
- This feature will provide a clearer understanding of modifications over time, 
making it easier to track complex changes.

7. Expanded README Customization:

- Users will have more control over the generated README.md, 
including templates and additional content options.
- This feature will enable users to personalize documentation for 
their `.adof` repositories, enhancing shareability.

Potential Impact of Future Features:

Enhanced Usability: Features like auto-update, profile management, 
and expanded README customization will make adof more user-friendly.

Security Improvements: With encryption support, users can confidently manage sensitive files, 
making adof suitable for a wider range of use cases.

Cloud Compatibility: Portable Mode will allow users to deploy adof in cloud environments, 
making configuration management more accessible.

Note: These future features showcase adof's commitment to continuous improvement, 
providing users with powerful new tools and options for managing configurations.
    ";

    println!("{}", output);
}

fn whatis() {
    let output = r"
adof whatis Command - Detailed Documentation

Flags: 
- The `whatis` command does not currently accept flags but 
may support them in future versions for additional functionality.

Parameters:
- `<command>`: The only parameter required by 
`whatis` is the specific command name the user wants detailed information about. 
- If `<command>` is left blank, 
`whatis` will provide a brief summary of each command available in adof.

Detailed Steps of whatis

1. Functionality Overview:

- Running `adof whatis <command>` displays detailed documentation for any command specified.
- If no specific command is provided, `adof whatis` provides an overview of all commands, 
including each command’s core functionality, usage, and parameters.

2. Usage with Specific Commands:

- For each adof command (such as `init`, `add`, `remove`, etc.), 
`whatis` provides a unique, 
in-depth description tailored to that command.
- This helps users understand the purpose and usage of each command individually, 
including flags, parameters, and typical workflows.

3. Auto-Updating Documentation:

- As new features are added to adof, 
the `whatis` command documentation dynamically updates to reflect these changes.
- This feature ensures users always have access to 
the latest information and best practices for using adof.

4. Example of `whatis` Usage:

To see documentation on a specific command, run:

adof whatis init

This will output a detailed breakdown of the `init` command, covering its flags, 
parameters, purpose, and example usage.

If you want an overview of all commands, simply run:

adof whatis

This displays a high-level summary of each command available in adof.

5. Potential Use Cases for whatis

Learning Tool: The `whatis` command acts as a built-in help system, 
making it easy for new users to understand adof's capabilities.

Command Reference: Regular users can refer to 
`whatis` for a quick refresher on specific commands, 
making it an essential resource for maximizing adof’s utility.

Troubleshooting Aid: By providing documentation on parameters and expected outputs, 
`whatis` can help users identify issues with command usage.

Note: The whatis command is a valuable resource for learning, reference, 
and troubleshooting, making adof easy to use and understand for users at all experience levels.
    ";

    println!("{}", output);
}

fn other() {
    let output = r"
    You have entered a wrong command, that is not currently supported by Adof.
        ";
    println!("{}", output);
}
