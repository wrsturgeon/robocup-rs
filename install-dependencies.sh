#!/bin/sh

set -eux

# Detect OS & install dependencies (currently just LLVM for bindgen)
uname -s || exit 1
case "$(uname -s)" in
*Darwin*)
  xcode-select --install || :
  brew --version || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  brew install cmake openssl llvm ant git gh
  ;;
*Linux*)
  sudo apt-get install -y software-properties-common lsb-release cmake libssl-dev llvm-dev libclang-dev clang ant git || \
  echo 'NOT finished; currently we only support `apt-get` on Linux, but this shouldn't be easy to change. Please fix and submit a PR if you can't use `apt-get`.'
  # https://github.com/cli/cli/blob/trunk/docs/install_linux.md
  type -p curl >/dev/null || sudo apt install curl -y
  curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
  && sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
  && sudo apt update \
  && sudo apt install gh -y
  ;;
*)
  echo 'Unrecognized OS'
  exit 1
  ;;
esac

# rustup
rustup --version || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup self update
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup component add rustfmt --toolchain nightly
rustup override set nightly
rustup component add clippy
cargo install cargo-asm

# Git
git config --global pull.rebase true
git config --global fetch.prune true

echo '\033[0;1;32mGood to go!\033[0m'
