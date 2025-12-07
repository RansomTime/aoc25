use std::collections::HashMap;
use std::collections::HashSet;

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


fn part_1(input: String) -> usize {
  let mut lines = input.lines();
  let first = lines.next().unwrap();
  let mut tachyons: HashSet<usize> = HashSet::new();
  for (i, e) in first.chars().enumerate() {
    if e == 'S' {
      tachyons.insert(i);
    }
  }
  let mut res = 0;
  
  while let Some(next) = lines.next() {
    if !next.contains('^') { //skip the empty . lines
      continue;
    }
    let mut new_tach = HashSet::new();
    let next_chr: Vec<char> = next.chars().collect();
    for tach in tachyons {
      match next_chr[tach] {
        '.' => {
          new_tach.insert(tach);
        },
        '^' => {
          new_tach.insert(tach+1);
          new_tach.insert(tach-1);
          res += 1;
        }
        _ => unreachable!(),
      }
    }
    tachyons = new_tach;
  }
  res
}

fn part_2(input: String) -> u64 {
  let mut lines = input.lines();
  let first: &str = lines.next().unwrap();
  let mut positions: HashMap<usize,u64> = HashMap::new(); //index, count
  for (i, e) in first.chars().enumerate() {
    if e == 'S' {
      positions.insert(i,1);
    }
  }
  while let Some(next) = lines.next() {
    if !next.contains('^') { //skip the empty . lines
      continue;
    }
    let mut new_pos: HashMap<usize,u64> = HashMap::new();
    let next_chr: Vec<char> = next.chars().collect();
    for (index, count) in positions {
      match next_chr[index] {
        '.' => {
          new_pos.insert(index, new_pos.get(&index).unwrap_or(&0) + count);
        },
        '^' => {
          new_pos.insert(index + 1, new_pos.get(&(index + 1)).unwrap_or(&0) + count);
          new_pos.insert(index - 1, new_pos.get(&(index - 1)).unwrap_or(&0) + count);
        }
        _ => unreachable!(),
      }
    }
    positions = new_pos;
  }
  positions.values().sum()
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 21);
  }
  
  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 40);
    assert_eq!(part_2(file_to_string("inputs/input")),25489586715621);
  } 
  
  #[test]
  fn test_incorrect_answers() {
    assert!(part_2(file_to_string("inputs/input")) > 12608160008022);
  }
}
