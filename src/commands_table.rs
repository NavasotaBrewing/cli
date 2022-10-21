use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};
use term_table::{Table, TableStyle};

use nbc_iris::model::RTU;

fn bold(text: &str) -> String {
    format!("{}{}{}", termion::style::Bold, text, termion::style::Reset)
}

fn cmd(cmd: &str, help: &str) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(cmd, 1, Alignment::Left),
        TableCell::new_with_alignment(help, 1, Alignment::Left),
    ])
}

pub fn commands_table() -> String {
    let mut table = Table::new();
    table.max_column_width = 80;
    
    general_commands(&mut table);
    waveshare_commands(&mut table);
    str1_commands(&mut table);
    cn7500_commands(&mut table);
    table.render()
}

fn general_commands(table: &mut Table) {
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("General Commands"), 2, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("Command"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Help"), 1, Alignment::Center),
    ]));

    table.add_row(cmd("help", "displays help information."));
    table.add_row(cmd("quit", "quits the shell"));
    table.add_row(cmd("exit", "exits the shell"));
    table.add_row(cmd("commands", "lists the commands page (this page)"));
    table.add_row(cmd("devices", "list all configured devices"));
    table.add_row(cmd("time", "prints the current time"));
    table.add_row(cmd("dashboard", "view a dashboard of all device states"));
}

fn waveshare_commands(table: &mut Table) {
    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("Waveshare Commands"), 2, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("Command"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Help"), 1, Alignment::Center),
    ]));

    table.add_row(cmd("[relayID]", "Gets a relay status"));
    table.add_row(cmd("[relayID] list_all", "Lists states of this and all the neighboring relays on this controller"));
    table.add_row(cmd("[relayID] [On|Off]", "Turns a relay on or off"));
    table.add_row(cmd("[relayID] set_all [On|Off]", "Sets this and all the neighboring relays on this controller"));
    table.add_row(cmd("[relayID] get_cn", "Attempts to find the controller number the board is set to. The configured controller number (from the conf file) doesn't matter"));
    table.add_row(cmd("[relayID] set_cn [0-254]", "Sets a new controller number for this controller. You'll need to update your rtu_conf.yaml file. Don't forget the controller number"));
    table.add_row(cmd("[relayID] software_revision", "Lists the software revision currently on the board"));
}

fn str1_commands(table: &mut Table) {
    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("STR1 Commands"), 2, Alignment::Center)
    ]));
    
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("Command"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Help"), 1, Alignment::Center),
    ]));

    table.add_row(cmd("[relayID]", "Gets a relay status"));
    table.add_row(cmd("[relayID] list_all", "Lists states of all the neighboring relays on this controller"));
    table.add_row(cmd("[relayID] [On|Off]", "Turns a relay on or off"));
    table.add_row(cmd("[relayID] set_cn [0-254]", "Sets a new controller number for this controller. You'll need to update your rtu_conf.yaml file. Don't forget the controller number"));
}

fn cn7500_commands(table: &mut Table) {
    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("CN7500 Commands"), 2, Alignment::Center)
    ]));
    
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("CN7500 Commands"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Help"), 1, Alignment::Center),
    ]));

    table.add_row(cmd("[deviceID]", "Gets the PV, SV, and status of the relay"));
    table.add_row(cmd("[deviceID] pv", "Gets the Process Value (actual)"));
    table.add_row(cmd("[deviceID] sv", "Gets the Setpoint Value (target)"));
    table.add_row(cmd("[deviceID] set [#.#]", "Sets the SV. Use a decimal number"));
    table.add_row(cmd("[deviceID] is_running", "Returns the status of the relay"));
    table.add_row(cmd("[deviceID] run", "Turns the relay on"));
    table.add_row(cmd("[deviceID] stop", "Turns the relay off"));
    table.add_row(cmd("[deviceID] degrees [F|C]", "Sets degree units to F or C"));
    table.add_row(cmd("[deviceID] watch", "Prints the PV and SV every few seconds until you quit"));
}


pub fn devices_list(rtu: &RTU) -> String {
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::extended();

    // Title row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("Configured Devices"), 6, Alignment::Center)
    ]));

    // Header row
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(bold("ID"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Name"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Type"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Controller Addr"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Device Addr"), 1, Alignment::Center),
        TableCell::new_with_alignment(bold("Port"), 1, Alignment::Center),
    ]));

    // Values from devices
    for device in &rtu.devices {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(&device.id, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.name, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.controller.to_string(), 1, Alignment::Left),
            TableCell::new_with_alignment(&device.controller_addr, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.addr, 1, Alignment::Left),
            TableCell::new_with_alignment(&device.port, 1, Alignment::Left)
        ]));
    }

    table.render()
}