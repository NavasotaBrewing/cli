use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};
use term_table::{Table, TableStyle};

use nbc_iris::model::RTU;

fn cmd(cmd: &str, help: &str) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(cmd, 1, Alignment::Left),
        TableCell::new_with_alignment(help, 1, Alignment::Left),
    ])
}

pub fn waveshare_commands() -> String {
    let mut table = Table::new();
    table.max_column_width = 60;

    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Waveshare Commands", 2, Alignment::Center)
    ]));
    
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Command", 1, Alignment::Center),
        TableCell::new_with_alignment("Help", 1, Alignment::Center),
    ]));

    table.add_row(cmd("$ [relayID]", "Gets a relay status"));
    table.add_row(cmd("$ [relayID] list_all", "Lists states of this and all the neighboring relays on this controller"));
    table.add_row(cmd("$ [relayID] [On|Off|1|0]", "Turns a relay on or off"));
    table.add_row(cmd("$ [relayID] set_all [On|Off|1|0]", "Sets this and all the neighboring relays on this controller"));
    table.add_row(cmd("$ [relayID] get_cn", "Attempts to find the controller number the board is set to. The configured controller number (from the conf file) doesn't matter"));
    table.add_row(cmd("$ [relayID] set_cn [0-254]", "Sets a new controller number for this controller. You'll need to update your rtu_conf.yaml file. Don't forget the controller number"));
    table.add_row(cmd("$ [relayID] software_revision", "Lists the software revision currently on the board"));

    table.render()
}

pub fn str1_commands() -> String {
    let mut table = Table::new();
    table.max_column_width = 60;

    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("STR1 Commands", 2, Alignment::Center)
    ]));
    
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Command", 1, Alignment::Center),
        TableCell::new_with_alignment("Help", 1, Alignment::Center),
    ]));

    table.add_row(cmd("$ [relayID]", "Gets a relay status"));
    table.add_row(cmd("$ [relayID] list_all", "Lists states of all the neighboring relays on this controller"));
    table.add_row(cmd("$ [relayID] [On|Off|1|0]", "Turns a relay on or off"));
    table.add_row(cmd("$ [relayID] set_cn [0-254]", "Sets a new controller number for this controller. You'll need to update your rtu_conf.yaml file. Don't forget the controller number"));

    table.render()
}

pub fn cn7500_commands() -> String {
    let mut table = Table::new();
    table.max_column_width = 60;

    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("CN7500 Commands", 2, Alignment::Center)
    ]));
    
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Command", 1, Alignment::Center),
        TableCell::new_with_alignment("Help", 1, Alignment::Center),
    ]));

    table.add_row(cmd("$ [deviceID]", "Gets the PV, SV, and status of the relay"));
    table.add_row(cmd("$ [deviceID] pv", "Gets the Process Value (actual)"));
    table.add_row(cmd("$ [deviceID] sv", "Gets the Setpoint Value (target)"));
    table.add_row(cmd("$ [deviceID] set [#.#]", "Sets the SV. Use a decimal number"));
    table.add_row(cmd("$ [deviceID] is_running", "Returns the status of the relay"));
    table.add_row(cmd("$ [deviceID] run", "Turns the relay on"));
    table.add_row(cmd("$ [deviceID] stop", "Turns the relay off"));
    table.add_row(cmd("$ [deviceID] degrees [F|C]", "Sets degree units to F or C"));
    
    table.render()
}


pub fn devices_list(rtu: &RTU) -> String {
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::extended();

    // Title row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Configured Devices", 6, Alignment::Center)
    ]));

    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("ID", 1, Alignment::Center),
        TableCell::new_with_alignment("Name", 1, Alignment::Center),
        TableCell::new_with_alignment("Type", 1, Alignment::Center),
        TableCell::new_with_alignment("Controller Addr", 1, Alignment::Center),
        TableCell::new_with_alignment("Device Addr", 1, Alignment::Center),
        TableCell::new_with_alignment("Port", 1, Alignment::Center),
    ]));

    // Values from devices
    for device in &rtu.devices {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(&device.id, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.name, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.driver.to_string(), 1, Alignment::Left),
            TableCell::new_with_alignment(&device.controller_addr, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.addr, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.port, 1, Alignment::Left)
        ]));
    }

    table.render()
}