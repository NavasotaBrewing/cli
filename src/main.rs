#![allow(non_snake_case)]

const HELP_PAGE: &'static str = include_str!("help_page");

use std::{
    io::{Write, stdout, stdin},
    process::exit,
    println,
    unimplemented
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
    print!(">  ");
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

#[derive(Debug)]
struct STR1Config {
    port: String,
    addr: u8,
    baudrate: u32
}

fn main() {
    println!("Brewdrivers {}", env!("CARGO_PKG_VERSION"));

    let mut str1_config = STR1Config {
        port: String::from("/dev/ttyUSB0"),
        addr: 0x01,
        baudrate: 9600
    };

    let mut cn7500_config = CN7500Config {
        port: String::from("/dev/ttyUSB0"),
        addr: 0x16,
        baudrate: 9600
    };

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
                "help" => println!("{}", HELP_PAGE),
                "quit" => exit(0),
                "config" => {
                    println!("{:#?}", str1_config);
                    println!("{:#?}", cn7500_config);
                }
                _ => {}
            }
            continue;
        }


        // Regular command groups
        // STR1 Config group
        match args[0].to_lowercase().as_str() {
            "str1.port" => {
                str1_config.port = String::from(args[1]);
                println!("{:#?}", str1_config);
                continue;
            },
            "str1.baudrate" => {
                if let Ok(baud) = args[1].parse::<u32>() {
                    str1_config.baudrate = baud;
                    println!("{:#?}", str1_config);
                }
                continue;
            },
            "str1.addr" => {
                if let Ok(addr) = args[1].parse::<u8>() {
                    str1_config.addr = addr;
                    println!("{:#?}", str1_config);
                }
                continue;
            }
            _ => {}
        }

        // CN7500 Config group
        match args[0].to_lowercase().as_str() {
            "cn7500.port" => {
                cn7500_config.port = String::from(args[1]);
                println!("{:#?}", cn7500_config);
                continue;
            },
            "cn7500.baudrate" => {
                if let Ok(baud) = args[1].parse::<u32>() {
                    cn7500_config.baudrate = baud;
                    println!("{:#?}", cn7500_config);
                }
                continue;
            },
            "cn7500.addr" => {
                if let Ok(addr) = args[1].parse::<u16>() {
                    cn7500_config.addr = addr;
                    println!("{:#?}", cn7500_config);
                }
                continue;
            }
            _ => {}
        }

        // STR1 actions
        match args[0] {
            "str1.relay" => {
                if args.len() == 3 {
                    // Set relay
                    println!("Relay {} set to {}", args[1], "On");
                } else if args.len() == 2 {
                    // Get relay
                    println!("Relay {} is {}", args[1], "On");
                }
                continue;
            },
            "str1.set_cn" => {
                if let Ok(new_cn) = args[1].parse::<u8>() {
                    // Set controller number
                    println!("controller number set to {}", new_cn);
                } else {
                    println!("Invalid controller number, should be 0-255");
                }
                continue;
            }
            _ => {}
        }

        // CN7500 actions
        match args[0] {
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
            }
            _ => {}
        }
    }
}
