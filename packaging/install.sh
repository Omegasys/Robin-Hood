#!/usr/bin/env bash

set -euo pipefail

PROGRAM_NAME="redrobin"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PREFIX="${PREFIX:-/usr/local}"
BINDIR="${PREFIX}/bin"
MANDIR="${PREFIX}/share/man/man1"
DATADIR="${PREFIX}/share/applications"

BINARY="${PROJECT_ROOT}/target/release/${PROGRAM_NAME}"
MANPAGE="${PROJECT_ROOT}/packaging/redrobin.1"
DESKTOP_FILE="${PROJECT_ROOT}/packaging/redrobin.desktop"

echo "Red Robin installer"
echo "==================="

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Cargo is not installed."
    echo "Install Rust and Cargo before installing Red Robin."
    exit 1
fi

echo
echo "Building Red Robin in release mode..."

cd "${PROJECT_ROOT}"

cargo build --release

if [[ ! -f "${BINARY}" ]]; then
    echo "Error: Release binary was not created."
    exit 1
fi

echo
echo "Installing binary..."

install -Dm755 \
    "${BINARY}" \
    "${BINDIR}/${PROGRAM_NAME}"

echo "Installing manual page..."

install -Dm644 \
    "${MANPAGE}" \
    "${MANDIR}/${PROGRAM_NAME}.1"

echo "Installing desktop entry..."

install -Dm644 \
    "${DESKTOP_FILE}" \
    "${DATADIR}/${PROGRAM_NAME}.desktop"

echo
echo "Red Robin installed successfully."

echo
echo "Binary:"
echo "  ${BINDIR}/${PROGRAM_NAME}"

echo
echo "Manual:"
echo "  ${MANDIR}/${PROGRAM_NAME}.1"

echo
echo "Try:"
echo "  redrobin --help"
echo "  redrobin"