//! Contains the functionality of the cn7500 commands

use chrono::Local;
use nbc_iris::model::Device;
use std::thread::{sleep};
use std::time::Duration;

use std::io::{Write, stdout};

use brewdrivers::controllers::CN7500;
use brewdrivers::controllers::cn7500::Degree;

use super::stringify;

pub(crate) async fn get_all(cn: &mut CN7500) {
    println!(
        "{{ PV: {}, SV: {}, Running: {} }}",
        stringify(cn.get_pv().await),
        stringify(cn.get_sv().await),
        stringify(cn.is_running().await)
    );
}

pub(crate) async fn get_pv(cn: &mut CN7500) {
    println!("PV: {}", stringify(cn.get_pv().await));
}

pub(crate) async fn get_sv(cn: &mut CN7500) {
    println!("SV: {}", stringify(cn.get_sv().await));
}

pub(crate) async fn is_running(cn: &mut CN7500) {
    println!("Running: {}", stringify(cn.is_running().await));
}

pub(crate) async fn run(cn: &mut CN7500) {
    // cn.run() returns Ok(()) so we won't use stringify
    match cn.run().await {
        Ok(_) => println!("Ok!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) async fn stop(cn: &mut CN7500) {
    // cn.stop() returns Ok(()) so we won't use stringify
    match cn.stop().await {
        Ok(_) => println!("Stopped!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) async fn set_sv(cn: &mut CN7500, new_sv: f64) {
    match cn.set_sv(new_sv).await {
        Ok(_) => println!("Ok! Set to {}", new_sv),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) async fn set_degrees(cn: &mut CN7500, deg_mode: Degree) {
    match cn.set_degrees(deg_mode.clone()).await {
        Ok(_) => println!("Degree mode set to {:?}", deg_mode),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) async fn watch(device: &Device) {
    println!();
    loop {
        print!("\n{}\t", Local::now().format(crate::TIME_FORMAT));
        // I don't know why but we have to reconnect every time here
        match CN7500::new(device.controller_addr, &device.port, 19200).await {
            Ok(mut cn) => {
                println!(
                    "{{ PV: {}, SV: {}, Running: {} }}",
                    stringify(cn.get_pv().await),
                    stringify(cn.get_sv().await),
                    stringify(cn.is_running().await)
                );
            },
            Err(e) => eprintln!("Error: {}", e)
        }
        stdout().flush().unwrap();
        sleep(Duration::from_secs(5));
    }
}