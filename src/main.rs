#![allow(non_snake_case)]
use std::{error::Error, time::Duration};

use env_logger::Env;
use log::{error, info};
use shellfish::{Command, Shell, async_fn, app::App};
use chrono::Local;

use brewdrivers::controllers::*;
use brewdrivers::controllers::cn7500::Degree;
use nbc_iris::model::{RTU, Device};

mod tables;
mod handlers;

const CONFIG_FILE: &'static str = "/etc/NavasotaBrewing/rtu_conf.yaml";
const TIME_FORMAT: &'static str = "%F %H:%M:%S";


#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format_timestamp(None).init();

    // Connect the command line arguments
    let mut args: Vec<String> = std::env::args().collect();
    
    // if this is true, the program will parse the given CLI arguments and use those as a command.
    // If it's false, the shell will be opened.
    let mut run_exec = false;
    
    // If the first arg is `exec`, then we want to run the application
    // with the provided command, not open the shell.
    if let Some(arg1) = args.get(1) {
        // If we found arg1 to be exec, set a flag and remove `exec` from the args
        // So that we can parse the commands properly
        run_exec = arg1 == "exec";
        args.remove(1);
    }


    // Load the RTU Digital Twin from the config file
    let mut rtu = match RTU::generate(Some(CONFIG_FILE)) {
        Ok(rtu) => rtu,
        Err(e) => {
            error!("Couldn't deserialize config file: {}", e);
            std::process::exit(1);
        }
    };

    // Copy a list of device ids for use later
    let device_ids = &rtu.devices.iter().map(|dev| dev.id.clone() ).collect::<Vec<String>>();
    
    // Create a shell
    let mut shell = Shell::new_async(rtu.clone(), format!("🍺 ==> "));

    // Add a few basic commands
    // this one lists the available commands, dynamically generated from the RTU configuration
    shell.commands.insert(
        "commands",
        Command::new("Lists all commands".to_string(), |_, _| {
            info!("Commands are run with a device ID, not the device name. Some commands affect the controller (like STR1.set_cn). Those commands can be run from any device configured to that controller.");
            println!("{}", tables::commands::render());
            Ok(())
        })
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

    shell.commands.insert(
        "dashboard",
        Command::new_async("Starts the device dashboard".to_string(), async_fn!(RTU, dashboard))
    );

    // For each device, add that devices id as the command
    for device_id in device_ids {
        shell.commands.insert(
            &device_id,
            Command::new_async(format!("operations for {}", device_id), async_fn!(RTU, device_ops))
        );
    }

    
    
    // Run either the cli or the shell
    if run_exec {
        // CLI
        let mut app = App::try_from_async(shell).unwrap();
        app.handler.proj_name = Some(String::from("nbc_cli"));
        app.load_cache().unwrap();
        app.run_vec_async(args).await.unwrap();
    } else {
        // Run the shell
        info!("Navasota Brewing Company -- RTU CLI Version {}", env!("CARGO_PKG_VERSION"));
        info!("RTU config built successfully from file `{CONFIG_FILE}`");
        devices(&mut rtu, vec![]).unwrap();
        println!("Prost!");
        match shell.run_async().await {
            Ok(_) => {},
            Err(e) => error!("Error: {}", e)
        }
    }

}

fn devices(rtu: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", tables::devices::render(&rtu));
    Ok(())
}

async fn device_ops(rtu: &mut RTU, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let device_id = args.get(0).expect("Arg not provided, this shouldn't be possible");

    if let Some(dev) = rtu.devices.iter().find(|dev| dev.id == *device_id ) {
        match dev.controller {
            Controller::STR1 => handle_str1(dev, args).await,
            Controller::CN7500 => handle_cn7500(dev, args).await,
            Controller::Waveshare => handle_ws(dev, args).await,
        }
    }

    Ok(())
}

async fn handle_ws(device: &Device, args: Vec<String>) {
    use handlers::waveshare as ws;
    let mut ws = match Waveshare::connect(device.controller_addr, &device.port) {
        Ok(ws) => ws,
        Err(e) => {
            error!("Couldn't connect to Waveshare: {}", e);
            return;
        }
    };


    if args.len() == 1 {
        // No arguments

        ws::get_relay(&mut ws, device.addr);
    }

    if args.len() == 2 {
        // 1 argument
        if let Some(arg1) = args.get(1) {
            if let Ok(state) = arg1.parse::<BinaryState>() {
                ws::set_relay(&mut ws, device.addr, state);
                return;
            }

            match arg1.as_str() {
                "list_all" => ws::list_all(&mut ws),
                "get_cn" => ws::get_cn(&mut ws),
                "software_revision" => ws::software_revision(&mut ws),
                _ => error!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
            
            
        }
    }

    if args.len() == 3 {
        // 2 arguments

        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set_all" => {
                    match arg2.parse::<BinaryState>() {
                        Ok(state) => ws::set_all(&mut ws, state),
                        Err(e) => error!("{}", e)
                    }
                },
                "set_cn" => {
                    match arg2.parse::<u8>() {
                        Ok(new_cn) => ws::set_cn(&mut ws, new_cn),
                        Err(e) => error!("Error, couldn't parse controller number (0-254): {}", e)
                    }
                },
                _ => error!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

}

async fn handle_str1(device: &Device, args: Vec<String>) {
    use handlers::str1 as s;
    let mut str1 = match STR1::connect(device.controller_addr, &device.port) {
        Ok(str1) => str1,
        Err(err) => {
            error!("Couldn't connect to STR1 board with ID: {}\nError: {}", device.id, err);
            return;
        }
    };
    


    if args.len() == 1 {
        s::get_relay(&mut str1, device.addr);
    }

    if args.len() == 2 {
        if let Some(arg1) = args.get(1) {
            if let Ok(state) = arg1.parse::<BinaryState>() {
                s::set_relay(&mut str1, device.addr, state);
                return;
            }
            
            match arg1.as_str() {
                "list_all" => s::list_all(&mut str1),
                _ => error!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() == 3 {
        if let (Some(arg1), Some(arg2)) = (args.get(1), args.get(2)) {
            match arg1.as_str() {
                "set_cn" => {
                    match arg2.parse::<u8>() {
                        Ok(new_cn) => s::set_cn(&mut str1, new_cn),
                        Err(e) => error!("Couldn't parse new controller number (0-255): {}", e)
                    }
                },
                _ => error!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        }
    }

    if args.len() > 3 {
        error!("Too many arguments ({}) provided: {:?}", args.len(), args);
    }
}

async fn handle_cn7500(device: &Device, args: Vec<String>) {
    // bring in all the CN7500
    use handlers::cn7500 as c;
    let mut cn = match CN7500::connect(device.controller_addr, &device.port).await {
        Ok(cn) => cn,
        Err(err) => {
            error!("Couldn't connect to CN7500 with ID: {}\nError: {}", device.id, err);
            return;
        }
    };
    


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
                _ => error!("Argument {:?} not found, or you provided the wrong number of arguments", arg1)
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
                        Err(e) => error!("`set` requires a floating point number as an argument: {}", e)
                    }
                },
                "degrees" => {
                    match arg2.as_str() {
                        "F" => c::set_degrees(&mut cn, Degree::Fahrenheit).await,
                        "C" => c::set_degrees(&mut cn, Degree::Celsius).await,
                        _ => error!("Unkown argument `{}`", arg2),
                    }
                },
                _ => error!("Argument `{}` not found, or you provided the wrong number of arguments", arg1)
            }
        } else {
            error!("Couldn't retrieve args 1 and 2");
        }
    }

    if args.len() > 3 {
        error!("Too many arguments ({}) provided: {:?}", args.len(), args);
    }

}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

async fn dashboard(mut rtu: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    clear();
    loop {
        match tables::dashboard::render(&mut rtu).await {
            Ok(table) => println!("{}", table),
            Err(e) => {
                error!("{}", e);
                break;
            },
        }
        std::thread::sleep(Duration::from_millis(5000));
        clear();
    }
    Ok(())
}