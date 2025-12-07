use std::str::FromStr;

use crate::shared::*;

fn parse_battery_bank(input: &str) -> Result<Vec<u64>, <u64 as FromStr>::Err> {
  input
    .trim()
    .chars()
    .map(|char| char.to_string().parse::<u64>())
    .collect()
}

mod puzzle1 {
  use std::cmp::Ordering;

  pub fn maximize_bank(bank: &[u64]) -> Result<u64, &'static str> {
    let (idx, first_digit) = (0..)
      .zip(bank[0..=(bank.len() - 2)].iter())
      .max_by(|(i1, x), (i2, y)| match x.cmp(y) {
        Ordering::Equal => i2.cmp(i1),
        other => other,
      })
      .ok_or("Must have at least 2 elements in a battery bank")?;

    let second_digit = bank[(idx + 1)..]
      .iter()
      .max()
      .ok_or("Must have at least 2 elements in a battery bank")?;

    Ok(first_digit * 10 + second_digit)
  }

  pub fn solve(input: &Vec<Vec<u64>>) -> Result<u64, &'static str> {
    let joltages = input
      .iter()
      .map(|bank| maximize_bank(bank))
      .collect::<Result<Vec<_>, &'static str>>()?;

    Ok(joltages.iter().sum())
  }
}

mod puzzle2 {
  use std::cmp::Ordering;

  pub fn maximize_bank(bank: &[u64]) -> Result<u64, &'static str> {
    let indexed = (0..).zip(bank).collect::<Vec<_>>();
    let mut start = 0;
    let mut end = bank.len() - 12;
    let mut digits = vec![];
    for _ in 1..=12 {
      let selected = indexed[start..=end]
        .iter()
        .max_by(|(i1, x), (i2, y)| match x.cmp(y) {
          Ordering::Equal => i2.cmp(i1),
          other => other,
        });

      let (idx, digit) = match selected {
        Some((i, d)) => (*i, **d),
        None => return Err("Must have at least 12 elements in a battery bank"),
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

  pub fn solve(input: &Vec<Vec<u64>>) -> Result<u64, &'static str> {
    let joltages = input
      .iter()
      .map(|bank| maximize_bank(bank))
      .collect::<Result<Vec<_>, &'static str>>()?;

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
      assert_eq!(maximize_bank(&bank).unwrap(), 98);
      let bank = parse_battery_bank("811111111111119").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 89);
      let bank = parse_battery_bank("234234234234278").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 78);
      let bank = parse_battery_bank("818181911112111").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 92);
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
      assert_eq!(maximize_bank(&bank).unwrap(), 987654321111);
      let bank = parse_battery_bank("811111111111119").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 811111111119);
      let bank = parse_battery_bank("234234234234278").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 434234234278);
      let bank = parse_battery_bank("818181911112111").unwrap();
      assert_eq!(maximize_bank(&bank).unwrap(), 888911112111);
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
