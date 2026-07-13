#!/bin/bash
set -euo pipefail

REPO="opeolluwa/x"
BINARY="forge"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

detect_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux*)  os="linux" ;;
    Darwin*) os="darwin" ;;
    MINGW*|MSYS*|CYGWIN*) os="windows" ;;
    *) echo "Error: unsupported OS: $os" >&2; exit 1 ;;
  esac

  case "$arch" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *) echo "Error: unsupported architecture: $arch" >&2; exit 1 ;;
  esac

  echo "${os}-${arch}"
}

main() {
  local platform tag url ext

  platform="$(detect_platform)"
  tag="$(curl -sSL -o /dev/null -w '%{url_effective}' "https://github.com/${REPO}/releases/latest" | sed 's|.*/||')"

  if [ "$platform" = "windows-x86_64" ]; then
    ext="zip"
  else
    ext="tar.gz"
  fi

  url="https://github.com/${REPO}/releases/download/${tag}/${BINARY}-${platform}.${ext}"
  echo "Downloading ${url}..."

  local tmp_dir
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT

  curl -sSL "$url" -o "${tmp_dir}/${BINARY}.${ext}"

  mkdir -p "$INSTALL_DIR"

  if [ "$ext" = "zip" ]; then
    unzip -o "${tmp_dir}/${BINARY}.${ext}" -d "$tmp_dir"
    mv "${tmp_dir}/${BINARY}.exe" "${INSTALL_DIR}/${BINARY}.exe"
  else
    tar xzf "${tmp_dir}/${BINARY}.${ext}" -C "$tmp_dir"
    mv "${tmp_dir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
    chmod +x "${INSTALL_DIR}/${BINARY}"
  fi

  echo "Installed ${BINARY} to ${INSTALL_DIR}/${BINARY}"

  if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    echo ""
    echo "NOTE: ${INSTALL_DIR} is not in your PATH."
    echo "Add this to your shell profile:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
  fi
}

main "$@"
