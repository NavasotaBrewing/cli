#![allow(non_snake_case, dead_code, unused_variables)]
use shellfish::{Command, Shell, async_fn};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;

use brewdrivers::controllers::{CN7500, Waveshare, STR1};
use brewdrivers::controllers::{ControllerPool, Controller};

use nbc_iris::model::{RTU, Driver};

#[tokio::main]
async fn main() {

    // Load the RTU Digital Twin from the config file and remove duplicate controllers
    // Multiple devices can run on a single controller, so we remove all but 1 device so we
    // get 1 device <-> 1 controller
    let mut rtu = RTU::generate(None).expect("Error, couldn't load RTU configuration from file /etc/NavasotaBrewing/rtu_conf.yaml");
    let mut seen: HashMap<u8, bool> = HashMap::new();
    rtu.devices.retain(|dev| {
        if seen.get(&dev.controller_addr).is_none() {
            seen.insert(dev.controller_addr, true);
            return true;
        }
        false
    });

    // Copy a list of device ids for use later
    let device_ids = &rtu.devices.iter().map(|dev| dev.id.clone() ).collect::<Vec<String>>();

    
    // Create the controller pool. this will be 1 instance per controller, not device
    let mut pool = create_controller_pool(rtu).await.expect("Couldn't build device pool");

    // Testing block =============
    // let mut cn = CN7500::new(0x16, "/dev/ttyUSB0", 19200).await.expect("Couldn't get device");
    // if let Ok(pv) = cn.get_pv().await {
    //     println!("CN7500 PV: {}", pv);
    // } else {
    //     println!("Couldn't get PV from device, check the connection details!");
    // }
    // end testing --------------------

    let mut shell = Shell::new_async(pool, "BCS => ");

    shell.commands.insert(
        "list_controllers",
        Command::new_async("Lists all found controllers".to_string(), async_fn!(Mutex<ControllerPool>, list_controllers))
    );

    for device_id in device_ids {
        shell.commands.insert(
            &device_id,
            Command::new_async(format!("operations for {}", device_id), async_fn!(Mutex<ControllerPool>, controller_ops))
        );
    }


    shell.run_async().await.unwrap();
}



async fn create_controller_pool(rtu: RTU) -> Result<Mutex<ControllerPool>, Box<dyn std::error::Error>> {
    let mut pool = ControllerPool::create();

    // Totally unrelated to this code or project, but Ron Cross is
    // the worst professor at the University of Texas at Arlington.
    // Just for posterity.

    // TODO: This logic could probably go in iris::Device. it should turn a Device model into a Brewdrivers instrument
    // Or maybe RTU::into::<ControllerPool>()
    for device in rtu.devices {
        match device.driver {
            Driver::STR1 => {
                let str1 = STR1::connect(device.controller_addr, &device.port)?;
                pool.add(&device.id, Controller::STR1(str1));
            },
            Driver::Waveshare => {
                let waveshare = Waveshare::connect(device.controller_addr, &device.port)?;
                pool.add(&device.id, Controller::Waveshare(waveshare));
            },
            Driver::CN7500 => {
                // TODO: figure out how to bubble up the future error and not expect()
                let cn7500 = CN7500::new(device.controller_addr, &device.port, 19200)
                    .await
                    .expect("Couldn't connect to CN7500");

                pool.add(&device.id, Controller::CN7500(cn7500));
            },
        }
    }

    Ok(Mutex::new(pool))
}

async fn list_controllers(pool: &mut Mutex<ControllerPool>, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    for (key, controller) in pool.lock().await.controllers() {
        println!("{}:\t{:?}", key, controller);
    }
    Ok(())
}

async fn controller_ops(pool: &mut Mutex<ControllerPool>, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let controller_id = args.get(0).expect("Arg not provided, this shouldn't be possible");
    match args.len() {
        // Print the controller
        1 => {
            println!("{:#?}", pool.lock().await.controller(controller_id));
            return Ok(());
        }
        _ => {}
    }

    
    // for cont in pool.lock().await.controllers() {
    //     match cont.1 {
    //         Controller::CN7500(mut cn) => {
    //             // CN7500::new(, "/dev/ttyUSB0", 19200).await.expect("Couldn't get device");
    //             cn.get_pv().await.unwrap();
    //         },
    //         _ => {}
    //     }
    // }

    // Now we know there are >= 2 arguments

    if let Some(controller) = pool.lock().await.controller(controller_id) {
        match controller {
            Controller::CN7500(cn7500) => handle_cn7500(cn7500, args).await,
            Controller::STR1(str1) => handle_str1(str1, args).await,
            Controller::Waveshare(ws) => handle_ws(ws, args).await,
        }
    } else {
        eprintln!("Controller {} not found", controller_id);
    }

    Ok(())
}

async fn handle_ws(ws: &mut Waveshare, args: Vec<String>) {
    todo!()
}

async fn handle_str1(str1: &mut STR1, args: Vec<String>) {
    todo!()
}

async fn handle_cn7500(cn7500: &mut CN7500, args: Vec<String>) {
    println!("Handling cn7500!");
    println!("{:#?}", args);
    println!("{:#?}", cn7500);
    println!("Lets go: {:#?}", cn7500.get_pv().await.unwrap());
}
