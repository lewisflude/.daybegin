#!/bin/bash

# Clone the Daybegin repository from GitHub to ~/.daybegin
git clone https://github.com/lewisflude/daybegin.git ~/.daybegin

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

