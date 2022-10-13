use shellfish::{Command, Shell};
use std::collections::HashMap;
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
    let device_pool = create_device_pool().await.expect("Couldn't build device pool");
    
    let mut shell = Shell::new_async(device_pool, "BCS => ");


    shell.commands.insert(
        "list_devices",
        Command::new("Lists all found controllers".to_string(), list_devices)
    );

    shell.run_async().await.unwrap();
}

async fn create_device_pool() -> Result<DevicePool, Box<dyn std::error::Error>> {
    let mut rtu = match RTU::generate(None) {
        Ok(rtu) => rtu,
        Err(e) => panic!("Error, couldn't load RTU configuration from file /etc/NavasotaBrewing/rtu_conf.yaml\n{}", e)
    };

    let mut device_pool = DevicePool::create();

    // Totally unrelated to this code or project, but Ron Cross is
    // the worst professor at the University of Texas at Arlington.
    // Just for posterity.

    let mut seen: HashMap<u8, bool> = HashMap::new();

    rtu.devices.retain(|dev| {
        if seen.get(&dev.controller_addr).is_none() {
            seen.insert(dev.controller_addr, true);
            return true;
        }
        false
    });

    // TODO: This logic could probably go in iris::Device. it should turn a Device model into a Brewdrivers instrument
    for device in rtu.devices {
        match device.driver {
            Driver::STR1 => {
                let str1 = STR1::connect(device.controller_addr, &device.port)?;
                device_pool.add(&device.id, Device::STR1(str1));
            },
            Driver::Waveshare => {
                let waveshare = Waveshare::connect(device.controller_addr, &device.port)?;
                device_pool.add(&device.id, Device::Waveshare(waveshare));
            },
            Driver::CN7500 => {
                // TODO: figure out how to bubble up the future error and not expect()
                let cn7500 = CN7500::new(device.controller_addr, &device.port, 19200)
                    .await
                    .expect("Couldn't connect to CN7500");

                device_pool.add(&device.id, Device::CN7500(cn7500));
            },
        }
    }

    Ok(device_pool)
}

fn list_devices(device_pool: &mut DevicePool, _: Vec<String>) -> Result<(), Box<dyn Error>> {
    for (key, device) in device_pool.devices() {
        println!("{}:\t{:?}", key, device);
    }
    Ok(())
}
