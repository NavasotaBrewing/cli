use log::{info, error};

use brewdrivers::controllers::*;
use brewdrivers::state::BinaryState;

use super::stringify;

pub(crate) fn get_relay(ws: &mut Waveshare, relay_num: u8) {
    info!("{}", stringify(ws.get_relay(relay_num)));
}

pub(crate) fn list_all(ws: &mut Waveshare) {
    let states = match ws.get_all_relays() {
        Ok(list) => list,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let mut i = 0;
    for state in states {
        info!("Relay {}: {}", i, state);
        i += 1;
    }
}

pub(crate) fn set_relay(ws: &mut Waveshare, relay_num: u8, new_state: BinaryState) {
    match ws.set_relay(relay_num, new_state) {
        Ok(_) => info!("Ok!"),
        Err(e) => error!("{}", e)
    }
}

pub(crate) fn get_cn(ws: &mut Waveshare) {
    info!("{}", stringify(ws.get_address()));
}

pub(crate) fn software_revision(ws: &mut Waveshare) {
    info!("{}", stringify(ws.software_revision()));
}

pub(crate) fn set_all(ws: &mut Waveshare, new_state: BinaryState) {
    match ws.set_all_relays(new_state) {
        Ok(_) => info!("Ok!"),
        Err(e) => error!("{}", e)
    }
}

pub(crate) fn set_cn(ws: &mut Waveshare, new_cn: u8) {
    match ws.set_address(new_cn) {
        Ok(_) => info!("Ok!"),
        Err(e) => error!("{}", e)
    }
}
