use super::stringify;

use brewdrivers::{controllers::STR1, drivers::serial::State};


pub fn get_relay(str1: &mut STR1, addr: u8) {
    println!("{}", stringify(str1.get_relay(addr)));
}

pub fn list_all(str1: &mut STR1) {
    match str1.list_all_relays() {
        Ok(_) => { /* it prints from the method */},
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub fn set_relay(str1: &mut STR1, relay_num: u8, new_state: State) {
    match str1.set_relay(relay_num, new_state) {
        Ok(_) => println!("Ok!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub fn set_cn(str1: &mut STR1, new_cn: u8) {
    match str1.set_controller_num(new_cn) {
        Ok(_) => println!("Ok! Don't forget to update your config file and restart the CLI"),
        Err(e) => eprintln!("Error: {}", e)
    }
}