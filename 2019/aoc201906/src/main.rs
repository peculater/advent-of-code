
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

    let mut mytransfers = HashMap::new();
    let mut my_planet = "YOU";
    let mut my_count = -1;
    while my_planet != "COM" {
        my_count += 1;
        my_planet = my_map.get(my_planet).unwrap();
        mytransfers.insert(my_planet, my_count);
    }
    let mut santa_planet = "SAN";
    let mut santa_count = -1;
    while santa_planet != "COM" {
        santa_count += 1;
        santa_planet = my_map.get(santa_planet).unwrap();
        if mytransfers.contains_key(santa_planet){
            let total_count = santa_count + mytransfers.get(santa_planet).unwrap();
            println!("Total transfers to meet santa is {}", total_count);
            break
        }
    }

}



