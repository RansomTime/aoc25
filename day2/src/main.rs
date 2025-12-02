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


fn part_1(input: String) -> i128 {
  let mut res = 0;
  for line in input.split(',') {
    for id in split_range(line) {
      res += match is_only_repeating_digits(id) {
        true => id,
        false => 0,
      };
    }
  };
  res
}


fn split_range(range: &str) -> std::ops::RangeInclusive<i128>  {
  let mut range_split = range.split('-');
  let start: i128 = range_split.next().unwrap().parse::<i128>().unwrap();
  let end: i128 = range_split.next().unwrap().parse::<i128>().unwrap();
  start..=end
}

fn is_only_repeating_digits(input: i128) -> bool {
  let input_str = format!("{input}");
  if input_str.chars().count() % 2 == 1 {
    return false;
  }
  let half = input_str.chars().count() / 2;
  format!("{0}{0}",&input_str[..half]) == input_str
}

fn part_2(input: String) -> i128 {
  unimplemented!()
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
  assert_eq!(part_1(file_to_string("inputs/demo")), 1227775554);
  assert_eq!(part_1(file_to_string("inputs/input")),15873079081);
  }

    #[test]
  fn test_part_1_range() {
  assert_eq!(split_range("11-22"), 11..=22);
  }

  #[test]
  fn test_part_1_repeating_string_odd() {
    assert_eq!(is_only_repeating_digits(1), false);
    assert_eq!(is_only_repeating_digits(11), true);
    assert_eq!(is_only_repeating_digits(211), false);
  }

  #[test]
  fn test_part_1_repeating_string_two_digit() {
    assert_eq!(is_only_repeating_digits(21), false);
    assert_eq!(is_only_repeating_digits(99), true);
  }
    /*
  #[test]
  fn test_part_2() {
  assert_eq!(part_2(file_to_string("inputs/demo")), 2286);
  } */
  
  #[test]
  fn test_incorrect_answers() {
  }
}
