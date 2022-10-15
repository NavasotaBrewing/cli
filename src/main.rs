#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]
use shellfish::{Command, Shell, async_fn};
use std::error::Error;
use std::io::{stdout, Write};

use brewdrivers::controllers::{CN7500, Waveshare, STR1};
use brewdrivers::controllers::cn7500::Degree;

use nbc_iris::model::{RTU, Driver, Device};

mod commands_table;
mod handlers;

const COMMANDS_PAGE: &'static str = include_str!("commands");

#[tokio::main]
async fn main() {

    // Load the RTU Digital Twin from the config file and remove duplicate controllers
    // Multiple devices can run on a single controller, so we remove all but 1 device so we
    // get 1 device <-> 1 controller
    let mut rtu = RTU::generate(None).expect("Error, couldn't load RTU configuration from file /etc/NavasotaBrewing/rtu_conf.yaml");

    // Copy a list of device ids for use later
    let device_ids = &rtu.devices.iter().map(|dev| dev.id.clone() ).collect::<Vec<String>>();


    let mut shell = Shell::new_async(rtu, "BCS => ");

    shell.commands.insert(
        "commands",
        Command::new("Lists all commands".to_string(), commands)
    );

    for device_id in device_ids {
        shell.commands.insert(
            &device_id,
            Command::new_async(format!("operations for {}", device_id), async_fn!(RTU, device_ops))
        );
    }


    match shell.run_async().await {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn commands(rtu: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", COMMANDS_PAGE);
    println!("{}", commands_table::devices_list(&rtu));
    println!("{}", commands_table::str1_commands());
    println!("{}", commands_table::waveshare_commands());
    println!("{}", commands_table::cn7500_commands());
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
    let mut ws = Waveshare::connect(device.controller_addr, &device.port).expect(&format!("Couldn't connect to Waveshare board with ID: {}", device.id));
    todo!()
}

async fn handle_str1(device: &Device, args: Vec<String>) {
    let mut str1 = STR1::connect(device.controller_addr, &device.port).expect(&format!("Couldn't connect to STR1 board with ID: {}", device.id));
    str1.set_relay(1, brewdrivers::drivers::serial::State::Off).unwrap();
    todo!()
}

async fn handle_cn7500(device: &Device, args: Vec<String>) {
    // bring in all the CN7500
    use handlers::cn7500::*;
    let mut cn = CN7500::new(device.controller_addr, &device.port, 19200).await.unwrap();
    
    print!("({})\t", &device.name);
    stdout().flush().unwrap();

    if args.len() == 1 {
        // 0 argument commands

        get_all(&mut cn).await;
    }

    
    if args.len() == 2 {
        // 1 arg commands
        if let Some(arg1) = args.get(1) {
            match arg1.as_str() {
                "pv" => get_pv(&mut cn).await,
                "sv" => get_sv(&mut cn).await,
                "is_running" => is_running(&mut cn).await,
                "run" => run(&mut cn).await,
                "stop" => stop(&mut cn).await,
                _ => eprintln!("Argument {:?} not found, or you didn't provide enought arguments", arg1)
            }
        }
    }

    if args.len() == 3 {
        // 2 arguments
        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set" => {
                    match arg2.parse::<f64>() {
                        Ok(new_sv) => set_sv(&mut cn, new_sv).await,
                        Err(e) => eprintln!("`set` requires a floating point number as an argument: {}", e)
                    }
                },
                "degrees" => {
                    match arg2.as_str() {
                        "F" => set_degrees(&mut cn, Degree::Fahrenheit).await,
                        "C" => set_degrees(&mut cn, Degree::Celsius).await,
                        _ => eprintln!("Unkown argument `{}`", arg2),
                    }
                },
                _ => eprintln!("Argument `{}` not found, or you didn't provide enough arguments", arg1)
            }
        } else {
            eprintln!("Couldn't retrieve args 1 and 2");
        }
    }

    if args.len() > 3 {
        eprintln!("Too many arguments ({}) provided", args.len());
    }
    
}
