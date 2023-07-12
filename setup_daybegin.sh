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

# Determine the user's shell
if [[ "$SHELL" == *"/bash"* ]]; then
    echo "Setting up Daybegin for Bash..."
    backup_file "$(expand_tilde "$HOME/.bashrc")"
    echo "alias daybegin='$(expand_tilde "~/.daybegin/daybegin.sh")'" >> "$(expand_tilde "$HOME/.bashrc")"
    echo "source $(expand_tilde "~/.daybegin/daybegin.sh")" >> "$(expand_tilde "$HOME/.bashrc")"
fi

if [[ "$SHELL" == *"/zsh"* ]]; then
    echo "Setting up Daybegin for Zsh..."
    backup_file "$(expand_tilde "$HOME/.zshrc")"
    echo "alias daybegin='$(expand_tilde "~/.daybegin/daybegin.sh")'" >> "$(expand_tilde "$HOME/.zshrc")"
    echo "source $(expand_tilde "~/.daybegin/daybegin.sh")" >> "$(expand_tilde "$HOME/.zshrc")"
    if [ -f "$(expand_tilde "$HOME/.zshrc")" ]; then
        echo "source $(expand_tilde "~/.zshrc")" >> "$(expand_tilde "$HOME/.zshrc")"
    fi
fi

if [[ "$SHELL" == *"/fish"* ]]; then
    echo "Setting up Daybegin for Fish..."
    backup_file "$(expand_tilde "$HOME/.config/fish/config.fish")"
    echo "alias daybegin='$(expand_tilde "~/.daybegin/daybegin.sh")'" >> "$(expand_tilde "$HOME/.config/fish/config.fish")"
    echo "source $(expand_tilde "~/.daybegin/daybegin.sh")" >> "$(expand_tilde "$HOME/.config/fish/config.fish")"
fi

echo "Daybegin setup complete! You can now use the 'daybegin' command to run Daybegin."
