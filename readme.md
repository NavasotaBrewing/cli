# CLI
This package provides a CLI for interacting with the Navasota Brewing Company's brewing hardware. It is an interface built over the [NavasotaBrewing/brewdrivers](https://github.com/NavasotaBrewing/brewdrivers) library.

This repository only contains code and documentation for the CLI. If you're looking for another part of the BCS or for hardware information, look at the [organization docs](https://github.com/NavasotaBrewing/readme).

<!-- TODO: rename this crate? -->
Technically, this isn't a command line interface (CLI), it's a terminal user interface (TUI) but I named it CLI a long time ago and I don't want to rename it.

# Installation
**note**: if you use the build scripts to build an RTU automatically, this should be installed for you. Check by running `NBC_cli`.

--------------------------

Install with `cargo`

```
$ cargo install NBC_cli     # Install
$ NBC_cli                   # Start
```

Alternatively, you can clone this repository and build from source with `cargo run --release`.

# Usage
## Configuration File
Before starting, you should be sure you have a configuration file for the RTU. See [this documentation page](https://github.com/NavasotaBrewing/documentation/blob/master/RTU_Configuration/configuration.md) on writing a configuration file.

Note: the configuration file will be validated when launching the CLI. If there are any errors, the CLI won't start up. This is a useful way to check your configuration file for errors.

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
║                                               General Commands                                               ║
╠═════════════════════════════╦════════════════════════════════════════════════════════════════════════════════╣
║           Command           ║                                      Help                                      ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ help                        ║ displays help information.                                                     ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ quit                        ║ quits the shell                                                                ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ exit                        ║ exits the shell                                                                ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ commands                    ║ lists the commands page (this page)                                            ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ devices                     ║ list all configured devices                                                    ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ time                        ║ prints the current time                                                        ║
╠═════════════════════════════╬════════════════════════════════════════════════════════════════════════════════╣
║ dashboard                   ║ view a dashboard of all device states                                          ║
╠═════════════════════════════╩════════════════════════════════════════════════════════════════════════════════╣
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
