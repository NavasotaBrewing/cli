#![allow(non_snake_case)]

const HELP_PAGE: &'static str = include_str!("help_page");
use std::io::{Write, stdout, stdin};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::clear;

// macro_rules! prompt {
//     ($type:ty) => {
//         match prompt().parse::<$type>() {
//             Ok(value) => Some(value),
//             Err(e) => {
//                 eprintln!("Couldn't get value of the correct type, try again. ({})", e);
//                 None
//             }
//         }
//     };
// }


// fn prompt() -> String {
//     print!(">  ");
//     stdout().flush().unwrap();

//     let mut buffer = String::new();
//     match stdin().read_line(&mut buffer) {
//         Ok(_) => {},
//         Err(e) => {
//             eprintln!("Error: couldn't read line: {}", e);
//         }
//     }

//     String::from(buffer.trim())
// }

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let mut cmd_buffer = String::new();
    let mut history: Vec<String> = vec![];

    println!("{}", clear::All);


    write!(stdout, "> ");
    for c in stdin.keys() {
        // Clear the current line.
        // write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();
        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Ctrl('q') => break,
            Key::Char(c)   => {
                cmd_buffer.push(c);
                write!(stdout, "{}", c);
            },

            Key::Backspace => {
                cmd_buffer.pop();
                write!(stdout, "^H ^H");
            },
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => {},
        }
        // writeln!(stdout, "> {}", cmd_buffer);

        // Flush again.
        stdout.flush().unwrap();
    }

    // loop {
    //     if let Some(cmd) = prompt!(String) {
    //         match cmd.as_str() {
    //             _ => println!("{}", HELP_PAGE)
    //         }
    //     }
    // }
}
