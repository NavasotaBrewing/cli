#![allow(non_snake_case)]

const HELP_PAGE: &'static str = include_str!("help_page");

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use brewdrivers::{omega::{Degree, CN7500}, relays::BoardError};
use brewdrivers::{
    modbus::ModbusError,
    relays::{Waveshare, State}
};

macro_rules! prompt {
    ($type:ty) => {
        match prompt().parse::<$type>() {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("Couldn't get value of the correct type, try again. ({})", e);
                None
            }
        }
    };
}

fn prompt() -> String {
    print!("=>  ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: couldn't read line: {}", e);
        }
    }

    buffer.trim().to_owned()
}

#[derive(Debug)]
struct CN7500Config {
    port: String,
    addr: u16,
    baudrate: u32,
}

impl CN7500Config {
    // Connects to the device
    pub async fn connect(&self) -> Result<CN7500, ModbusError> {
        CN7500::new(self.addr as u8, &self.port, self.baudrate).await
    }

    pub async fn try_connect(&self) {
        match self.connect().await {
            Ok(mut instr) => {
                if instr.is_running().await.is_ok() {
                    println!("CN7500 connected");
                } else {
                    println!("The serial port was opened, but the CN7500 is not responding");
                }
            },
            Err(e) => {
                eprintln!("Cannot connect to CN7500. Error: {}", e);
                eprintln!("Config:\n{:#?}", self);
            }
        }
    }
}

#[derive(Debug)]
struct WaveshareConfig {
    port: String,
    addr: u8
}

impl WaveshareConfig {
    // Connects to the board
    pub fn connect(&self) -> Result<Waveshare, BoardError> {
        Waveshare::connect(self.addr, &self.port)
    }

    // Tries to connect to the board, printing the results
    pub fn try_connect(&self) {
        match self.connect() {
            Ok(mut ws) => {
                if ws.software_revision().is_ok() {
                    println!("Waveshare connected");
                } else {
                    println!("Serial port opened, but the Waveshare board didn't respond. Do you have the address correct?\n{:#?}", self);
                }
            }
            Err(_) => {
                eprintln!("Error: Couldn't connect to Waveshare with config:\n{:#?}", self);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut waveshare_config = WaveshareConfig {
        port: String::from("/dev/ttyAMA0"),
        addr: 0xFE
    };

    let mut cn7500_config = CN7500Config {
        port: String::from("/dev/ttyAMA0"),
        addr: 0x16,
        baudrate: 9600,
    };

    // we connect here just to print the status messages to see if
    // they connect or not.
    waveshare_config.try_connect();

    cn7500_config.try_connect().await;

    loop {
        let input = match prompt!(String) {
            Some(value) => value,
            _ => continue,
        };

        let args = input.split(' ').collect::<Vec<&str>>();
        if args.get(0).is_none() {
            eprintln!("Provide at least one command");
            continue;
        }
        let cmd = args.get(0).unwrap().to_lowercase();
        #[allow(unused_variables)]
        let (arg1, arg2, arg3) = (args.get(1), args.get(2), args.get(3));

        // No arg commands
        if arg1.is_none() {
            // Basic commands, no args
            match cmd.as_str() {
                "quit" => exit(0),
                "help" => {
                    println!("NBC CLI {}", env!("CARGO_PKG_VERSION"));
                    println!("{}", HELP_PAGE);
                }
                "config" => {
                    println!("{:#?}", waveshare_config);
                    println!("{:#?}", cn7500_config);
                }
                _ => {}
            }
        }

        // Regular command groups
        // Waveshare Config group
        match cmd.as_str() {
            "waveshare.port" => {
                if let Some(port) = arg1 {
                    waveshare_config.port = String::from(*port);
                    waveshare_config.try_connect();
                }
                continue;
            }
            "waveshare.addr" => {
                if let Some(addr_arg) = arg1 {
                    match addr_arg.parse::<u8>() {
                        Ok(addr) => {
                            waveshare_config.addr = addr;
                            println!("Address set to {}", waveshare_config.addr);
                            waveshare_config.try_connect();
                        }
                        Err(e) => {
                            eprintln!("Couldn't parse addr from '{}'", addr_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                continue;
            }
            "waveshare.config" => {
                println!("{:#?}", waveshare_config);
                continue;
            }
            _ => {}
        }

        // CN7500 Config group
        match cmd.as_str() {
            "cn7500.port" => {
                if let Some(port) = arg1 {
                    cn7500_config.port = String::from(*port);
                    cn7500_config.try_connect().await;
                }
                continue;
            }
            "cn7500.baudrate" => {
                if let Some(baud_arg) = arg1 {
                    match baud_arg.parse::<u32>() {
                        Ok(baud) => {
                            cn7500_config.baudrate = baud;
                            println!("Baudrate changed to {}", cn7500_config.baudrate);
                            cn7500_config.try_connect().await;
                        }
                        Err(e) => {
                            eprintln!("Couldn't parse baudrate, found {}", baud_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                continue;
            }
            "cn7500.addr" => {
                if let Some(addr_arg) = arg1 {
                    match addr_arg.parse::<u16>() {
                        Ok(addr) => {
                            cn7500_config.addr = addr;
                            println!("Address set to {}", cn7500_config.addr);
                            cn7500_config.try_connect().await;
                        }
                        Err(e) => {
                            eprintln!("Couldn't parse addr from '{}'", addr_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                continue;
            }
            "cn7500.config" => {
                println!("{:#?}", cn7500_config);
                continue;
            }
            _ => {}
        }

        // Waveshare actions
        match cmd.as_str() {
            "waveshare.connected" => {
                // Is Waveshare running
                waveshare_config.try_connect();
                continue;
            }
            "waveshare.relay" => {
                // Connect to the board

                let mut ws = match waveshare_config.connect() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                if let Some(relay_num_arg) = arg1 {
                    // Get our relay number
                    let relay_num = match relay_num_arg.parse::<u8>() {
                        Ok(relay_num) => relay_num,
                        Err(e) => {
                            eprintln!("Couldn't parse relay number from '{}'", relay_num_arg);
                            eprintln!("Error: {}", e);
                            continue;
                        }
                    };

                    // If we need to update the relay, update it
                    if let Some(state_arg) = arg2 {
                        let state = match state_arg.to_lowercase().trim() {
                            "1" | "on" => State::On,
                            _ => State::Off,
                        };

                        ws.set_relay(relay_num, state).unwrap();
                    }

                    // Afterwards, always print it
                    match ws.get_relay(relay_num) {
                        Ok(state) => println!("Relay {}: {}", relay_num, state),
                        Err(board_error) => eprintln!("{:?}", board_error)
                    }
                    
                } else {
                    println!("Provide a relay number (0-255)");
                }
                continue;
            }
            "waveshare.set_cn" => {
                if arg1.is_none() {
                    eprintln!("Provide a new controller number (0-255)");
                    continue;
                }

                match arg1.unwrap().parse::<u8>() {
                    Ok(new_cn) => {
                        let mut ws = match waveshare_config.connect() {
                            Ok(s) => s,
                            Err(_) => continue,
                        };
                        ws.set_address(new_cn).unwrap();
                        println!("controller number set to {}", new_cn);
                    }
                    Err(e) => {
                        eprintln!(
                            "Invalid controller number '{}', should be 0-255",
                            arg1.unwrap()
                        );
                        eprintln!("Error: {}", e);
                    }
                }
                continue;
            }
            "waveshare.all_relays" => {
                // List all relays
                let mut ws = match waveshare_config.connect() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                match ws.get_all_relays() {
                    Ok(states) => {
                        for (i, state) in states.iter().enumerate() {
                            println!("Relay {}: {}", i, state);
                        }
                    },
                    Err(e) => {
                        eprintln!("Something went wrong, couldn't get relay states: {}", e)
                    }
                }
            }
            _ => {}
        }

        // CN7500 actions
        match cmd.as_str() {
            "cn7500.connected" => {
                // Is CN7500 connected
                cn7500_config.try_connect().await;
                continue;
            }
            "cn7500.pv" => {
                // Get PV
                let mut cn = match cn7500_config.connect().await {
                    Ok(instr) => instr,
                    Err(_) => continue
                };
                println!("pv: {:?}", cn.get_pv().await);
                continue;
            }
            "cn7500.sv" => {
                let mut cn = match cn7500_config.connect().await {
                    Ok(instr) => instr,
                    Err(_) => continue
                };
                // If there's no arg, print the sv and leave
                if args.len() < 2 {
                    println!("sv: {:?}", cn.get_sv().await);
                    continue;
                }

                // otherwise, set the sv
                match args[1].parse::<f64>() {
                    Ok(new_sv) => eprintln!("Setting SV: {:?}", cn.set_sv(new_sv).await),
                    Err(e) => {
                        eprintln!("Couldn't parse new sv (f64) from arg '{}'", arg1.unwrap());
                        eprintln!("{}", e);
                    }
                }
                continue;
            }
            "cn7500.run" => {
                // Run the cn7500
                let mut cn = match cn7500_config.connect().await {
                    Ok(instr) => instr,
                    Err(_) => continue
                };
                println!("CN7500 is running: {:?}", cn.run().await);
                continue;
            }
            "cn7500.stop" => {
                // Stop the cn7500
                let mut cn = match cn7500_config.connect().await {
                    Ok(instr) => instr,
                    Err(_) => continue
                };
                println!("CN7500 stopped: {:?}", cn.stop().await);
                continue;
            }
            "cn7500.set_units" => {
                if args.len() < 2 {
                    println!("Not enough args. Should be F or C");
                    continue;
                }

                let mut cn = match cn7500_config.connect().await {
                    Ok(instr) => instr,
                    Err(_) => continue
                };

                match arg1.unwrap().to_uppercase().as_str() {
                    "F" => {
                        // Set to F
                        println!(
                            "CN7500 set to Fahrenheit: {:?}",
                            cn.set_degrees(Degree::Fahrenheit).await
                        );
                    }
                    "C" => {
                        // Set to C
                        println!(
                            "CN7500 set to Celsius: {:?}",
                            cn.set_degrees(Degree::Celsius).await
                        );
                    }
                    _ => {
                        println!("Invalid arg '{}', should be C or F.", arg1.unwrap());
                    }
                }
                continue;
            }
            _ => {}
        }
    }
}
