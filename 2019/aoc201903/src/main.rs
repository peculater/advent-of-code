#[derive(Hash, Clone, Copy, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


use std::io::{BufReader, BufRead};
use std::fs::File;
use array_tool::vec::*;
use std::collections::HashSet;

fn main() {
    let _dummy = vec![1,1,3,5].intersect(vec![1,2,3]);
    let file = BufReader::new(File::open("input").unwrap());
    let mut fileiter = file.lines();
    print!("Parsing first wire...");
    let first_line_string = fileiter.next().unwrap().unwrap();
    let first_line_steps = first_line_string.split(",").collect::<Vec<&str>>();
    let first_line_points = steps_to_points(&first_line_steps); 
    let mut first_line_hash = HashSet::new();
    for point in &first_line_points {
        first_line_hash.insert(point);
    }
    println!("done");
    println!("First wire has {} points", first_line_points.len());
    print!("Parsing second wire...");
    let second_line_string = fileiter.next().unwrap().unwrap();
    let second_line_steps = second_line_string.split(",").collect::<Vec<&str>>();
    let second_line_points = steps_to_points(&second_line_steps); 
    println!("done");
    println!("Second wire has {} points", second_line_points.len());
    println!("Calculating intersection.  This will take {} loops", second_line_points.len());
    let mut second_line_hash = HashSet::new();
    for point in &second_line_points {
        second_line_hash.insert(point);
    }
    //let intersections: Vec<Point> = second_line_points
    //        .into_iter()
    //        .filter(|p| first_line_points.contains(p))
    //        .collect();
    let intersections = first_line_hash.intersection(&second_line_hash);
    println!("done");
    //for intersection in intersections{
    //  println!("Intersected at {}, {} total of {}", intersection.x, intersection.y, intersection.x.abs() + intersection.y.abs());
    //}
    let min = intersections.min_by(|l, r| (l.x.abs()+l.y.abs()).cmp(&(r.x.abs()+r.y.abs()))).unwrap();
    println!("Min distance {}", min.x.abs() + min.y.abs());
}

fn steps_to_points<'a>(steps: &'a Vec<&str>) -> Vec<Point> {
    let mut returnable = vec![];
    let mut this_x:i32 = 0;
    let mut this_y:i32 = 0;
    for step in steps {
        let (dir, howmany) = car_cdr(step);
        let number:i32 = howmany.parse::<i32>().unwrap();
    //    println!("Stepping {} in the {} dir ", number, dir);
        for this_step in 1..=number {
            let this_point = match dir {
                "R" => Point { x: this_x + this_step, y: this_y },
                "L" => Point { x: this_x - this_step, y: this_y },
                "U" => Point { x: this_x, y: this_y + this_step },
                "D" => Point { x: this_x, y: this_y - this_step },
                _ => panic!("Bad direction {}", dir)
                };
            if this_step == number {
                this_x = this_point.x;
                this_y = this_point.y;
            }
            returnable.push(this_point);
        }
    }
    return returnable;
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}


