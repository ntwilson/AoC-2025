use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};

fn load_input() -> io::Result<String> {
  let file = File::open("../../puzzleInput/day2.txt").expect("Failed to open input file");
  let mut reader = BufReader::new(file);
  let mut buf = String::new();
  reader.read_to_string(&mut buf)?;
  Ok(buf)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
  let pairs = input.split(",");
  pairs
    .map(|pair| {
      let mut nums = pair.trim().split("-");
      let first = nums
        .next()
        .expect(format!("Unable to parse pair '{}' with the pattern '_-_'.", pair).as_str())
        .parse::<i64>()
        .expect(format!("Unable to parse '{}' as a pair of numbers.", pair).as_str());
      let second = nums
        .next()
        .expect(format!("Unable to parse pair '{}' with the pattern '_-_'.", pair).as_str())
        .parse::<i64>()
        .expect(format!("Unable to parse '{}' as a pair of numbers.", pair).as_str());

      (first, second)
    })
    .collect()
}

mod puzzle1 {

  fn is_repeat_pair(i: i64) -> bool {
    let s = i.to_string();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len % 2 == 1 {
      return false;
    }
    let forwards = chars[0..len / 2].to_vec();
    let backwards = chars[(len + 1) / 2..].iter();
    forwards.iter().zip(backwards).all(|(a, b)| a == b)
  }

  pub fn invalid_ids(l: i64, r: i64) -> Vec<i64> {
    (l..=r).filter(|i| is_repeat_pair(*i)).collect()
  }

  pub fn solve(input: &Vec<(i64, i64)>) -> i64 {
    input.iter().flat_map(|(a, b)| invalid_ids(*a, *b)).sum()
  }
}

mod puzzle2 {
  pub fn is_repeat_any_size(i: i64) -> bool {
    let s = i.to_string();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut chunk_sizes = 1..=(len / 2);
    chunk_sizes.any(|size| {
      let mut chunks = chars.chunks(size);
      match chunks.next() {
        None => false,
        Some(first_window) => chunks.all(|w| w == first_window),
      }
    })
  }

  pub fn invalid_ids(l: i64, r: i64) -> Vec<i64> {
    (l..=r).filter(|i| is_repeat_any_size(*i)).collect()
  }

  pub fn solve(input: &Vec<(i64, i64)>) -> i64 {
    input.iter().flat_map(|(a, b)| invalid_ids(*a, *b)).sum()
  }
}

pub fn run() {
  let input = load_input().unwrap();
  let parsed_input = parse_input(&input);
  let puz1 = puzzle1::solve(&parsed_input);
  let puz2 = puzzle2::solve(&parsed_input);
  println!("Day 2 Puzzle 1: {}", puz1);
  println!("Day 2 Puzzle 2: {}", puz2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_given_inputs_individually_for_puzzle_1() {
    let result = puzzle1::invalid_ids(11, 22);
    assert_eq!(result, vec![11, 22]);
    let result = puzzle1::invalid_ids(95, 115);
    assert_eq!(result, vec![99]);
    let result = puzzle1::invalid_ids(998, 1012);
    assert_eq!(result, vec![1010]);
  }

  #[test]
  fn solve_given_input_for_puzzle_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let parsed_input = parse_input(input);
    let result = puzzle1::solve(&parsed_input);
    assert_eq!(result, 1227775554);
  }

  #[test]
  fn solve_given_inputs_individually_for_puzzle_2() {
    let result = puzzle2::invalid_ids(11, 22);
    assert_eq!(result, vec![11, 22]);
    let result = puzzle2::invalid_ids(95, 115);
    assert_eq!(result, vec![99, 111]);
    let result = puzzle2::invalid_ids(998, 1012);
    assert_eq!(result, vec![999, 1010]);
  }

  #[test]
  fn solve_given_input_for_puzzle_2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let parsed_input = parse_input(input);
    let result = puzzle2::solve(&parsed_input);
    assert_eq!(result, 4174379265);
  }
}
