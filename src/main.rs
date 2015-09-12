use std::io::Write;
use std::process::exit;
use std::vec::Vec;

extern crate backlight;

macro_rules! printerr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 || args[1].len() == 0 {
        printerr!("Usage: {} [+/-]<value>", args[0]);
        std::process::exit(1);
    }

    let mut value = String::from(args[1].trim());
    let relative = value.starts_with('+') || value.starts_with('-');
    if value.starts_with('+') {
        value.remove(0);
    }

    let mut new_value: i32;
    match value.parse::<i32>() {
        Ok(value) => new_value = value,
        Err(e) => {
            printerr!("Invalid value supplied: {}", e);
            std::process::exit(1);
        }
    }

    let brightness: backlight::Brightness = std::default::Default::default();
    if relative {
        match brightness.get_percent() {
            Ok(value) => new_value = new_value + value,
            Err(e) => {
                printerr!("Error reading current brightness: {}", e);
                std::process::exit(1);
            }
        }
    }

    match brightness.set_percent(new_value) {
        Ok(_) => {},
        Err(e) => {
            printerr!("Error setting new brightness: {}", e);
            std::process::exit(1);
        }
    }

    match brightness.get_percent() {
        Ok(value) => println!("{}%", value),
        Err(e) => {
            printerr!("Error reading current brightness: {}", e);
            std::process::exit(1);
        }
    }
}
