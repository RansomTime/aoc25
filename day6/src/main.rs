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


fn part_1(input: String) -> u64 {
  let mut lines = input.lines();
  let mut problems = vec![];
  for e in problem_vec_from_str(lines.next().unwrap()) {
    problems.push(vec![e]);
  };
  
  let mut operands= vec![];
  
  for line in lines {
    //let line = lines.next().unwrap();
    if line.contains("+") {
      operands = operands_from_str(line);
      break;
    }
    
    for (i,e) in problem_vec_from_str(line).iter().enumerate() {
      problems[i].push(*e);
    }
  }
  
  let mut res = 0;
  for (i, e) in problems.iter().enumerate() {
    res += get_line_answer(e.clone(), operands[i])
  }
  res
}

fn part_2(input: String) -> u64 {
  
  let mut rtl_lines: Vec<String> = vec![String::from("");input.lines().next().unwrap().len()];
  let mut operands: Vec<Operand> = vec![];
  for line in input.lines() {
    for (i, e) in line.chars().rev().enumerate() {
      match e {
        ' ' => continue,
        '*' => operands.push(Operand::Multiply),
        '+' => operands.push(Operand::Add),
        '0'..='9' => rtl_lines[i].push(e),
        _ => unreachable!(),
      }
    }
  }
  
  let mut res = 0;
  let mut lines_iter = rtl_lines.iter();
  for operand in operands {
    let mut acc = match operand {
      Operand::Add => 0,
      Operand::Multiply => 1,
    };
    for num in lines_iter.by_ref() { 
      if num.is_empty() { break; }      
      acc = match operand {
        Operand::Add      => acc + num.parse::<u64>().unwrap(),
        Operand::Multiply => acc * num.parse::<u64>().unwrap(),
      };
    }
    res += acc;  
  }  
  res
}

fn problem_vec_from_str(input: &str)  -> Vec<u64> {
  let mut res = vec![];
  for x in input.split(" ") {
    if let Ok(number) = x.parse::<u64>() { res.push(number) };
  }
  res
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operand {
  Add,
  Multiply,
}

fn operands_from_str(input: &str)  -> Vec<Operand> {
  let mut res = vec![];
  for x in input.chars() {
    match x {
      '+' => res.push(Operand::Add),
      '*' => res.push(Operand::Multiply),
      ' ' => (),
      _ => unreachable!(),
    }
  }
  res
}

fn get_line_answer(line: Vec<u64>, operand: Operand) -> u64 {
  let mut res = match operand {
    Operand::Add => 0,
    Operand::Multiply => 1,
  };
  
  for num in line {
    res = match operand {
      Operand::Add      => res + num,
      Operand::Multiply => res * num,
    }  
  }
  res
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_1_parser() {
    assert_eq!(problem_vec_from_str("123 328  51 64 "), vec![123,328,51,64]);
  }
  
  #[test]
  fn test_part_1_operand_parser() {
    assert_eq!(operands_from_str("*   +   *   +  "), vec![Operand::Multiply, Operand::Add, Operand::Multiply, Operand::Add]);
  }
  #[test]
  fn test_part_1_first_line() {
    assert_eq!(get_line_answer(vec![123,45,6], Operand::Multiply),33210);
  }
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 4277556);
  }
  
  
  
  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 3263827);
  }
  
  #[test]
  fn test_incorrect_answers() {
  }
}
