# CLI
This package provides a CLI for interacting with the Navasota Brewing Company's brewing hardware. It is an interface built over the [NavasotaBrewing/brewdrivers](https://github.com/NavasotaBrewing/brewdrivers) library.

This repository only contains code and documentation for the CLI. If you're looking for another part of the BCS or for hardware information, look at the [organization readme](https://github.com/NavasotaBrewing/readme).

# Installation
If you have `cargo` installed:

```
$ cargo install NBC_cli
```

or you can clone this repository and build from source with `cargo build`.

## Cross Compiling
NBC provides a Docker image for cross compiling. We use this CLI on our `armv7` RTUs (Raspberry Pi 3B), which don't compile Rust very well. We use `cross` to cross compile, then send the binary over to the RTU.

This repo has a `Cross.toml`, and all the configuration is taken care of already. Be sure you have docker installed, then clone this repo and run
```
$ cross build --target armv7-unknown-linux-gnueabihf
```

Executables will be put in `target/armv7-unknown-linux-gnueabihf`.

# Usage
See the [help page](src/help_page) or type `help` in the cli for usage instructions.
