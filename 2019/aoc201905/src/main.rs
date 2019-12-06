use std::io::{BufReader, BufRead};
use std::fs::File;

const INPUT: i32 = 5;

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
    let this_instruction = program[i] % 100;
    let this_i = i.clone();
    execute(program, &mut i);
    i += match this_instruction {
        1 => 4,
        2 => 4,
        3 => 2,
        4 => 2,
        5 => 3,
        6 => 3,
        7 => 4,
        8 => 4,
        _ => panic!("Unknown instruction length for {} at {}", this_instruction, this_i)
    }
  }

  return program[0];
}

fn execute<'b>(program: &'b mut Vec<i32>, i: &mut usize) {
  let instruction = program[*i];
  if instruction % 100 == 1 {
      let oper1 = program[*i+1] as usize;
      let oper2 = program[*i+2] as usize;
      let retloc = program[*i+3] as usize;
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
  }
  if instruction % 100 == 2 {
      let oper1 = program[*i+1] as usize;
      let oper2 = program[*i+2] as usize;
      let retloc = program[*i+3] as usize;
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
      let retloc = program[*i+1] as usize;
      program[retloc] = INPUT;
  }
  if instruction % 100 == 4 {
      let retloc = program[*i+1] as usize;
      println!("Output from inst 4 is {}", program[retloc]);
  }
  if instruction % 100 == 5 {
      let oper1 = program[*i+1] as usize;
      let retloc = program[*i+2] as usize;
      let tester = match instruction % 1000 - 5 {
          100..=900 => oper1 as i32,
          _         => program[oper1]
      };
      if tester != 0 {
          *i = match instruction % 10000 - 5 {
              1000..=9000 => retloc - 3,
              _ => program[retloc] as usize - 3
          }
      };
  }
  if instruction % 100 == 6 {
      let oper1 = program[*i+1] as usize;
      let retloc = program[*i+2] as usize;
      let tester = match instruction % 1000 - 6 {
          100..=900 => oper1 as i32,
          _         => program[oper1]
      };
      if tester == 0 {
          *i = match instruction % 10000 - 5 {
              1000..=9000 => retloc - 3,
              _ => program[retloc] as usize - 3
          }
      };
  }
  if instruction % 100 == 7 {
      let oper1 = program[*i+1] as usize;
      let oper2 = program[*i+2] as usize;
      let retloc = program[*i+3] as usize;
      let tester1 = match instruction % 1000 - 7 {
          100..=900 => oper1 as i32,
          _         => program[oper1]
      };
      let tester2 = match instruction % 10000 - 7 {
          1000..=9000 => oper2 as i32,
          _         => program[oper2]
      };
      program[retloc] = match tester1 < tester2 {
          true => 1,
          false => 0
      };
  }
  if instruction % 100 == 8 {
      let oper1 = program[*i+1] as usize;
      let oper2 = program[*i+2] as usize;
      let retloc = program[*i+3] as usize;
      let tester1 = match instruction % 1000 - 8 {
          100..=900 => oper1 as i32,
          _         => program[oper1]
      };
      let tester2 = match instruction % 10000 - 8 {
          1000..=9000 => oper2 as i32,
          _         => program[oper2]
      };
      program[retloc] = match tester1 == tester2 {
          true => 1,
          false => 0
      };
  }
}
