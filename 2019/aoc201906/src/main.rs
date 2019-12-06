
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file = BufReader::new(File::open("input").unwrap());
    let mut my_map = HashMap::new();
    print!("Reading in program...");
    for line in file.lines() {
        let _my_line = line.unwrap();
        let mut split = _my_line.split(")");
        let center: String = split.next().unwrap().into();
        let orbiter: String = split.next().unwrap().into();
        my_map.insert(orbiter, center);
    }
    println!("done");
    let mut count:i32 = 0;
    for planet in my_map.keys() {
      let mut this_planet = planet;
      while this_planet != "COM" {
          count += 1;
          this_planet = my_map.get(this_planet).unwrap();
      }
    }
    println!("Count is {}", count);
}



