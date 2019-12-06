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
