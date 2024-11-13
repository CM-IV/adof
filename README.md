# ADOF - Automatic Dotfile Organizer Friend

Managing dotfiles across multiple systems can be tedious, repetitive, and time-consuming. **adof** takes the complexity out of configuration management with a powerful, yet easy-to-use tool that tracks, manages, and deploys your dotfiles. Whether you’re a developer, sysadmin, or power user, adof helps you seamlessly keep your system configurations in sync, making your setup truly portable and shareable. With git integration, customizable tracking, and a deploy feature, *adof* is your ultimate companion for maintaining and sharing configurations with ease.

---

## About Our Organization

We’re building a community-driven organization to create powerful, open-source tools that solve real-world problems in the developer and systems management space. By supporting us, you’ll help bring more innovative projects like *adof* to life. With your sponsorship, we can continue delivering impactful tools and expanding the capabilities of our projects to meet the evolving needs of our users. We invite you to become a sponsor and join us on this journey!

## Installation

### Install from crates.io

Install *adof* using Cargo from crates.io:

```bash
cargo install adof
```

### Building from Source

To build *adof* manually, clone the repository and compile it with Cargo:

```bash
git clone https://github.com/yourusername/adof.git
cargo install --path adof/
```

---

## Commands and Usage

### `init`
- **Description**: Initializes tracking of dotfiles. Finds files based on patterns and displays them with `fzf` for easy selection.
- **Usage**: `adof init`
- **Benefit**: Quickly set up tracking of multiple files with interactive selection.

### `add`
- **Description**: Add additional files for tracking that may not match patterns.
- **Usage**: `adof add`
- **Benefit**: Easily include overlooked files for complete configuration coverage.

### `remove`
- **Description**: Remove files from tracking.
- **Usage**: `adof remove`
- **Benefit**: Clean up your tracking list by removing unnecessary files.

### `link <repo-url>`
- **Description**: Link a GitHub repository to store backups and track changes.
- **Usage**: `adof link https://github.com/yourusername/dotfiles`
- **Benefit**: Centralize your configuration management in a GitHub repo for easy access and sharing.

### `unlink`
- **Description**: Remove the linked GitHub repository.
- **Usage**: `adof unlink`
- **Benefit**: Manage your repository connections easily.

### `push`
- **Description**: Push tracked dotfiles to the linked GitHub repository.
- **Usage**: `adof push`
- **Benefit**: Sync changes manually to GitHub, ensuring backups are updated when needed.

### `update`
- **Description**: Manually update changes from original files to backups.
- **Usage**: `adof update`
- **Benefit**: Keep your backup files current with minimal effort.

### `deploy <repo-url>`
- **Description**: Deploy dotfiles from any GitHub URL to your system.
- **Usage**: `adof deploy https://github.com/username/dotfiles`
- **Benefit**: Quickly set up new systems with configuration files from GitHub.

### `uninstall`
- **Description**: Uninstall *adof* and remove all configurations.
- **Usage**: `adof uninstall`
- **Benefit**: Clean removal of all traces if you no longer need *adof*.

### `log`
- **Description**: View a log of all changes, with support for filtering by date and time.
- **Usage**: `adof log`
- **Benefit**: Track history for better visibility into file changes and management.

---

## Future Features

- **Auto-update**: Set an interval for *adof* to regularly check for and sync changes automatically.
- **Profile Management**: Maintain multiple profiles within one repository for different setups or environments.
- **File Encryption**: Securely track and deploy sensitive files with encryption support.
- **Portable Mode**: Optimize *adof* for cloud environments, using minimal resources and allowing temporary configuration deployments.

---

## Other Information

- **Contributing**: Contributions are always welcome! Check out our [CONTRIBUTING.md](link-to-CONTRIBUTING.md) for more details.
- **Security**: For details on reporting vulnerabilities, please see our [SECURITY.md](link-to-SECURITY.md).
- **License**: *adof* is open-source and available under the [MIT License](LICENSE).

Thank you for choosing *adof* for your configuration management needs! We look forward to your feedback and contributions as we continue to make *adof* even better.
