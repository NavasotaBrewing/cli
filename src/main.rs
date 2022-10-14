#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]
use shellfish::{Command, Shell, async_fn};
use std::error::Error;

use brewdrivers::controllers::{CN7500, Waveshare, STR1};

use nbc_iris::model::{RTU, Driver, Device};

mod commands_table;

const COMMANDS_PAGE: &'static str = include_str!("commands");

#[tokio::main]
async fn main() {

    // Load the RTU Digital Twin from the config file and remove duplicate controllers
    // Multiple devices can run on a single controller, so we remove all but 1 device so we
    // get 1 device <-> 1 controller
    let mut rtu = RTU::generate(None).expect("Error, couldn't load RTU configuration from file /etc/NavasotaBrewing/rtu_conf.yaml");
    // let mut seen: HashMap<u8, bool> = HashMap::new();
    // rtu.devices.retain(|dev| {
    //     if seen.get(&dev.controller_addr).is_none() {
    //         seen.insert(dev.controller_addr, true);
    //         return true;
    //     }
    //     false
    // });

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
            Command::new_async(format!("operations for {}", device_id), async_fn!(RTU, controller_ops))
        );
    }


    shell.run_async().await.unwrap();
}

fn commands(rtu: &mut RTU, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", COMMANDS_PAGE);
    println!("{}", commands_table::devices_list(&rtu));
    println!("{}", commands_table::str1_commands());
    println!("{}", commands_table::waveshare_commands());
    println!("{}", commands_table::cn7500_commands());
    Ok(())
}

async fn controller_ops(rtu: &mut RTU, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let controller_id = args.get(0).expect("Arg not provided, this shouldn't be possible");


    if let Some(dev) = rtu.devices.iter().find(|dev| dev.id == *controller_id ) {
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
    println!("Handling cn7500!");
    let mut cn = CN7500::new(device.controller_addr, &device.port, 19200).await.unwrap();
    println!("{}", cn.get_pv().await.unwrap());
    todo!()
}
