#![allow(non_snake_case)]
use std::io::{stdout, Write};
use std::error::Error;

use shellfish::{Command, Shell, async_fn};
use chrono::Local;

use brewdrivers::drivers::serial::State;
use brewdrivers::controllers::{CN7500, Waveshare, STR1};
use brewdrivers::controllers::cn7500::Degree;
use nbc_iris::model::{RTU, Driver, Device};

mod commands_table;
mod handlers;

const COMMANDS_PAGE: &'static str = include_str!("commands");
const DEFAULT_CONF_FILE: &'static str = "/etc/NavasotaBrewing/rtu_conf.yaml";
const TIME_FORMAT: &'static str = "%F %H:%M:%S";


#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();

    // Use either the provided conf file or the default one
    let conf_file = match args.get(1) {
        Some(conf_file) => {
            println!("Using custom config file: {}", conf_file);
            conf_file
        },
        None => {
            println!("Using default config file: {}", DEFAULT_CONF_FILE);
            DEFAULT_CONF_FILE
        }
    };

    // Load the RTU Digital Twin from the config file
    let mut rtu = RTU::generate(Some(conf_file)).expect(&format!("Error, couldn't load RTU configuration from file {}", conf_file));
    println!("RTU config build successfully!");
    devices(&mut rtu, vec![]).unwrap();

    // Copy a list of device ids for use later
    let device_ids = &rtu.devices.iter().map(|dev| dev.id.clone() ).collect::<Vec<String>>();
    
    // Create a shell
    let mut shell = Shell::new_async(rtu, format!("🍺 ==> "));

    // Add a few basic commands
    // this one lists the available commands, dynamically generated from the RTU configuration
    shell.commands.insert(
        "commands",
        Command::new("Lists all commands".to_string(), commands)
    );

    // This prints a list of connected devices
    shell.commands.insert(
        "devices",
        Command::new("Lists all connected devices".to_string(), devices)
    );

    shell.commands.insert(
        "time",
        Command::new("Prints the current timestamp".to_string(), |_, _| {
            println!("{}", Local::now().format("%F %H:%M:%S"));
            Ok(())
        })
    );

    // For each device, add that devices id as the command
    for device_id in device_ids {
        shell.commands.insert(
            &device_id,
            Command::new_async(format!("operations for {}", device_id), async_fn!(RTU, device_ops))
        );
    }

    
    // Run the shell
    println!("Prost!");
    match shell.run_async().await {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn commands(_: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", COMMANDS_PAGE);
    println!("{}", commands_table::str1_commands());
    println!("{}", commands_table::waveshare_commands());
    println!("{}", commands_table::cn7500_commands());
    Ok(())
}

fn devices(rtu: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", commands_table::devices_list(&rtu));
    Ok(())
}

async fn device_ops(rtu: &mut RTU, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let device_id = args.get(0).expect("Arg not provided, this shouldn't be possible");

    if let Some(dev) = rtu.devices.iter().find(|dev| dev.id == *device_id ) {
        match dev.driver {
            Driver::STR1 => handle_str1(dev, args).await,
            Driver::CN7500 => handle_cn7500(dev, args).await,
            Driver::Waveshare => handle_ws(dev, args).await,
        }
    }

    Ok(())
}

async fn handle_ws(device: &Device, args: Vec<String>) {
    use handlers::waveshare as ws;
    let mut ws = match Waveshare::connect(device.controller_addr, &device.port) {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Couldn't connect to Waveshare: Error: {}", e);
            return;
        }
    };

    print!("({})\t", &device.name);
    stdout().flush().unwrap();

    if args.len() == 1 {
        // No arguments

        ws::get_relay(&mut ws, device.addr);
    }

    if args.len() == 2 {
        // 1 argument
        if let Some(arg1) = args.get(1) {
            match arg1.as_str() {
                "list_all" => ws::list_all(&mut ws),
                "On" | "on" | "1" => ws::set_relay(&mut ws, device.addr, State::On),
                "Off" | "off" | "0" => ws::set_relay(&mut ws, device.addr, State::Off),
                "get_cn" => ws::get_cn(&mut ws),
                "software_revision" => ws::software_revision(&mut ws),
                _ => eprintln!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() == 3 {
        // 2 arguments

        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set_all" => {
                    match arg2.as_str() {
                        "On" | "on" | "1" => ws::set_all(&mut ws, State::On),
                        "Off" | "off" | "0" => ws::set_all(&mut ws, State::Off),
                        _ => eprintln!("Unknown argument `{}`, expecting [On|Off|1|0]", arg2),
                    }
                },
                "set_cn" => {
                    match arg2.parse::<u8>() {
                        Ok(new_cn) => ws::set_cn(&mut ws, new_cn),
                        Err(e) => eprintln!("Error, couldn't parse controller number (0-254): {}", e)
                    }
                },
                _ => eprintln!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

}

async fn handle_str1(device: &Device, args: Vec<String>) {
    use handlers::str1 as s;
    let mut str1 = match STR1::connect(device.controller_addr, &device.port) {
        Ok(str1) => str1,
        Err(err) => {
            eprintln!("Couldn't connect to STR1 board with ID: {}\nError: {}", device.id, err);
            return;
        }
    };
    
    print!("({})\t", &device.name);
    stdout().flush().unwrap();

    if args.len() == 1 {
        s::get_relay(&mut str1, device.addr);
    }

    if args.len() == 2 {
        if let Some(arg1) = args.get(1) {
            match arg1.as_str() {
                "list_all" => s::list_all(&mut str1),
                "On" | "on" | "1" => s::set_relay(&mut str1, device.addr, State::On),
                "Off" | "off" | "0" => s::set_relay(&mut str1, device.addr, State::Off),
                _ => eprintln!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() == 3 {
        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set_cn" => {
                    match arg2.parse::<u8>() {
                        Ok(new_cn) => s::set_cn(&mut str1, new_cn),
                        Err(e) => eprintln!("Couldn't parse new controller number (0-255): {}", e)
                    }
                },
                _ => eprintln!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() > 3 {
        eprintln!("Too many arguments ({}) provided: {:?}", args.len(), args);
    }
}

async fn handle_cn7500(device: &Device, args: Vec<String>) {
    // bring in all the CN7500
    use handlers::cn7500 as c;
    let mut cn = match CN7500::new(device.controller_addr, &device.port, 19200).await {
        Ok(cn) => cn,
        Err(err) => {
            eprintln!("Couldn't connect to CN7500 with ID: {}\nError: {}", device.id, err);
            return;
        }
    };
    
    print!("({})\t", &device.name);
    stdout().flush().unwrap();

    if args.len() == 1 {
        // 0 argument commands

        c::get_all(&mut cn).await;
    }
    
    if args.len() == 2 {
        // 1 arg commands
        if let Some(arg1) = args.get(1) {
            match arg1.as_str() {
                "pv" => c::get_pv(&mut cn).await,
                "sv" => c::get_sv(&mut cn).await,
                "is_running" => c::is_running(&mut cn).await,
                "run" => c::run(&mut cn).await,
                "stop" => c::stop(&mut cn).await,
                "watch" => c::watch(&device).await,
                _ => eprintln!("Argument {:?} not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() == 3 {
        // 2 arguments
        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set" => {
                    match arg2.parse::<f64>() {
                        Ok(new_sv) => c::set_sv(&mut cn, new_sv).await,
                        Err(e) => eprintln!("`set` requires a floating point number as an argument: {}", e)
                    }
                },
                "degrees" => {
                    match arg2.as_str() {
                        "F" => c::set_degrees(&mut cn, Degree::Fahrenheit).await,
                        "C" => c::set_degrees(&mut cn, Degree::Celsius).await,
                        _ => eprintln!("Unkown argument `{}`", arg2),
                    }
                },
                _ => eprintln!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        } else {
            eprintln!("Couldn't retrieve args 1 and 2");
        }
    }

    if args.len() > 3 {
        eprintln!("Too many arguments ({}) provided: {:?}", args.len(), args);
    }

}
