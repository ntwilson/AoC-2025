use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load_input() -> io::Result<Vec<String>> {
  let file = File::open("../../puzzleInput/day1.txt").expect("Failed to open input file");
  let reader = BufReader::new(file);
  reader.lines().collect::<io::Result<Vec<String>>>()
}

#[derive(Debug)]
enum Input {
  Left(i16),
  Right(i16),
}

impl std::fmt::Display for Input {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Input::Left(n) => write!(f, "L{}", n),
      Input::Right(n) => write!(f, "R{}", n),
    }
  }
}

fn parse_input(line: &str) -> Result<Input, <i16 as FromStr>::Err> {
  if line.chars().next() == Some('L') {
    Ok(Input::Left(line[1..].parse()?))
  } else {
    Ok(Input::Right(line[1..].parse()?))
  }
}

fn positive_mod(a: i16, b: i16) -> i16 {
  if a >= 0 {
    a % b
  } else {
    positive_mod(a + b, b)
  }
}

fn solve_puzzle1(inputs: &Vec<Input>) -> usize {
  let starting_pos = 50;
  let ending_spots = inputs.iter().scan(starting_pos, |state, element| {
    *state = match element {
      Input::Left(n) => positive_mod(*state - n, 100),
      Input::Right(n) => positive_mod(*state + n, 100),
    };

    Some(*state)
  });

  ending_spots.filter(|x| *x == 0).count()
}

fn solve_puzzle2(inputs: &Vec<Input>) -> i16 {
  let starting_pos = 50;

  let zero_traversals = inputs.iter().scan(starting_pos, |state, element| {
    let zeros = match element {
      Input::Right(n) => (n + *state) / 100,
      Input::Left(n) => max(n - *state + (if *state == 0 { 0 } else { 100 }), 0) / 100,
    };

    *state = match element {
      Input::Right(n) => positive_mod(*state + n, 100),
      Input::Left(n) => positive_mod(*state - n, 100),
    };

    Some(zeros)
  });

  zero_traversals.sum()
}

fn main() -> io::Result<()> {
  let input_lines = load_input()?;
  let inputs = input_lines
    .iter()
    .map(|line| parse_input(line))
    .collect::<Result<Vec<_>, _>>()
    .expect("Unable to parse input file");

  println!("Day 1 Puzzle 1: {}", solve_puzzle1(&inputs));
  println!("Day 1 Puzzle 2: {}", solve_puzzle2(&inputs));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(solve_puzzle1(&vec![]), 0);
  }

  #[test]
  fn mod_works_as_i_think() {
    assert_eq!(positive_mod(0, 100), 0);
    assert_eq!(positive_mod(99, 100), 99);
    assert_eq!(positive_mod(100, 100), 0);
    assert_eq!(positive_mod(-1, 100), 99);
    assert_eq!(positive_mod(-100, 100), 0);
    assert_eq!(positive_mod(-101, 100), 99);
  }

  #[test]
  fn solve_given_input() {
    let given_inputs = vec![
      "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    let parsed_inputs = given_inputs
      .iter()
      .map(|line| parse_input(line).unwrap())
      .collect::<Vec<_>>();

    assert_eq!(solve_puzzle1(&parsed_inputs), 3);
    assert_eq!(solve_puzzle2(&parsed_inputs), 6);
  }
}
