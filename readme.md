# CLI
This is a CLI that utilizes the [`NavasotaBrewing/brewdrivers`](https://github.com/NavasotaBrewing/brewdrivers) library. It allows for manual control of the brewery control system hardware.

See the [help page](./src/help_page) for a list of commands.

# Installation
If you have `cargo` installed:

```
$ cargo install NBC_cli
```

Otherwise, see the [releases](https://github.com/NavasotaBrewing/cli/releases) section for precompiled binaries.

## Cross Compiling
There's a `Dockerfile` here that will let you cross compile. Build the `Docker` image, then cross compile with the `cross` Rust tool

```
$ docker build -t mytag/name:version .
$ cross build --target armv7-unknown-linux-gnueabihf
```

# Usage
See the [help page](src/help_page) or type `help` in the cli for usage instructions.


