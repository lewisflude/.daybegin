#!/bin/bash

# Confirm Installation
read -p "Do you want to install Daybegin? (y/n): " choice
if [[ $choice != [Yy] ]]; then
    echo "Installation canceled."
    exit 0
fi

# Error Handling
set -eo pipefail

# Expand Tildas to Home Directory
expand_tilde() {
    local path="$1"
    echo "${path/#\~/$HOME}"
}

# Backup Existing Configuration Files
backup_file() {
    local file="$1"
    local backup="$file.bak"
    if [[ -f "$file" && ! -f "$backup" ]]; then
        cp "$file" "$backup"
        echo "Backed up $file to $backup"
    fi
}

# Check if Git is Installed
if ! command -v git >/dev/null 2>&1; then
    echo "Git is not installed. Please install Git and run the script again."
    exit 1
fi

# Clone the Daybegin repository from GitHub to ~/.daybegin
git clone https://github.com/lewisflude/.daybegin.git "$(expand_tilde "~/.daybegin")"

# Check if Bash is available
if [ -n "$BASH_VERSION" ]; then
  echo "Setting up Daybegin for Bash..."
  echo "alias daybegin='~/.daybegin/daybegin.sh'" >> ~/.bashrc
  echo "source ~/.daybegin/daybegin.sh" >> ~/.bashrc
fi

# Check if Zsh is available
if [ -n "$ZSH_VERSION" ]; then
  echo "Setting up Daybegin for Zsh..."
  echo "alias daybegin='~/.daybegin/daybegin.sh'" >> ~/.zshrc
  echo "source ~/.daybegin/daybegin.sh" >> ~/.zshrc
  echo "source ~/.zshrc" >> ~/.zshrc
fi

# Check if Fish is available
if command -v fish >/dev/null 2>&1; then
  echo "Setting up Daybegin for Fish..."
  echo "alias daybegin='~/.daybegin/daybegin.sh'" >> ~/.config/fish/config.fish
  echo "source ~/.daybegin/daybegin.sh" >> ~/.config/fish/config.fish
fi

echo "Daybegin setup complete! You can now use the 'daybegin' command to run Daybegin."

# Source the updated shell configuration
if [ -n "$BASH_VERSION" ]; then
  source ~/.bashrc
fi

if [ -n "$ZSH_VERSION" ]; then
  source ~/.zshrc
fi
