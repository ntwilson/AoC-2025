use crate::shared;

#[derive(Debug)]
struct MathProblem {
  operands: Vec<i64>,
  operator: fn(i64, i64) -> i64,
  monoid_identity: i64,
}

fn add(x: i64, y: i64) -> i64 {
  x + y
}
fn multiply(x: i64, y: i64) -> i64 {
  x * y
}
fn subtract(x: i64, y: i64) -> i64 {
  x - y
}
fn divide(x: i64, y: i64) -> i64 {
  x / y
}

mod puzzle1 {
  use super::MathProblem;
  use super::{add, divide, multiply, subtract};

  pub fn parse_input(input: &Vec<String>) -> Result<Vec<MathProblem>, String> {
    let all_operands = input[..input.len() - 1]
      .iter()
      .map(|line| {
        line
          .trim()
          .split_whitespace()
          .map(|token| {
            token
              .trim()
              .parse::<i64>()
              .map_err(|e| format!("Failed to parse operand '{}': {}", token, e))
          })
          .collect::<Result<Vec<i64>, _>>()
      })
      .collect::<Result<Vec<Vec<i64>>, String>>()?;

    let operator_line = input[input.len() - 1]
      .split_whitespace()
      .map(|token| match token.trim() {
        "+" => Ok((add as fn(i64, i64) -> i64, 0)),
        "*" => Ok((multiply as fn(i64, i64) -> i64, 1)),
        "-" => Ok((subtract as fn(i64, i64) -> i64, 0)),
        "/" => Ok((divide as fn(i64, i64) -> i64, 1)),
        _ => Err(format!("Unknown operator '{}'", token)),
      })
      .collect::<Result<Vec<(fn(i64, i64) -> i64, i64)>, String>>()?;

    Ok(
      (0..operator_line.len())
        .map(|i| {
          let operands = all_operands.iter().map(|ops| ops[i]).collect();
          let (operator, monoid_identity) = operator_line[i];
          MathProblem {
            operands,
            operator,
            monoid_identity,
          }
        })
        .collect::<Vec<MathProblem>>(),
    )
  }
}

mod puzzle2 {
  use super::MathProblem;
  use super::{add, divide, multiply, subtract};

  pub fn parse_input(input: &Vec<String>) -> Result<Vec<MathProblem>, String> {
    let operator_line = &input[input.len() - 1];

    let operators = operator_line
      .split_whitespace()
      .map(|token| match token.trim() {
        "+" => Ok((add as fn(i64, i64) -> i64, 0)),
        "*" => Ok((multiply as fn(i64, i64) -> i64, 1)),
        "-" => Ok((subtract as fn(i64, i64) -> i64, 0)),
        "/" => Ok((divide as fn(i64, i64) -> i64, 1)),
        _ => Err(format!("Unknown operator '{}'", token)),
      })
      .collect::<Result<Vec<(fn(i64, i64) -> i64, i64)>, String>>()?;

    let operand_lines = &input[..input.len() - 1];
    let mut all_operands = vec![];
    let mut current_problem = vec![];
    for col in 0..operator_line.len() {
      let num_str = operand_lines
        .iter()
        .map(|line| line.chars().nth(col).unwrap_or(' '))
        .collect::<String>()
        .trim()
        .to_string();

      if num_str.is_empty() {
        all_operands.push(current_problem);
        current_problem = vec![];
      } else {
        let num = num_str
          .parse::<i64>()
          .map_err(|e| format!("Failed to parse operand in column {}: {}", col, e))?;

        current_problem.push(num);
      }
    }

    if current_problem.len() > 0 {
      all_operands.push(current_problem);
    }

    Ok(
      all_operands
        .iter()
        .zip(operators)
        .map(|(operand, (operator, identity))| MathProblem {
          operands: operand.clone(),
          operator,
          monoid_identity: identity,
        })
        .collect(),
    )
  }
}

fn solve(input: &Vec<MathProblem>) -> i64 {
  input
    .iter()
    .map(|problem| {
      problem
        .operands
        .iter()
        .fold(problem.monoid_identity, |acc, &op| {
          (problem.operator)(acc, op)
        })
    })
    .sum()
}

pub fn run() {
  let input_lines = shared::load_input_lines("day6.txt").unwrap();
  let puzzle_input1 = puzzle1::parse_input(&input_lines).unwrap();

  let result1 = solve(&puzzle_input1);
  println!("Day 6, Puzzle 1: {}", result1);
  let puzzle_input2 = puzzle2::parse_input(&input_lines).unwrap();

  let result2 = solve(&puzzle_input2);
  println!("Day 6, Puzzle 2: {}", result2);
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::sync::LazyLock;

  static TEST_INPUT: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
      "123 328  51 64 ".to_string(),
      " 45 64  387 23 ".to_string(),
      "  6 98  215 314".to_string(),
      "*   +   *   +  ".to_string(),
    ]
  });

  mod puzzle1 {
    use super::super::puzzle1::*;
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_input() {
      let parsed = parse_input(&TEST_INPUT).unwrap();
      assert_eq!(parsed.len(), 4);
      assert_eq!(parsed[0].operands, vec![123, 45, 6]);
      assert_eq!(parsed[1].operands, vec![328, 64, 98]);
    }

    #[test]
    fn test_solve() {
      let parsed = parse_input(&TEST_INPUT).unwrap();
      assert_eq!(solve(&parsed), 4277556);
    }
  }

  mod puzzle2 {
    use super::super::puzzle2::*;
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_input() {
      let parsed = parse_input(&TEST_INPUT).unwrap();
      assert_eq!(parsed.len(), 4);
      assert_eq!(parsed[0].operands, vec![1, 24, 356]);
      assert_eq!(parsed[1].operands, vec![369, 248, 8]);
    }

    #[test]
    fn test_solve() {
      let parsed = parse_input(&TEST_INPUT).unwrap();
      assert_eq!(solve(&parsed), 3263827);
    }
  }
}
