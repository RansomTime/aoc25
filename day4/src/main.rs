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

struct TpGrid{
  grid: Vec<Vec<char>>
}

impl TpGrid {
  //note this is indexed [y][x] where [0][0] is top left
  fn from_input_str(input: String) -> TpGrid {
    let mut grid = vec!();
    for line in input.split('\n') {
      grid.push(line.chars().collect());
    }
    TpGrid { grid }
  }
  
  fn len (&self) -> usize {
    self.grid.len()
  }
  
  fn len_inner (&self, y: usize) -> usize {
    self.grid[y].len()
  }
  
  fn is_tp(&self, x: usize, y: usize) -> bool {
    self.grid[y][x] == '@'
  }
  
  fn count_tp(&self, x: usize, y: usize) -> u32 {
    let mut res = 0;
    for x_offset in [-1,0,1] {
      let new_x = match x.checked_add_signed(x_offset) {
        Some(a) => a,
        None => continue,
      };
      for y_offset in [-1,0,1] {

        if x_offset == 0 && y_offset == 0 { continue; } //itself

        let new_y = match y.checked_add_signed(y_offset) {
          Some(a) => a,
          None => continue,
        };
        
        if new_y > self.grid.len() -1 {
          continue;
        }
        if new_x > self.grid[new_y].len() -1 {
          continue;
        }
        if self.is_tp(new_x, new_y) {
          res += 1;
        }        
      }    
    }
    res
  }
  
  fn count_removals(&self) -> Option<Vec<(usize,usize)>> {
    let mut res = vec![];
    for y in 0..self.len() {
      for x in 0..self.len_inner(y) {
        match self.is_tp(x, y) {
          false => (),
          true => {
            let count = self.count_tp(x, y);
            if count < 4 {
              res.push((x,y));
            }
          },
        }
      }
    }
    if res.is_empty() {
      None
    } else {
      Some(res)
    }
  }
  
  fn remove(&mut self, coords: Vec<(usize,usize)>) {
    for (x,y) in coords {
      self.grid[y][x] = '.';      
    }
  }
}

fn part_1(input: String) -> usize {
  let grid = TpGrid::from_input_str(input);
  match grid.count_removals() {
    Some(answer) =>  answer.len(),
    None => 0,
  }
}

fn part_2(input: String) -> usize {
  let mut res = 0;
  let mut grid = TpGrid::from_input_str(input);
  loop {
    match grid.count_removals() {
      Some(removals) =>  {
        res += removals.len();
        grid.remove(removals);
      },
      None => return res,
    };      
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_1_count() {
    let grid = TpGrid::from_input_str(file_to_string("inputs/demo"));
    assert_eq!(grid.count_tp(6, 0),3);
  }
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 13);
    assert_eq!(part_1(file_to_string("inputs/input")), 1445);
  }
  
  
  
  #[test]
  fn test_part_2() {
    assert_eq!(part_2(file_to_string("inputs/demo")), 43);
  } 
  
  #[test]
  fn test_incorrect_answers() {
  }
}
