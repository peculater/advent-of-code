extern crate regex;
use regex::Regex;

fn main() {
    let mut count: i32 = 0;
    for password in 125730..=579381 {
        if is_six_digit(password) &&
           has_a_double(password) &&
           monotonically_increases(password) {
             count += 1;
           }
    }
    println!("Count is {} ", count);

    count = 0;
    for password in 125730..=579381 {
        if is_six_digit(password) &&
           has_a_better_double(password) &&
           monotonically_increases(password) {
             count += 1;
           }
    }
    println!("Count is {} ", count);
}

fn is_six_digit(i: i32) -> bool {
    i < 1_000_000 && i >= 100_000
}

fn has_a_double(i: i32) -> bool {
    i.to_string().contains("00") ||
    i.to_string().contains("11") ||
    i.to_string().contains("22") ||
    i.to_string().contains("33") ||
    i.to_string().contains("44") ||
    i.to_string().contains("55") ||
    i.to_string().contains("66") ||
    i.to_string().contains("77") ||
    i.to_string().contains("88") ||
    i.to_string().contains("99") 
}

fn has_a_better_double(i: i32) -> bool {
    //// Important information, Rust regexes don't support lookahead/lookbehind
    //let re = Regex::new(r"(?!\1)(.)\1{2}(?!\1)").unwrap();
    //re.is_match(&i.to_string())
    i.to_string().contains("00") && !i.to_string().contains("000") ||
    i.to_string().contains("11") && !i.to_string().contains("111") ||
    i.to_string().contains("22") && !i.to_string().contains("222") ||
    i.to_string().contains("33") && !i.to_string().contains("333") ||
    i.to_string().contains("44") && !i.to_string().contains("444") ||
    i.to_string().contains("55") && !i.to_string().contains("555") ||
    i.to_string().contains("66") && !i.to_string().contains("666") ||
    i.to_string().contains("77") && !i.to_string().contains("777") ||
    i.to_string().contains("88") && !i.to_string().contains("888") ||
    i.to_string().contains("99") && !i.to_string().contains("999") 
}

fn monotonically_increases(i: i32) -> bool {
    let mut last: i32 = 0;
    for this_char in i.to_string().chars(){
      let this_int = this_char.to_string().parse::<i32>().unwrap();
      if this_int < last {
          return false;
      }
      last = this_int;
    }
    true
}
