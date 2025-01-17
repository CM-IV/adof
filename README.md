<h1 align="center">ADOF - Automatic Dotfile Organizer Friend</h1>

**Adof** helps you seamlessly keep your system configurations in sync, making your setup truly portable and shareable. With git integration, customizable tracking, and a deployed feature, *Adof* is your ultimate companion for maintaining and sharing configurations with ease.

> **For demo checkout this repo -> https://github.com/fnabinash/test-adof.git**

**Note:- You have to have `fzf` installed on your system.**

<table>
  <tr>
    <td><img src="https://github.com/fnabinash/adof/blob/main/assets/adof-init.png" alt="adof init"/></td>
    <td><img src="https://github.com/fnabinash/adof/blob/main/assets/commit-message.png" alt="commit message"/></td>
  </tr>
  <tr>
    <td><img src="https://github.com/fnabinash/adof/blob/main/assets/adof-log.png" alt="adof log"/></td>
    <td><img src="https://github.com/fnabinash/adof/blob/main/assets/adof-add.png" alt="adof add" /></td>
  </tr>
</table>

---

<p align="center"><img src="https://github.com/OxideOrg/.github/blob/main/images/oxide_logo_no_bg.png" width="15%"></p>

<h1 align="center">Oxide Org.</h1>

<p align="center">
  <a href="https://github.com/OxideOrg" target="_blank">
    <img src="https://img.shields.io/badge/github-%23000000.svg?style=for-the-badge&logo=github&logoColor=white">
  </a>
  
  <a href="https://x.com/oxide_org" target="_blank">
    <img src="https://img.shields.io/badge/OxideOrg-%23000000.svg?style=for-the-badge&logo=X&logoColor=white">
  </a>

  <a href="https://oxideorg.medium.com/" target="_blank">
    <img src="https://img.shields.io/badge/Medium-%23000000?style=for-the-badge&logo=medium&logoColor=white">
  </a>
</p>

<p align="center">We build fast, reliable and easy-to-use tools for developers.</p>

If you love **Adof**, you are most likely going to love my future projects (obviously going to be free and open source for everyone), but those are going to be more focused on enterprise or professional use cases. Already, planned out 7 projects that will help you in your different stages of development. So, I need your help to build those projects. Of course, your help never going to be free, you will get your exclusive benefits after the Org is launched.

As everything going to be free and open-source, I am planning to sustain this org on sponsorships. So, the plan is simple after the org launch it going to have different tiers but as an individual, you will get two tiers one is **tech enthusiast** and the other is **tech influencer**. 

If you have some suggestions or need more info please share them with me on Twitter (https://x.com/fnabinash) or email me at fnabiansh@gmail.com.

---

## Comparison

| Feature                  | adof                                    | GNU Stow | Homeshick | chezmoi   |
|--------------------------|-----------------------------------------|----------|-----------|-----------|
| **Pattern-based Tracking**    | Yes (fzf for selection)               | No       | No        | No        |
| **Commit Message Customization** | Yes                              | No       | No        | Limited   |
| **Cloud Portability**         | Planned                              | No       | No        | No        |
| **README Generation**         | Yes                                   | No       | No        | No        |
| **File Encryption**           | Planned                              | No       | No        | Yes       |
| **Deploy from GitHub URL**    | Yes                                   | No       | No        | Yes       |
| **Auto Update with Time Limits** | Planned                            | No       | No        | No        |
| **Profiling Support**         | Planned                              | No       | No        | Limited   |

## Installation

### Install from crates.io

Install *Adof* using Cargo from crates.io:

```bash
cargo install adof
```

### Building from Source

To build *adof* manually, clone the repository and compile it with Cargo:

```bash
git clone https://github.com/fnabinash/adof.git
```
```
cargo install --path adof/
```

---

## Commands and Usage

### whatis
- `adof whatis <command-name>` - If you do not know which command what does then you use the `whatis` command to get detailed info about any command.
- For example `adof whatis init` give you detailed info about `init` command.

### `init`
- When you run `adof init` quickly, open the `fzf` window and list the files that match a specific pattern(you can improve the pattern by checking out [pattern.rs](https://github.com/fnabinash/adof/blob/main/src/commands/patterns.rs) and make a PR), then use the `tab` to select multiple files and when ready hit enter.
- Adof will create backup files of all the files you have selected in `.adof/` dir.
- Adof automatically stage the file and commit the changes.
- If you think that the pattern can not cover all the files you want to track do not worry, for this scenario adof `add` command.

### `add`
- By running `adof add` you can add any files you want to keep track of and back of.
- It also opens a `fzf` window, and you can use `tab` to select multiple files.

### `remove`
- If you accidentally added a file that you do not want to keep track of, you can remove it using `adof remove` command.
- It list all the files that you are keeping track of and show them in `fzf` window and you can use `tab` to select file that you want to remove.
- It will remove the files, stage the changes and commit it.

### `link <repo-url>`
- If you are a tech influence most likely, your followers always ask you for your dotfiles. For that reason, I made the link command which takes a GitHub URL and links the local repo to the remote.
- It will auto-generate a README for you with instruction to copy the file to their local system with just one command.
- They like your ditfile, install Adof and just run one command and boom they have the exam files in the correct place in just a second.

### `unlink`
- `adof unlink` does what it said, it unlinks the remote repo with your local repo.

### `push`
- You do not have to manually go to the `.adof` dir and run `git push` to push the changes to GitHub, you can run `adof push` where ever you are in your terminal.
- If you have already set the credentials then it will push the changes if not it going to ask for the creds.

### `update`
- This command does what it is named for, it updates the changes.
- At anywhere in your terminal, just run the `adof update` it will fetch the changes in the files you are keeping track of and make the changes, stage those and commit it.
- Also it has a flag `adof update --check` or `adof update -c`, sometimes you just want to check the files that have changes to make, then run any of these commands it will list all the files that have changes but not backed up but it does not update the changes. To update, you have to run `adof update`.
- In future, I have plans to make `auto-update` command which periodically checks for changes if there are any changes it automatically backs up the changes.

### `deploy <repo-url>`
- Deploy is the most interesting command of Adof.
- It helps you copy any config from the internet that is made using Adof.
- It has two parameters, one is a GitHub URL and the other is a commit hash.
- `adof deploy giithu-rul -c commit-id` - it will copy all the files from that GitHub repo from that commit id and place it where it is meant to be.
- `adof deploy github-url` - it does the same thing but from the latest commit.
- `adof deploy -c commit-id` - it looks for the commit in your local repo and copies the file from that instance to the actual file location. For example, if you made some changes to your config and then realise you do not like it then you can run `adof log` to list all the commits and copy the commit id then run the `adof deploy -c ccommit-id` then your file is set to that instance.
- `adof deploy` - it deploys the latest commit instance from your local repo.

### `uninstall`
- It uninstalls Adof from your system completely.

### `log`
- It takes one flag and one parameter. The flag is `--remote` or `-r` and the parameter is `number less than 100`.
- `log` command is used to list the git commit or logs in your `.adof` folder history.
- `adof log` - It list all the commits that you have not pushed to remote means the changes that are present in your local repo but not in the remote repo. If your local and remote are up to date then it lists the last 5 comits.
- `adof log [number]` - It lists the last [number] commits present in your local repo.
- `adof log -r` or `adof log --remote` - It lists the last 5 commits from your remote repo, If you have not configured your remote repo with local then it will list the last 5 commits from your local repo.
- `adof log -r [number]` or `adof log --remote [number]` - It lists the last [number] commits from your remote repo, if not configured then it lists from the local repo.

### `list`
- It will list all the file that you are keeping track of using Adof.

---

## Other Key Features

In addition to core commands, *Adof* offers these valuable features:

- **Enhanced Commit Messages**: Automatically generated commit messages with detailed information on the day, date, time, and number of files and lines changed, including specific filenames and line numbers. This helps you easily track what has changed.
  
- **Automated README Generation**: For users who wish to share their dotfiles on GitHub, *adof* can automatically generate a `README.md` file containing essential information about the repository contents and usage instructions. The README can be customized to add more personal notes, but by default, it includes everything a user needs to get started.

- **Error Handling**: Provides detailed error messages and logging to help identify and resolve issues quickly, enhancing stability and ease of use.

## Future Plans

- **Auto-update**: Set an interval for *adof* to regularly check for and sync changes automatically.
- **Profile Management**: Maintain multiple profiles within one repository for different setups or environments(like for work, personal, streaming etc).
- **File Encryption**: Securely track and deploy sensitive files with encryption support.
- **Portable Mode**: Optimize *Adof* for cloud environments, using minimal resources and allowing temporary configuration deployments.

---

## Other Information

- **Contributing**: Contributions are always welcome! Check out our [CONTRIBUTING.md](link-to-CONTRIBUTING.md) for more details.
- **Security**: For details on reporting vulnerabilities, please see our [SECURITY.md](link-to-SECURITY.md).
- **License**: *adof* is open-source and available under the [MIT License](LICENSE).

Thank you for choosing *adof* for your configuration management needs! We look forward to your feedback and contributions as we continue to make *adof* even better.
