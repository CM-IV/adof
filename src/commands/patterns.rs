pub const FILE_PATTERNS: [&str; 33] = [
    // Shells
    ".bashrc", // Bash shell configuration
    ".zshrc",  // Zsh shell configuration
    ".bash_profile", // Bash profile
    ".profile", // General shell profile
    ".fish/config.fish", // Fish shell configuration

    // Editors & IDEs
    ".vimrc", // Vim configuration
    ".config/nvim/init.vim", // Neovim configuration
    ".config/nvim/**/*.vim", // Neovim plugin configurations
    ".config/nvim/**/*.lua", // Neovim Lua configurations
    ".emacs", // Emacs configuration
    ".spacemacs", // Spacemacs configuration
    ".config/atom/config.cson", // Atom editor config
    ".config/sublime-text-3/Packages/User/Preferences.sublime-settings", // Sublime Text 3 config
    ".vscode/settings.json", // VSCode settings

    // Version Control Systems
    ".gitconfig", // Git global configuration
    ".gitignore", // Git ignore files
    ".gitmodules", // Git submodules

    // Terminal Emulators
    ".config/starship.toml", // Starship prompt configuration
    ".config/tmux/tmux.conf", // Tmux configuration
    ".config/alacritty/alacritty.yml", // Alacritty terminal configuration
    ".config/wezterm/wezterm.lua", // Wezterm terminal configuration

    // Package Managers & Build Tools
    ".cargo/config", // Cargo configuration (Rust)
    ".npmrc", // NPM configuration
    ".yarnrc", // Yarn configuration
    ".docker/config.json", // Docker CLI configuration
    ".npm/_logs", // NPM logs
    ".config/pip/pip.conf", // Python pip configuration

    // Cloud/Development Environment Configs
    ".aws/config", // AWS config
    ".terraformrc", // Terraform configuration

    // Miscellaneous Configuration Files
    ".config/git/config", // Git configuration (system or project specific)
    ".config/kitty/kitty.conf", // Kitty terminal configuration
    ".config/helix/config.toml", // Helix editor configuration
    ".config/discord/settings.json", // Discord application settings
];
