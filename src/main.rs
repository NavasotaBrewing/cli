#![allow(non_snake_case)]

const HELP_PAGE: &'static str = include_str!("help_page");

use std::{
    io::{Write, stdout, stdin},
    process::exit
};

use brewdrivers::relays::{STR1, State};
use brewdrivers::omega::{CN7500, Degree};

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
        Ok(_) => {},
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
    baudrate: u32
}

impl CN7500Config {
    // Connects to the device
    pub async fn connect(&self) -> CN7500 {
        let mut cn = CN7500::new(
            self.addr as u8,
            &self.port,
            self.baudrate
        ).await;

        cn
    }
}

#[derive(Debug)]
struct STR1Config {
    port: String,
    addr: u8,
    baudrate: u32
}

impl STR1Config {
    // Connects to the board
    pub fn connect(&self) -> STR1 {
        let mut str1 = STR1::new(
            self.addr,
            &self.port,
            self.baudrate
        ).expect(
            &format!("Error opening serial port {}. Try changing the STR1 config.", self.port)
        );

        str1
    }
}




#[tokio::main]
async fn main() {

    let mut str1_config = STR1Config {
        port: String::from("/dev/ttyUSB0"),
        addr: 0xFE,
        baudrate: 9600
    };

    let mut cn7500_config = CN7500Config {
        port: String::from("/dev/ttyUSB0"),
        addr: 0x16,
        baudrate: 19200
    };

    // we connect here just to print the status messages to see if
    // they connect or not.
    if str1_config.connect().connected() {
        println!("STR1 connected successfully.");
    } else {
        println!("Could not connect to STR1 with config:");
        println!("{:#?}", str1_config);
    }

    if cn7500_config
        .connect()
        .await
        .is_running()
        .await
        .is_ok()
    {
        println!("CN7500 connected");
    } else {
        println!("Could not connect to CN7500 with config:");
        println!("{:#?}", cn7500_config);
    }
    
    loop {


        let input = match prompt!(String) {
            Some(value) => value,
            _ => continue,
        };

        let args = input.split(' ').collect::<Vec<&str>>();

        // No arg commands
        if args.len() == 1 {
            // Basic commands, no args
            match args[0] {
                "help" => {
                    println!("Brewdrivers {}", env!("CARGO_PKG_VERSION"));
                    println!("{}", HELP_PAGE);
                },
                "quit" => exit(0),
                "config" => {
                    println!("{:#?}", str1_config);
                    println!("{:#?}", cn7500_config);
                }
                _ => {}
            }
        }


        // Regular command groups
        // STR1 Config group
        match args[0].to_lowercase().as_str() {
            "str1.port" => {
                str1_config.port = String::from(args[1]);
                println!("STR1 connected: {}", str1_config.connect().connected());
                continue;
            },
            "str1.baudrate" => {
                if let Ok(baud) = args[1].parse::<u32>() {
                    str1_config.baudrate = baud;
                    println!("STR1 connected: {}", str1_config.connect().connected());
                }
                continue;
            },
            "str1.addr" => {
                if let Ok(addr) = args[1].parse::<u8>() {
                    str1_config.addr = addr;
                    println!("STR1 connected: {}", str1_config.connect().connected());
                }
                continue;
            },
            "str1.config" => {
                println!("{:#?}", str1_config);
                continue;
            }
            _ => {}
        }

        // CN7500 Config group
        match args[0].to_lowercase().as_str() {
            "cn7500.port" => {
                cn7500_config.port = String::from(args[1]);
                println!("CN7500 connected: {}", cn7500_config
                    .connect()
                    .await
                    .is_running()
                    .await
                    .is_ok());
                continue;
            },
            "cn7500.baudrate" => {
                if let Ok(baud) = args[1].parse::<u32>() {
                    cn7500_config.baudrate = baud;
                    println!("CN7500 connected: {}", cn7500_config
                        .connect()
                        .await
                        .is_running()
                        .await
                        .is_ok());
                }
                continue;
            },
            "cn7500.addr" => {
                if let Ok(addr) = args[1].parse::<u16>() {
                    cn7500_config.addr = addr;
                    println!("CN7500 connected: {}", cn7500_config
                        .connect()
                        .await
                        .is_running()
                        .await
                        .is_ok());
                }
                continue;
            },
            "cn7500.config" => {
                println!("{:#?}", cn7500_config);
                continue;
            }
            _ => {}
        }

        // STR1 actions
        match args[0] {
            "str1.connected" => {
                // Is STR1 running
                println!("STR1 connected: {}", str1_config.connect().connected());
                continue;
            },
            "str1.relay" => {
                // Connect to the board
                let mut str1 = str1_config.connect();

                // This will actually always fire because we handle single args above,
                // and continue in each case.
                if let Some(relay_num_arg) = args.get(1) {
                    // Get our relay number
                    let relay_num = relay_num_arg.parse::<u8>().expect(
                        &format!("Invalid relay number, need 0-255, found {}", args[1])
                    );

                    // If we need to update the relay, update it
                    if let Some(state_arg) = args.get(2) {
                        let state = match state_arg.to_lowercase().trim() {
                            "1" | "on" => State::On,
                            _ => State::Off
                        };

                        str1.set_relay(relay_num, state);
                    }

                    // Afterwords, always print it
                    println!("Relay {}: {}", relay_num, str1.get_relay(relay_num));
                } else {
                    println!("Provide a relay number (0-255)");
                }
                continue;
            },
            "str1.set_cn" => {
                if let Ok(new_cn) = args[1].parse::<u8>() {
                    // Set controller number
                    let mut str1 = str1_config.connect();
                    str1.set_controller_num(new_cn);
                    println!("controller number set to {}", new_cn);
                } else {
                    println!("Invalid controller number, should be 0-255");
                }
                continue;
            },
            "str1.all_relays" => {
                // List all relays
                let mut str1 = str1_config.connect();
                str1.list_all_relays();
            }
            _ => {}
        }

        // CN7500 actions
        match args[0] {
            "cn7500.connected" => {
                // Is CN7500 connected
                println!("CN7500 connected");
                continue;
            },
            "cn7500.pv" => {
                // Get PV
                println!("{}", "118.3");
                continue;
            },
            "cn7500.sv" => {
                if args.len() == 2 {
                    // Set SV
                    println!("SV set to {}", args[1]);
                } else {
                    // Get SV
                    println!("SV: {}", "150.0");
                }
                continue;
            },
            "cn7500.run" => {
                // Run the cn7500
                println!("CN7500 is running...");
                continue;
            },
            "cn7500.stop" => {
                // Stop the cn7500
                println!("CN7500 stopped.");
                continue;
            },
            "cn7500.set_units" => {
                if args.len() < 2 {
                    println!("Not enough args. Should be F or C");
                    continue;
                }

                match args[1].to_uppercase().as_str() {
                    "F" => {
                        // Set to F
                        println!("CN7500 set to Fahrenheit");
                    },
                    "C" => {
                        // Set to C
                        println!("CN7500 set to Celsius");
                    }
                    _ => {}
                }
                continue;
            }
            _ => {}
        }


    }
}
