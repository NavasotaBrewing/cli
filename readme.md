# CLI
This package provides a CLI for interacting with the Navasota Brewing Company's brewing hardware. It is an interface built over the [NavasotaBrewing/brewdrivers](https://github.com/NavasotaBrewing/brewdrivers) library.

This repository only contains code and documentation for the CLI. If you're looking for another part of the BCS or for hardware information, look at the [organization docs](https://github.com/NavasotaBrewing/readme).

# Installation
If you have `cargo` installed:

```
$ cargo install NBC_cli
```

This will install the executable `~/.cargo/bin/NBC_cli` that you can run. If `~/.cargo/bin` is in your PATH, you can run it from anywhere with `NBC_cli`.

Alternatively, you can clone this repository and build from source with `cargo run --release`.

# Usage
## Configuration File
Before starting, you should be sure you have a configuration file for the RTU. This should go in `/etc/NavasotaBrewing/rtu_conf.yaml`. The CLI
will look at that path for a configuration file, but you can provide another path when starting the CLI to override that.

See [this documentation page](https://github.com/NavasotaBrewing/documentation/blob/master/config_file.md) for what goes in the configuration file.

---------------------

The CLI contains help pages and command lists. You can access them like this:

```
🍺 ==> help         # see the help page
🍺 ==> commands     # see the command tables
🍺 ==> devices      # see the list of connected devices
```

For reference, the same command tables are listed below

## Command Tables

```
╔════════════════════════════════════════════════════════════════════════════════════════╗
║                                      STR1 Commands                                     ║
╠═══════════════════════════╦════════════════════════════════════════════════════════════╣
║          Command          ║                            Help                            ║
╠═══════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID]                 ║ Gets a relay status                                        ║
╠═══════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] list_all        ║ Lists states of all the neighboring relays on this control ║
║                           ║ ler                                                        ║
╠═══════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] [On|Off|1|0]    ║ Turns a relay on or off                                    ║
╠═══════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] set_cn [0-254]  ║ Sets a new controller number for this controller. You'll n ║
║                           ║ eed to update your rtu_conf.yaml file. Don't forget the co ║
║                           ║ ntroller number                                            ║
╚═══════════════════════════╩════════════════════════════════════════════════════════════╝

╔══════════════════════════════════════════════════════════════════════════════════════════════╗
║                                      Waveshare Commands                                      ║
╠═════════════════════════════════╦════════════════════════════════════════════════════════════╣
║             Command             ║                            Help                            ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID]                       ║ Gets a relay status                                        ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] list_all              ║ Lists states of this and all the neighboring relays on thi ║
║                                 ║ s controller                                               ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] [On|Off|1|0]          ║ Turns a relay on or off                                    ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] set_all [On|Off|1|0]  ║ Sets this and all the neighboring relays on this controlle ║
║                                 ║ r                                                          ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] get_cn                ║ Attempts to find the controller number the board is set to ║
║                                 ║ . The configured controller number (from the conf file) do ║
║                                 ║ esn't matter                                               ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] set_cn [0-254]        ║ Sets a new controller number for this controller. You'll n ║
║                                 ║ eed to update your rtu_conf.yaml file. Don't forget the co ║
║                                 ║ ntroller number                                            ║
╠═════════════════════════════════╬════════════════════════════════════════════════════════════╣
║ [relayID] software_revision     ║ Lists the software revision currently on the board         ║
╚═════════════════════════════════╩════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════════╗
║                                  CN7500 Commands                                  ║
╠═══════════════════════════╦═══════════════════════════════════════════════════════╣
║      CN7500 Commands      ║                          Help                         ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID]                ║ Gets the PV, SV, and status of the relay              ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] pv             ║ Gets the Process Value (actual)                       ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] sv             ║ Gets the Setpoint Value (target)                      ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] set [#.#]      ║ Sets the SV. Use a decimal number                     ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] is_running     ║ Returns the status of the relay                       ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] run            ║ Turns the relay on                                    ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] stop           ║ Turns the relay off                                   ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] degrees [F|C]  ║ Sets degree units to F or C                           ║
╠═══════════════════════════╬═══════════════════════════════════════════════════════╣
║ [deviceID] watch          ║ Prints the PV and SV every few seconds until you quit ║
╚═══════════════════════════╩═══════════════════════════════════════════════════════╝
```
