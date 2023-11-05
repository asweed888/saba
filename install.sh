#!/bin/bash

set -e

VERSION=$(curl -s https://github.com/asweed888/saba/releases.atom | grep -o -E "releases/tag/v[0-9]+\.[0-9]+\.[0-9]+" | sed 's/releases\/tag\///' | head -n 1)

OS="$(uname -s)"
ARCH="$(uname -m)"

case $OS in
    "Linux")
        case $ARCH in
            "x86_64")
                TARGET=x86_64-unknown-linux-musl
            ;;
            "aarch64")
                TARGET=aarch64-unknown-linux-gnu
            ;;
        esac
    ;;
    "Darwin")
          case $ARCH in
          "x86_64")
              TARGET=x86_64-apple-darwin
          ;;
          "arm64")
              TARGET=aarch64-apple-darwin
          ;;
          esac
    ;;
esac

INSTALL_TARGET="saba-${VERSION}-${TARGET}.tar.gz"
INSTALL_TARGET_URL="https://github.com/asweed888/saba/releases/download/${VERSION}/${INSTALL_TARGET}"

HOME_BIN="$HOME/.bin"
if [ ! -e "$HOME_BIN" ]; then
    mkdir -p $HOME_BIN
    echo "[info] Created directory because $HOME_BIN was not found."
fi

if [[ ":$PATH:" != *":$HOME_BIN:"* ]]; then
    echo 'export PATH="$PATH:$HOME/.bin"' >> "$HOME/.bashrc"
    echo "[info] Added $HOME_BIN to PATH."
fi

curl -L $INSTALL_TARGET_URL -o - | tar -xzvf - && mv ./saba $HOME_BIN

alias_name="saba_install"
if ! alias | grep -q "\b$alias_name\b"; then
    echo "alias $alias_name='curl -sSL https://raw.githubusercontent.com/asweed888/saba/main/install.sh | sh -'"
    echo "[info] An alias for saba updates has been registered."
fi

exec $SHELL -l
