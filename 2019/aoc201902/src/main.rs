use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = BufReader::new(File::open("input").unwrap());
    for line in file.lines() {
        print!("Reading in program...");
        let _my_line = line.unwrap();
        let split = _my_line.split(",");    
        let mut program = split.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("done");
        print!("Fixup program...");
        program[1] = 12;
        program[2] = 2;
        println!("done");
        print!("Running program...");
        let retval = run(&mut program);
        println!("...done");
        println!("Location 0 is {}", retval);
        print!("Finding 19690720...");
        for noun in 0..100 {
            for verb in 0..100 {
                let mut thisprogram = _my_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                thisprogram[1] = noun;
                thisprogram[2] = verb;
                let retval = run(&mut thisprogram);
                if retval == 19690720 {
                    println!("done");
                    println!("noun {} verb {} result {}", noun, verb, 100*noun+verb);
                }
            }
        }

    }
}

fn run<'a>(program: &'a mut Vec<i32>) -> i32 {
  for i in (0..program.iter().count()).step_by(4) {
    if program[i] == 99 { 
        break; 
    }
    execute(program, i);
  }

  return program[0];
}

fn execute<'b>(program: &'b mut Vec<i32>, i: usize) {
  let instruction = program[i];
  if instruction == 1 {
      let oper1 = program[i+1] as usize;
      let oper2 = program[i+2] as usize;
      let retloc = program[i+3] as usize;
      program[retloc] = program[oper1] + program[oper2];
  }
  if instruction == 2 {
      let oper1 = program[i+1] as usize;
      let oper2 = program[i+2] as usize;
      let retloc = program[i+3] as usize;
      program[retloc] = program[oper1] * program[oper2];
  }
}
