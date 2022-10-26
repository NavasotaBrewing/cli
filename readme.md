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
will look at that path for a configuration file.

See [the RTU Configuration guide](https://github.com/NavasotaBrewing/documentation/blob/master/RTU_Configuration/configuration.md) for what goes in the configuration file. [This link shows](https://github.com/NavasotaBrewing/documentation/blob/master/RTU_Configuration/rtu_conf.yaml) all the possible keys that go into a configuration file, and you can copy/paste and edit that if you want.

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
╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗
║                                              Waveshare Commands                                              ║
╠═════════════════════════════╦════════════════════════════════════════════════════════════════════════════════╣
║           Command           ║                                      Help                                      ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID]                   ║ Gets a relay status                                                            ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] list_all          ║ Lists states of this and all the neighboring relays on this controller         ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] [On|Off]          ║ Turns a relay on or off                                                        ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] set_all [On|Off]  ║ Sets this and all the neighboring relays on this controller                    ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] get_cn            ║ Attempts to find the controller number the board is set to. The configured con ║
║                             ║ troller number (from the conf file) doesn't matter                             ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] set_cn [0-254]    ║ Sets a new controller number for this controller. You'll need to update your r ║
║                             ║ tu_conf.yaml file. Don't forget the controller number                          ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] software_revision ║ Lists the software revision currently on the board                             ║
╠═════════════════════════════╩════════════════════════════════════════════════════════════════════════════════╣
║                                                 STR1 Commands                                                ║
╠═════════════════════════════╦════════════════════════════════════════════════════════════════════════════════╣
║           Command           ║                                      Help                                      ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID]                   ║ Gets a relay status                                                            ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] list_all          ║ Lists states of all the neighboring relays on this controller                  ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] [On|Off]          ║ Turns a relay on or off                                                        ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [relayID] set_cn [0-254]    ║ Sets a new controller number for this controller. You'll need to update your r ║
║                             ║ tu_conf.yaml file. Don't forget the controller number                          ║
╠═════════════════════════════╩════════════════════════════════════════════════════════════════════════════════╣
║                                                CN7500 Commands                                               ║
╠═════════════════════════════╦════════════════════════════════════════════════════════════════════════════════╣
║       CN7500 Commands       ║                                      Help                                      ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID]                  ║ Gets the PV, SV, and status of the relay                                       ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] pv               ║ Gets the Process Value (actual)                                                ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] sv               ║ Gets the Setpoint Value (target)                                               ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] set [#.#]        ║ Sets the SV. Use a decimal number                                              ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] is_running       ║ Returns the status of the relay                                                ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] run              ║ Turns the relay on                                                             ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] stop             ║ Turns the relay off                                                            ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] degrees [F|C]    ║ Sets degree units to F or C                                                    ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ [deviceID] watch            ║ Prints the PV and SV every few seconds until you quit                          ║
╚═════════════════════════════╩════════════════════════════════════════════════════════════════════════════════╝
```
