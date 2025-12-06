use std::fmt;

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

fn part_1(input: String) -> u128 {
  let mut lines = input.lines();
  let mut ranges: Vec<Range> = vec![];
  loop {
    let line = lines.next().unwrap();
    if line.contains("-") {
      ranges.push(Range::from_str(line));
    } else {
      break;
    }
  }
  
  let mut res = 0;
  for line in lines {
    let needle = line.parse::<u128>().unwrap();
    for range in &ranges {
      if range.as_std_range().contains(&needle) {
        res += 1;
        break;
      }
    }
  };
  
  res
  
}

fn part_2(input: String) -> usize {
  let mut lines = input.lines();
  let mut ranges: Vec<Range> = vec![];
  loop {
    let line = lines.next().unwrap();
    if line.contains("-") {
      ranges.push(Range::from_str(line));
    } else {
      break;
    }
  }
  
  ranges = sort_ranges(ranges);
  
  let mut res = 0;
  for range in ranges {
    res += range.as_std_range().count();
  }
  res
  
  
}

fn sort_ranges(mut ranges: Vec<Range> ) -> Vec<Range> {
  /* recursively sorts ranges by merging them
  it'd probably be nice to do this in place
  but it's also past midnight and this works
  */
  let mut res = vec!();
  let mut changes = false;
  for i in 0..ranges.len() {
    let mut e = ranges[i];
    if e == Range::new(0,0) {
      //"deleted" ranges are set to 0,0 and not incuded in res
      continue;
    }
    for j in i+1..ranges.len() {
      let mut f = ranges[j];
      if e.overlaps(f) {
        e.merge(f);
        ranges[i] = e;
        f.start = 0;
        f.end = 0;
        ranges[j] = f;
        changes = true;
      }
    }
    res.push(e);
  }
  if changes {
    sort_ranges(res)
  } else {
    res
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RangeContainsResult {
  SubsetOfOther,
  OtherIsSubset,
  LeftExtend,
  RightExtend,
  NoOverlap,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Range {
  start: u128,
  end: u128
}

impl Range {
  fn new( start: u128, end: u128) -> Range {
    Range {
      start,
      end
    }
  }
  
  fn from_str(range: &str) -> Range {
    let mut range_split = range.split('-');
    let start = range_split.next().unwrap().parse::<u128>().unwrap();
    let end = range_split.next().unwrap().parse::<u128>().unwrap();
    Range {
      start,
      end
    }
  }
  
  fn as_std_range(&self) -> std::ops::RangeInclusive<u128>  {
    self.start..=self.end
  }
  
  fn compare(&self, other: Range) -> RangeContainsResult {
    if self.start > other.end {
      return RangeContainsResult::NoOverlap;      
    }
    if other.start > self.end {
      return RangeContainsResult::NoOverlap;
    }
    if self.start <= other.start && self.end >= other.end {
      return RangeContainsResult::OtherIsSubset;
    }
    if other.start <= self.start && other.end >= self.end {
      return RangeContainsResult::SubsetOfOther;
    }
    if other.start < self.start && self.as_std_range().contains(&other.end) {
      return RangeContainsResult::LeftExtend
    }
    if self.as_std_range().contains(&other.start) && other.end > self.end {
      return RangeContainsResult::RightExtend
    }
    unreachable!()
  }
  
  fn merge(&mut self, other: Range) {
    match self.compare(other) {
      RangeContainsResult::NoOverlap => panic!("Attemted to merge 2 non-overlapping ranges"),
      RangeContainsResult::OtherIsSubset => (),
      RangeContainsResult::SubsetOfOther => {
        self.start = other.start;
        self.end   = other.end;
      },
      RangeContainsResult::LeftExtend  => self.start = other.start,
      RangeContainsResult::RightExtend => self.end   = other.end,
    }
  }
  
  fn overlaps(&mut self, other: Range) -> bool {
    !matches!(self.compare(other), RangeContainsResult::NoOverlap)
  }
}

impl std::fmt::Display for Range {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f,"{0}..={1}",self.start, self.end)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_1() {
    assert_eq!(part_1(file_to_string("inputs/demo")), 3);
  }
  
  #[test]
  fn test_parse() {
    "484155089502467".parse::<u128>().unwrap();
  }
  
  #[test]
  fn test_no_overlap() {
    assert_eq!(Range::new(1,5).compare(Range::new(7,9)),RangeContainsResult::NoOverlap);
  }
  
  #[test]
  fn test_subset() {
    assert_eq!(Range::new(1,5).compare(Range::new(2,3)),RangeContainsResult::OtherIsSubset);
  }
  #[test]
  fn test_subset_other() {
    assert_eq!(Range::new(2,3).compare(Range::new(1,5)),RangeContainsResult::SubsetOfOther);
  }
  
  #[test]
  fn test_left_extend() {
    assert_eq!(Range::new(4,5).compare(Range::new(1,4)),RangeContainsResult::LeftExtend);
  }
  #[test]
  fn test_right_extend() {
    assert_eq!(Range::new(4,5).compare(Range::new(5,6)),RangeContainsResult::RightExtend);
  }
  
  #[should_panic]
  #[test]
  fn test_no_overlap_merge() {
    Range::new(1,5).merge(Range::new(7,9));
  }
  
  #[test]
  fn test_subset_merge() {
    let mut a = Range::new(1,5);
    let b = Range::new(2,3);
    a.merge(b);
    assert_eq!(a,Range::new(1,5));
  }
  
  #[test]
  fn test_subset_other_merge() {
    let mut a = Range::new(2,3);
    let b = Range::new(1,5);
    a.merge(b);
    assert_eq!(a,Range::new(1,5));
  }
  
  #[test]
  fn test_left_extend_merge() {
    let mut a = Range::new(4,5);
    let b = Range::new(1,4);
    a.merge(b);
    assert_eq!(a,Range::new(1,5));
    
    let mut c = Range::new(1,4);
    let d = Range::new(4,5);
    c.merge(d);
    assert_eq!(c,Range::new(1,5));
  }
  
  #[test]
  fn test_right_extend_merge() {
    let mut a = Range::new(4,5);
    let b = Range::new(5,6);
    a.merge(b);
    assert_eq!(a,Range::new(4,6));
    
    let mut c = Range::new(4,5);
    let d = Range::new(5,6);
    c.merge(d);
    assert_eq!(c,Range::new(4,6));
  }
  
  #[test]
  fn test_left_extend_merge_part_2_demo() {
    let mut a = Range::new(10,14);
    let b = Range::new(12,18);
    a.merge(b);
    assert_eq!(a,Range::new(10,18));
    
    let c = Range::new(16,20);
    a.merge(c);
    assert_eq!(a,Range::new(10,20));
  }
  
  
  #[test]
  fn test_part_2() { 
    assert_eq!(part_2(file_to_string("inputs/demo")), 14);
    //assert_eq!(part_2(file_to_string("inputs/input")), 6671);
    assert!(part_2(file_to_string("inputs/input")) > 303238030370608);
    assert!(part_2(file_to_string("inputs/input")) < 366732210252741);
    assert_eq!(part_2(file_to_string("inputs/input")),366181852921027);
  }
  
}
