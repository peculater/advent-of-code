extern crate libm;

use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    print!("Reading in file...");
    let mut basicsum = 0f64;
    let mut extendedsum = 0f64;
    let file = BufReader::new(File::open("input").unwrap());
    for line in file.lines() {
        let _my_line = line.unwrap();
        let _my_number = _my_line.parse::<f64>().unwrap();
        let mut _this_fuel = fuel_for(_my_number);
        basicsum += _this_fuel;
        while _this_fuel > 0.0 {
            extendedsum += _this_fuel;
            _this_fuel = fuel_for(_this_fuel);
        }
    }
    println!("done");
    println!("Sum {}", basicsum);
    println!("Extended sum {}", extendedsum);
}

fn fuel_for(x : f64) -> f64 {
    let _my_floored = libm::floor(x / 3.0);
    return _my_floored - 2.0; 
}
