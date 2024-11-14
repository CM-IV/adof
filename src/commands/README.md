# Dotfiles Managed with adof

Welcome! This repository contains dotfiles managed with **adof** (Automatic Dotfile Organizer Friend). These configurations are designed to be easily deployable to any system, so you can quickly set up the same environment. Whether you’re looking to replicate this setup or personalize it further, this guide will walk you through the steps.

## What is adof?

*Adof* is a tool for managing, tracking, and sharing dotfiles. With *adof*, configurations are versioned, backed up, and easily deployable. You can sync these dotfiles from this repository to your own system effortlessly.

## Getting Started

To use these dotfiles, you need to install *adof*. Once installed, you can deploy all the configurations from this repository to your system.

### 1. Install adof

You can install *Adof* using Cargo or curl. Here’s how:

#### Using Cargo

```bash
cargo install adof
```

#### Building from Source

To build *Adof* manually, clone the repository and compile it with Cargo:

```bash
git clone https://github.com/fnabinash/adof.git
```
```
cargo install --path adof/
```

### 2. Copy this Repository URL

### 3. Deploy the Dotfiles to Your System

Use the `adof deploy github-url` command to set up these dotfiles on your system.

```bash
adof deploy https://github.com/influencerusername/dotfiles
```

This command will copy the files to their respective locations on your system, creating the same configuration environment as defined in this repository.

## Customization

Once deployed, you’re free to modify these files directly on your system. *adof* will only track changes if you choose to use it on your local setup.

## Contributing

If you have improvements or tweaks to suggest, feel free to fork the repository and make a pull request! Contributions are always welcome.

## License

This repository is open-source. 

---

Enjoy your new setup and happy configuring!
