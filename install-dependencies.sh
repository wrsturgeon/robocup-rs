#!/bin/sh

set -eux

# Detect OS & install dependencies (currently just LLVM for bindgen)
uname -s || exit 1
case "$(uname -s)" in
*Darwin*)
  xcode-select --install || :
  brew --version || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  brew install llvm ant
  ;;
*Linux*)
  sudo apt-get install llvm-dev libclang-dev clang ant || \
  echo 'NOT finished; currently we only support `apt-get` on Linux, but this shouldn't be easy to change. Please fix and submit a PR if you can't use `apt-get`.'
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
rustup override set nightly
rustup component add clippy

echo '\033[0;1;32mGood to go!\033[0m'
