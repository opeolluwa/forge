# Forge

Forge is a lightweight extensible, command line toolchain to streamline a
developer workflow

## Installation

### Quick install (recommended)

```sh
curl -sSL https://raw.githubusercontent.com/opeolluwa/x/master/install.sh | bash
```

This downloads the latest prebuilt binary for your platform and installs it to `~/.local/bin`.

### Install from source

```sh
git clone https://github.com/opeolluwa/x.git
cd x
cargo build --release
cargo install --path .
```

### Usage

The command is `forge`

### Configuration

- Config path : `$HOME/.config/forge/default-config.toml`
