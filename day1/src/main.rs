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

fn clock_add(input: i32, add: i32, max: i32) -> i32 {
  let mut sum = input + add;
  while sum > max {
    sum -= max + 1;
  }
  while sum < 0 {
    sum += max + 1;
  }
  sum
}



fn part_1(input: String) -> i32 {
  let mut cur = 50;
  let mut pw = 0;
  for line in input.lines() {
    let mut chars = line.chars();
    cur = match chars.next().unwrap() {
      'L' => clock_add(cur, -chars.as_str().to_string().parse::<i32>().unwrap(), 99),
      'R' => clock_add(cur, chars.as_str().to_string().parse::<i32>().unwrap(), 99),
      _   => unreachable!(),
    };
    if cur == 0 {
      pw += 1;
    }
  }
  pw
}

fn part_2(input: String) -> i32 {
  let mut cur = 50;
  let mut pw = 0;
  for line in input.lines() {
    let mut chars = line.chars();
    let clicks;
    (cur, clicks) = match chars.next().unwrap() {
      'L' => clock_sub_with_clicks(cur, chars.as_str().to_string().parse::<i32>().unwrap(), 99),
      'R' => clock_add_with_clicks(cur, chars.as_str().to_string().parse::<i32>().unwrap(), 99),
      _   => unreachable!(),
    };
    pw += clicks;
  }
  pw
}

fn clock_add_with_clicks(input: i32, add: i32, max: i32) -> (i32, i32) {
  let mut sum = input + add;
  let mut clicks = 0;
  while sum > max {
    sum -= max + 1;
    clicks += 1;
  }
  while sum < 0 {
    sum += max + 1;
    clicks += 1;
  }
  (sum, clicks)
}

fn clock_sub_with_clicks(input: i32, sub: i32, max: i32) -> (i32, i32) {
  let mut sum = input - sub;
  let mut clicks = match input {
    0 => -1,
    _ => 0,
  };
  while sum < 0 {
    sum += max + 1;
    clicks += 1;
  }
  if sum == 0 {
    clicks += 1;
  }
  (sum, clicks)
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 3);
  }
  
  #[test]
  fn test_clock_add() {
    assert_eq!(clock_add(99, 1, 99), 0);
    assert_eq!(clock_add(4, 1, 99), 5);
  }
  
  #[test]
  fn test_clock_sub() {
    assert_eq!(clock_sub(0, 1, 99), 99);
    assert_eq!(clock_sub(4, 1, 99), 3);
  }
  
  #[test]
  fn test_clock_add_with_clicks() {
    assert_eq!(clock_add_with_clicks(0, 100, 99), (0,1));
    assert_eq!(clock_add_with_clicks(0, 200, 99), (0,2));
  }
  
  
  #[test]
  fn test_part_2_starts_at_zero() {
    assert_eq!(clock_sub_with_clicks(0,5,99),(95,0)); 
  }
  
  #[test]
  fn test_part_2_starts_and_ends_at_zero() {
    assert_eq!(clock_sub_with_clicks(0, 400, 99), (0,4));
  }
  
  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 6);
    assert_eq!(part_2(file_to_string("inputs/input")), 6671);
  }
  
}
