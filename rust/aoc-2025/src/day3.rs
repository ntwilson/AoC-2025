use std::cmp::Ordering;
use std::str::FromStr;

use crate::shared::*;

fn parse_battery_bank(input: &str) -> Result<Vec<u64>, <u64 as FromStr>::Err> {
  input
    .trim()
    .chars()
    .map(|char| char.to_string().parse::<u64>())
    .collect()
}

fn maximize_bank(bank: &[u64], size: usize) -> Result<u64, String> {
  let indexed = (0..).zip(bank).collect::<Vec<_>>();
  let mut start = 0;
  let mut end = bank.len() - size;
  let mut digits = vec![];
  for _ in 1..=size {
    let selected = indexed[start..=end]
      .iter()
      .max_by(|(i1, x), (i2, y)| match x.cmp(y) {
        Ordering::Equal => i2.cmp(i1),
        other => other,
      });

    let (idx, digit) = match selected {
      Some((i, d)) => (*i, **d),
      None => {
        return Err(format!(
          "Must have at least {} elements in a battery bank",
          size
        ));
      }
    };

    digits.push(digit);
    start = idx + 1;
    end += 1;
  }

  Ok(
    digits
      .iter()
      .rev()
      .zip(0..)
      .map(|(digit, power)| digit * 10u64.pow(power))
      .sum(),
  )
}

mod puzzle1 {

  use super::maximize_bank;

  pub fn solve(input: &Vec<Vec<u64>>) -> Result<u64, String> {
    let joltages = input
      .iter()
      .map(|bank| maximize_bank(bank, 2))
      .collect::<Result<Vec<_>, _>>()?;

    Ok(joltages.iter().sum())
  }
}

mod puzzle2 {

  use super::maximize_bank;

  pub fn solve(input: &Vec<Vec<u64>>) -> Result<u64, String> {
    let joltages = input
      .iter()
      .map(|bank| maximize_bank(bank, 12))
      .collect::<Result<Vec<_>, _>>()?;

    Ok(joltages.iter().sum())
  }
}

pub fn run() {
  let raw_input = load_input_lines("day3.txt").unwrap();
  let input = raw_input
    .iter()
    .map(|line| parse_battery_bank(line).unwrap())
    .collect::<Vec<_>>();

  let result = puzzle1::solve(&input).unwrap();
  println!("Day 3 Puzzle 1: {}", result);
  let result = puzzle2::solve(&input).unwrap();
  println!("Day 3 Puzzle 2: {}", result);
}

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod puzzle1 {
    use super::super::puzzle1::*;
    use super::super::*;
    #[test]
    fn test_maximize_bank() {
      let bank = parse_battery_bank("987654321111111").unwrap();
      assert_eq!(maximize_bank(&bank, 2).unwrap(), 98);
      let bank = parse_battery_bank("811111111111119").unwrap();
      assert_eq!(maximize_bank(&bank, 2).unwrap(), 89);
      let bank = parse_battery_bank("234234234234278").unwrap();
      assert_eq!(maximize_bank(&bank, 2).unwrap(), 78);
      let bank = parse_battery_bank("818181911112111").unwrap();
      assert_eq!(maximize_bank(&bank, 2).unwrap(), 92);
    }

    #[test]
    fn solve_given_input() {
      let raw_input = vec![
        ("987654321111111"),
        ("811111111111119"),
        ("234234234234278"),
        ("818181911112111"),
      ];

      let input = raw_input
        .iter()
        .map(|line| parse_battery_bank(line).unwrap())
        .collect::<Vec<Vec<u64>>>();

      assert_eq!(solve(&input).unwrap(), 357);
    }
  }

  #[cfg(test)]
  mod puzzle2 {
    use super::super::puzzle2::*;
    use super::super::*;
    #[test]
    fn test_maximize_bank() {
      let bank = parse_battery_bank("987654321111111").unwrap();
      assert_eq!(maximize_bank(&bank, 12).unwrap(), 987654321111);
      let bank = parse_battery_bank("811111111111119").unwrap();
      assert_eq!(maximize_bank(&bank, 12).unwrap(), 811111111119);
      let bank = parse_battery_bank("234234234234278").unwrap();
      assert_eq!(maximize_bank(&bank, 12).unwrap(), 434234234278);
      let bank = parse_battery_bank("818181911112111").unwrap();
      assert_eq!(maximize_bank(&bank, 12).unwrap(), 888911112111);
    }

    #[test]
    fn solve_given_input() {
      let raw_input = vec![
        ("987654321111111"),
        ("811111111111119"),
        ("234234234234278"),
        ("818181911112111"),
      ];

      let input = raw_input
        .iter()
        .map(|line| parse_battery_bank(line).unwrap())
        .collect::<Vec<Vec<u64>>>();

      assert_eq!(solve(&input).unwrap(), 3121910778619);
    }
  }
}
