use std::io::{BufReader, BufRead};
use std::fs::File;

const INPUT: i32 = 1;

fn main() {
    let file = BufReader::new(File::open("input").unwrap());
    for line in file.lines() {
        print!("Reading in program...");
        let _my_line = line.unwrap();
        let split = _my_line.split(",");    
        let mut program = split.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("done");
        print!("Running program...");
        let retval = run(&mut program);
        println!("...done");
        println!("Location 0 is {}", retval);

    }
}

fn run<'a>(program: &'a mut Vec<i32>) -> i32 {
  let mut i: usize = 0;
  loop {
    if program[i] == 99 { 
        break; 
    }
    execute(program, i);
    i += match program[i] % 100 {
        1 => 4,
        2 => 4,
        3 => 2,
        4 => 2,
        _ => panic!("Unknown instruction length for {} at {}", program[i], i)
    }
  }

  return program[0];
}

fn execute<'b>(program: &'b mut Vec<i32>, i: usize) {
  let instruction = program[i];
  //println!("Instruction {}", instruction);
  if instruction % 100 == 1 {
      let oper1 = program[i+1] as usize;
      let oper2 = program[i+2] as usize;
      let retloc = program[i+3] as usize;
      //println!("Adding, previous retloc {} value {}", retloc, program[retloc]);
      program[retloc] = 
        match instruction % 1000 - 1 {
            100..=900 => oper1 as i32,
            _         => program[oper1]
        }
        +
        match instruction % 10000 - 1 {
            1000..=9000 => oper2 as i32,
            _           => program[oper2]
        };
     // println!("Adding, new retloc {} value {}", retloc, program[retloc]);
  }
  if instruction % 100 == 2 {
      let oper1 = program[i+1] as usize;
      let oper2 = program[i+2] as usize;
      let retloc = program[i+3] as usize;
      program[retloc] = 
        match instruction % 1000 - 2 {
            100..=900 => oper1 as i32,
            _         => program[oper1]
        }
        *
        match instruction % 10000 - 2 {
            1000..=9000 => oper2 as i32,
            _           => program[oper2]
        };
  }
  if instruction % 100 == 3 {
      //println!("Storing input");
      let retloc = program[i+1] as usize;
      program[retloc] = INPUT;
  }
  if instruction % 100 == 4 {
      let retloc = program[i+1] as usize;
      println!("Output from inst 4 is {}", program[retloc]);
  }
}
