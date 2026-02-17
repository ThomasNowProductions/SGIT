#!/bin/sh
set -eu

INSTALL_DIR="${SupGIT_INSTALL_DIR:-${HOME}/.local/bin}"
TARGET_PATH="$INSTALL_DIR/supgit"

status() {
    printf "\r\033[K%s" "$1"
}

if [ ! -e "$TARGET_PATH" ]; then
    printf "\r\033[KSupGIT is not installed at %s\n" "$TARGET_PATH"
    exit 1
fi

status "Uninstalling SupGIT..."
rm -f "$TARGET_PATH"

printf "\r\033[K👋 SupGIT has been uninstalled 👋\n"
