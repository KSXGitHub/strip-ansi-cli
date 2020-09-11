# strip-ansi-cli

[![Test](https://github.com/KSXGitHub/strip-ansi-cli/workflows/Test/badge.svg)](https://github.com/KSXGitHub/strip-ansi-cli/actions?query=workflow%3ATest)
[![Travis Build Status](https://img.shields.io/travis/KSXGitHub/strip-ansi-cli/master?label=build&logo=travis)](https://travis-ci.org/KSXGitHub/strip-ansi-cli)
[![Crates.io Version](https://img.shields.io/crates/v/strip-ansi-cli?logo=rust)](https://crates.io/crates/strip-ansi-cli)

Strip ANSI escape sequences from text.

## Usage

### Stdin as input

```sh
ls --color=always | strip-ansi
```

### Argument as input

```sh
strip-ansi "$(ls --color=always)"
```

### Print help message

```sh
strip-ansi --help
```

## Installation

### Manually build from source

1. Make sure to have `cargo` and `rustc` installed. If not, you may install it using [rustup](https://rustup.rs/).

2. Clone this repository.

3. Run `cargo build --release --locked` in the repo root.

4. The binary should be created at `$REPO_DIR/target/release/strip-ansi`.

### Manually download prebuilt binary

Go to [the Release Page](https://github.com/KSXGitHub/strip-ansi-cli/releases) and find file whose name contains `strip-ansi` and your platform name.

### From [Crates.io](https://crates.io/crates/strip-ansi-cli)

```sh
cargo install strip-ansi-cli
```

### From the [Arch User Repository](https://aur.archlinux.org)

#### [Build from source](https://aur.archlinux.org/packages/strip-ansi)

```sh
yay -S strip-ansi
```

#### [Prebuilt binary](https://aur.archlinux.org/packages/strip-ansi-bin)

```sh
yay -Ss strip-ansi-bin
```

## License

[MIT](https://git.io/JUWVF) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
