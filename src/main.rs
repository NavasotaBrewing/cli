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
        let cn = CN7500::new(
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
        let str1 = STR1::new(
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
        if args.get(0).is_none() {
            eprintln!("Provide at least one command");
            continue;
        }
        let cmd = *args.get(0).unwrap();
        #[allow(unused_variables)]
        let (arg1, arg2, arg3) = (args.get(1), args.get(2), args.get(3));

        // No arg commands
        if arg1.is_none() {
            // Basic commands, no args
            match cmd {
                "quit" => exit(0),
                "help" => {
                    println!("Brewdrivers CLI {}", env!("CARGO_PKG_VERSION"));
                    println!("{}", HELP_PAGE);
                },
                "config" => {
                    println!("{:#?}", str1_config);
                    println!("{:#?}", cn7500_config);
                }
                _ => {}
            }
        }


        // Regular command groups
        // STR1 Config group
        match cmd.to_lowercase().as_str() {
            "str1.port" => {
                if let Some(port) = arg1 {
                    str1_config.port = String::from(*port);
                    println!("STR1 connected: {}", str1_config.connect().connected());
                }
                continue;
            },
            "str1.baudrate" => {
                if let Some(baud_arg) = arg1 {
                    match baud_arg.parse::<u32>() {
                        Ok(baud) => {
                            str1_config.baudrate = baud;
                            println!("Baudrate changed to {}", str1_config.baudrate);
                            println!("STR1 connected: {}", str1_config.connect().connected());
                        },
                        Err(e) => {
                            eprintln!("Couldn't parse baudrate, found {}", baud_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                continue;
            },
            "str1.addr" => {
                if let Some(addr_arg) = arg1 {
                    match addr_arg.parse::<u8>() {
                        Ok(addr) => {
                            str1_config.addr = addr;
                            println!("Address set to {}", str1_config.addr);
                            println!("STR1 connected: {}", str1_config.connect().connected());
                        },
                        Err(e) => {
                            eprintln!("Couldn't parse addr from '{}'", addr_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
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
                if let Some(port) = arg1 {
                    cn7500_config.port = String::from(*port);
                    println!("CN7500 connected: {}", cn7500_config
                    .connect()
                    .await
                    .is_running()
                    .await
                    .is_ok());
                }
                continue;
            },
            "cn7500.baudrate" => {
                if let Some(baud_arg) = arg1 {
                    match baud_arg.parse::<u32>() {
                        Ok(baud) => {
                            cn7500_config.baudrate = baud;
                            println!("Baudrate changed to {}", cn7500_config.baudrate);
                            println!("CN7500 connected: {}", cn7500_config
                                .connect()
                                .await
                                .is_running()
                                .await
                                .is_ok());
                        },
                        Err(e) => {
                            eprintln!("Couldn't parse baudrate, found {}", baud_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                continue;
            },
            "cn7500.addr" => {
                if let Some(addr_arg) = arg1 {
                    match addr_arg.parse::<u16>() {
                        Ok(addr) => {
                            cn7500_config.addr = addr;
                            println!("Address set to {}", cn7500_config.addr);
                            println!("CN7500 connected: {}", cn7500_config
                                .connect()
                                .await
                                .is_running()
                                .await
                                .is_ok());
                        },
                        Err(e) => {
                            eprintln!("Couldn't parse addr from '{}'", addr_arg);
                            eprintln!("Error: {}", e);
                        }
                    }
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
                            _ => State::Off
                        };

                        str1.set_relay(relay_num, state);
                    }

                    // Afterwards, always print it
                    println!("Relay {}: {}", relay_num, str1.get_relay(relay_num));
                } else {
                    println!("Provide a relay number (0-255)");
                }
                continue;
            },
            "str1.set_cn" => {
                if arg1.is_none() {
                    eprintln!("Provide a new controller number (0-255)");
                    continue;
                }

                match arg1.unwrap().parse::<u8>() {
                    Ok(new_cn) => {
                        let mut str1 = str1_config.connect();
                        str1.set_controller_num(new_cn);
                        println!("controller number set to {}", new_cn);
                    },
                    Err(e) => {
                        eprintln!("Invalid controller number '{}', should be 0-255", arg1.unwrap());
                        eprintln!("Error: {}", e);
                    }
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
                println!("CN7500 connected: {}", cn7500_config
                    .connect()
                    .await
                    .is_running()
                    .await
                    .is_ok()
                );
                continue;
            },
            "cn7500.pv" => {
                // Get PV
                let mut cn = cn7500_config.connect().await;
                println!("pv: {:?}", cn.get_pv().await);
                continue;
            },
            "cn7500.sv" => {
                let mut cn = cn7500_config.connect().await;
                // If there's no arg, print the sv and leave
                if args.len() < 2 {
                    println!("sv: {:?}", cn.get_sv().await);
                    continue;
                }

                // otherwise, set the sv
                match args[1].parse::<f64>() {
                    Ok(new_sv) => {
                        let result = cn.set_sv(new_sv).await;
                        if result.is_err() {
                            eprintln!("Error: {}", result.unwrap_err());
                        }
                    },
                    Err(e) => {
                        eprintln!("Couldn't parse new sv (f64) from arg '{}'", args[1]);
                        eprintln!("{}", e);
                    }
                }
                continue;
            },
            "cn7500.run" => {
                // Run the cn7500
                let mut cn = cn7500_config.connect().await;
                println!("CN7500 is running: {:?}", cn.run().await);
                continue;
            },
            "cn7500.stop" => {
                // Stop the cn7500
                let mut cn = cn7500_config.connect().await;
                println!("CN7500 stopped: {:?}", cn.stop().await);
                continue;
            },
            "cn7500.set_units" => {
                if args.len() < 2 {
                    println!("Not enough args. Should be F or C");
                    continue;
                }

                let mut cn = cn7500_config.connect().await;

                match args[1].to_uppercase().as_str() {
                    "F" => {
                        // Set to F
                        println!("CN7500 set to Fahrenheit: {:?}", cn.set_degrees(Degree::Fahrenheit).await);
                    },
                    "C" => {
                        // Set to C
                        println!("CN7500 set to Celsius: {:?}", cn.set_degrees(Degree::Celsius).await);
                    }
                    _ => {
                        println!("Invalid arg '{}', should be C or F.", args[1]);
                    }
                }
                continue;
            }
            _ => {}
        }


    }
}
