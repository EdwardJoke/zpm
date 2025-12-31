# zpm
A fast, lightweight Zig version manager written in Rust.

## Demo
![zpm demo](.assets/demo.gif)

## Features
- Install multiple Zig versions (latest, master, stable, or specific versions)
- Set a default Zig version
- Uninstall unwanted Zig versions
- List installed Zig versions
- Install ZLS (Zig Language Server) for the current Zig version
- Fast and efficient downloads
- Lightweight and minimal dependencies

## Installation
```bash
# Install from source
cargo install --path .

# Or install from crates.io (once published)
cargo install zpm
```

## Usage

### Install a Zig version
```bash
# Install the latest version
zpm install
zpm i

# Install a specific version
zpm install 0.13.0
zpm i 0.13.0

# Install and set as default
zpm install --default 0.13.0
zpm i -d 0.13.0

# Install master branch
zpm install master

# Install stable version
zpm install stable
```

### Set default Zig version
```bash
zpm use 0.13.0
```

### Uninstall a Zig version
```bash
zpm uninstall 0.13.0
zpm rm 0.13.0
```

### List installed versions
```bash
zpm list
zpm ls
```

### Install ZLS for current version
```bash
zpm install-zls
```

## Requirements
- Rust 1.70+ (for building)
- Zig (for usage)
- Internet connection (for downloading Zig versions)

## License
Apache 2.0
