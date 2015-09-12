use std::default::Default;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io;
use std::io::Write;
use std::path::{PathBuf};
use std::process::exit;
use std::vec::Vec;

macro_rules! printerr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

struct Brightness {
    backend: String,
    max_brightness: i32,
}

impl std::default::Default for Brightness {
    fn default() -> Brightness {
        return Brightness {
            backend: "intel_backlight".to_string(),
            max_brightness: 0,
        }
    }
}

impl Brightness {
    fn get(&self, filename: &str) -> Result<i32, io::Error> {
        let mut path_buffer = PathBuf::from("/sys/class/backlight");
        path_buffer.push(self.backend.clone());
        path_buffer.push(filename);

        let path = path_buffer.as_path();
        let mut file = try!(File::open(path));

        let mut content = String::new();
        try!(file.read_to_string(&mut content));

        match content.trim().parse::<i32>() {
            Ok(value) => Ok(value),
            Err(err) => {
                printerr!("Error parsing value from file '{}': {}",
                         path.display(), err);
                Ok(0)
            }
        }
    }

    fn set_brightness(&self, mut value: i32) -> Result<bool, io::Error> {
        let max = try!(self.get_max_brightness());
        if value > max {
            value = max;
        } else if value < 0 {
            value = 0;
        }

        let mut path_buffer = PathBuf::from("/sys/class/backlight");
        path_buffer.push(self.backend.clone());
        path_buffer.push("brightness");

        let path = path_buffer.as_path();

        let mut file = try!(OpenOptions::new().write(true).open(path));

        match file.write_all(value.to_string().as_bytes()) {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        }
    }

    fn get_max_brightness(&self) -> Result<i32, io::Error> {
        if self.max_brightness > 0 {
            return Ok(self.max_brightness);
        }
        return self.get("max_brightness");
    }

    fn get_brightness(&self) -> Result<i32, io::Error> {
        return self.get("brightness");
    }

    fn get_percent(&self) -> Result<i32, io::Error> {
        let value = try!(self.get_brightness()) as f32;
        let max = try!(self.get_max_brightness()) as f32;
        let result = (100 as f32) * (value + 0.5) / max;
        return Ok(result as i32);
    }

    fn set_percent(&self, value: i32) -> Result<bool, io::Error> {
        let max = try!(self.get_max_brightness());
        let value = (value as f32) / (100_f32) * (max as f32) + 0.5_f32;
        let value = value as i32;
        return self.set_brightness(value as i32);
    }

}

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

    let brightness: Brightness = std::default::Default::default();
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
