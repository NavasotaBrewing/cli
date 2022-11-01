use log::{error, info};

use brewdrivers::controllers::*;
use brewdrivers::state::BinaryState;

use super::stringify;

pub fn get_relay(str1: &mut STR1, addr: u8) {
    info!("{}", stringify(str1.get_relay(addr)));
}

pub fn list_all(str1: &mut STR1) {
    match str1.list_all_relays() {
        Ok(_) => { /* it prints from the method */},
        Err(e) => error!("{}", e)
    }
}

pub fn set_relay(str1: &mut STR1, relay_num: u8, new_state: BinaryState) {
    match str1.set_relay(relay_num, new_state) {
        Ok(_) => info!("Ok!"),
        Err(e) => error!("{}", e)
    }
}

pub fn set_cn(str1: &mut STR1, new_cn: u8) {
    match str1.set_controller_num(new_cn) {
        Ok(_) => info!("Ok! Don't forget to update your config file and restart the CLI"),
        Err(e) => error!("{}", e)
    }
}