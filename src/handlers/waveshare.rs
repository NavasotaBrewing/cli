use brewdrivers::controllers::*;

use super::stringify;

pub(crate) fn get_relay(ws: &mut Waveshare, relay_num: u8) {
    println!("Get relay");
    println!("{}", stringify(ws.get_relay(relay_num)));
}

pub(crate) fn list_all(ws: &mut Waveshare) {
    println!("List all");
    let states = match ws.get_all_relays() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut i = 0;
    for state in states {
        println!("Relay {}: {}", i, state);
        i += 1;
    }
}

pub(crate) fn set_relay(ws: &mut Waveshare, relay_num: u8, new_state: BinaryState) {
    println!("set relay {} to {}", relay_num, new_state);
    match ws.set_relay(relay_num, new_state) {
        Ok(_) => println!("Ok!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) fn get_cn(ws: &mut Waveshare) {
    println!("Get cn");
    println!("{}", stringify(ws.get_address()));
}

pub(crate) fn software_revision(ws: &mut Waveshare) {
    println!("Software revision");
    println!("{}", stringify(ws.software_revision()));
}

pub(crate) fn set_all(ws: &mut Waveshare, new_state: BinaryState) {
    println!("Setting all to {}", new_state);
    match ws.set_all_relays(new_state) {
        Ok(_) => println!("Ok!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub(crate) fn set_cn(ws: &mut Waveshare, new_cn: u8) {
    println!("Setting cn to {}", new_cn);
    match ws.set_address(new_cn) {
        Ok(_) => println!("Ok!"),
        Err(e) => eprintln!("Error: {}", e)
    }
}
