use shellfish::{app, Command, Shell, async_fn};
// use std::convert::TryInto;
use std::error::Error;
// use std::fmt;
// use std::ops::AddAssign;
// use std::pin::Pin;

use brewdrivers::device_pool::{DevicePool, Device};
use brewdrivers::omega::CN7500;
use brewdrivers::relays::{STR1, Waveshare};

use nbc_iris::model::{RTU, Driver};

#[tokio::main]
async fn main() {
    let device_pool = create_device_pool();
    
    let mut shell = Shell::new_async(0_u64, "BCS => ");


    shell.commands.insert(
        "greet",
        Command::new("Greets you".to_string(), greet)
    );

    shell.commands.insert(
        "cat",
        Command::new_async(
            "Displays a plaintext file.".to_string(),
            async_fn!(u64, cat)
        ),
    );

    // If there are args provided to the executable, don't start the shell, just run the command
    let mut args = std::env::args();
    if args.nth(1).is_some() {
        // Create the app from the shell.
        let mut app = app::App::try_from_async(shell).unwrap();
        // Set the binary name
        app.handler.proj_name = Some("BCS".to_string());
        app.load_cache().unwrap();
        app.run_args_async().await.unwrap();
    } else {
        // Run the shell
        shell.run_async().await.unwrap();
    }
}

fn create_device_pool() -> DevicePool {
    let rtu = match RTU::generate(None) {
        Ok(rtu) => rtu,
        Err(e) => panic!("Error, couldn't load RTU configuration from file /etc/NavasotaBrewing/rtu_conf.yaml\n{}", e)
    };

    let mut device_pool = DevicePool::create();

    // Unfinished, gotta go to class
    for device in rtu.devices {
        match device.driver {
            Driver::STR1 => todo!(),
            Driver::CN7500 => todo!(),
            Driver::Waveshare => todo!(),
        }
    }

    device_pool
}

fn greet(_state: &mut u64, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let arg = args.get(1).unwrap();
    println!("Greetings {}, my good friend.", arg);
    Ok(())
}

async fn cat(_state: &mut u64, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}