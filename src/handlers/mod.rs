use std::fmt::Display;

use brewdrivers::drivers::InstrumentError;

pub mod cn7500;
pub mod str1;
pub mod waveshare;
pub mod wavesharev2;


/// Converts Result<T, InstrumentError> into the string form of the value or the error.
/// This is used to process values as CLI output, so that errors will be reported but will not panic,
/// and to reduce boilerplate.
fn stringify<T: Display>(value: Result<T, InstrumentError>) -> String {
    match value {
        Ok(val) => format!("{}", val),
        Err(e) => format!("Error: {}", e)
    }
}