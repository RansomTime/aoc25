fn file_to_string(path: &str) -> String {
  use std::fs;
  fs::read_to_string(path).unwrap()
}

fn main() {
  use std::time::Instant;
  {
    let start = Instant::now();
    println!("Part 1: {}", part_1(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time for part 1: {:?}", duration);
  }
  {
    let start = Instant::now();
    println!("Part 2: {}", part_2(file_to_string("inputs/input")));
    let duration = start.elapsed();
    println!("Time for part 2: {:?}", duration);
  }
}


fn part_1(input: String) -> u32 {
  input.split('\n')
       .map(|x| get_battery_bank_rating(x))
       .sum()
}


fn part_2(input: String) -> u64 {
    input.split('\n')
       .map(|x| get_12_batteries_rating(x))
       .sum()
}

fn get_battery_bank_rating(input: &str) -> u32 {
  let batteries = input[..input.len() -1].chars();
  let last: u32 = input.chars().last().unwrap().to_digit(10).unwrap();
  let (mut a,mut b) = (0,0);
  for battery in batteries {
    let digit:u32 = battery.to_digit(10).unwrap();
    if digit > a {
      a = digit;
      b = 0;
      continue;
    }
    if digit > b {
      b = digit;
    }
  }
  if last > a || last > b {
    b = last;
  }
  a*10 + b
}


fn get_12_batteries_rating(input: &str) -> u64 {

  let mut partial = String::from("");
  let mut pos = 0;

  while partial.len() < 12 {
    //get the highest char from the window of possible next characters
    //then update pos to create the next window
    let window = &input[pos..(input.len()- 11+partial.len())];
    let selected = get_highest_char(window);
    partial.push(window.chars().nth(selected).unwrap());
    pos += 1+ selected;
  }

  partial.parse::<u64>().unwrap()
}

fn get_highest_char(input: &str) ->usize {

  //returns the position of the highest chr
  let mut res = 0;
  let mut res_digit = 0;
  for (e,i) in input.chars().enumerate() {
    let digit = i.to_digit(10).unwrap();
    if digit > res_digit {
      res_digit = digit; 
      res = e;
    }
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn battery_test_highest_num_normal() {
    assert_eq!(get_battery_bank_rating("987654321111111"),98);
  }

  #[test]
  fn battery_test_highest_num_at_end() {
    assert_eq!(get_battery_bank_rating("811111111111119"),89);
  }
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo" )), 357);
    assert_eq!(part_1(file_to_string("inputs/input")),17376);
  }

  #[test]
  fn battery_test_part_2_at_start() {
    assert_eq!(get_12_batteries_rating("987654321111111"), 987654321111);
  }

  #[test]
  fn battery_test_part_2_in_middle() {
    assert_eq!(get_12_batteries_rating("234234234234278"), 434234234278);
  }
  
  #[test]
  fn battery_test_part_2_12_digits_only() {
    assert_eq!(get_12_batteries_rating("9987654321123"), 998765432123);
  }

  #[test]
  fn highest_at_start() {
    assert_eq!(get_highest_char("9987654321123"), 0);
  }  


  #[test]
  fn highest_in_middle() {
    assert_eq!(get_highest_char("3987654321123"), 1);
  }  


  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 3121910778619);
  } 

  #[test]
  fn test_incorrect_answers() {
    assert!(part_2(file_to_string("inputs/input")) > 170151249695422)
  }
}
