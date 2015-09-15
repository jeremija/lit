use std::io::Write;
use std::vec::Vec;

extern crate backlight;

fn printerr(value: String) {
    match writeln!(&mut std::io::stderr(), "{}", value) {
        Ok(_) => {},
        Err(x) => panic!("Unable to write to stderr: {}", x),
    }
}

fn change_brightness(value: String, is_relative: bool) -> Result<i32, String> {
    let mut new_value: i32;
    match value.parse::<i32>() {
        Ok(value) => new_value = value,
        Err(e) => return Err(format!("Invalid value supplied: {}", e)),
    }

    let brightness: backlight::Brightness = std::default::Default::default();
    if is_relative {
        match brightness.get_percent() {
            Ok(value) => new_value = new_value + value,
            Err(e) => return Err(format!("Error reading brightness: {}", e)),
        }
    }

    match brightness.set_percent(new_value) {
        Ok(_) => {},
        Err(e) => return Err(format!("Error setting brightness: {}", e)),
    }

    match brightness.get_percent() {
        Ok(value) => Ok(value),
        Err(e) => Err(format!("Error reading brightness: {}", e)),
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 || args[1].len() == 0 {
        printerr(format!("Usage: {} [+/-]<value>", args[0]));
        std::process::exit(1);
    }

    let mut value = String::from(args[1].trim());
    let is_relative = value.starts_with('+') || value.starts_with('-');
    if value.starts_with('+') {
        value.remove(0);
    }

    match change_brightness(value, is_relative) {
        Ok(value) => println!("{}%", value),
        Err(e) => {
            printerr(e);
            std::process::exit(1);
        }
    }

}
