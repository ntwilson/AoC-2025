use crate::shared;

mod puzzle1 {
  use std::collections::HashSet;
  pub fn solve(input: &Vec<String>) -> i32 {
    let mut beams = HashSet::new();
    let mut count = 0;
    for line in input {
      let chars = line.chars().collect::<Vec<char>>();
      for col in 0..line.len() {
        let c = chars[col];
        if c == 'S' {
          beams.insert(col);
        }

        if c == '^' {
          if beams.contains(&col) {
            beams.remove(&col);
            beams.insert(col - 1);
            beams.insert(col + 1);
            count += 1;
          }
        }
      }
    }

    count
  }
}

mod puzzle2 {
  use std::collections::HashMap;

  fn add_beams(beams: &mut HashMap<usize, i64>, col: usize, value: i64) {
    let _ = *beams
      .entry(col)
      .and_modify(|counter| *counter += value)
      .or_insert(value);
  }

  pub fn solve(input: &Vec<String>) -> i64 {
    let mut beams = HashMap::new();
    for line in input {
      let chars = line.chars().collect::<Vec<char>>();
      for col in 0..line.len() {
        let c = chars[col];
        if c == 'S' {
          beams.insert(col, 1);
        }

        if c == '^' {
          if let Some(timelines_at_this_splitter) = beams.remove(&col) {
            add_beams(&mut beams, col - 1, timelines_at_this_splitter);
            add_beams(&mut beams, col + 1, timelines_at_this_splitter);
          }
        }
      }
    }

    beams.values().sum()
  }
}

pub fn run() {
  let input_lines = shared::load_input_lines("day7.txt").unwrap();

  let result1 = puzzle1::solve(&input_lines);
  println!("Day 7, Puzzle 1: {}", result1);

  let result2 = puzzle2::solve(&input_lines);
  println!("Day 7, Puzzle 2: {}", result2);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::LazyLock;
  static TEST_INPUT: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
      ".......S.......".to_string(),
      "...............".to_string(),
      ".......^.......".to_string(),
      "...............".to_string(),
      "......^.^......".to_string(),
      "...............".to_string(),
      ".....^.^.^.....".to_string(),
      "...............".to_string(),
      "....^.^...^....".to_string(),
      "...............".to_string(),
      "...^.^...^.^...".to_string(),
      "...............".to_string(),
      "..^...^.....^..".to_string(),
      "...............".to_string(),
      ".^.^.^.^.^...^.".to_string(),
      "...............".to_string(),
    ]
  });

  #[test]
  fn test_solve_puzzle1() {
    let result = puzzle1::solve(&TEST_INPUT);
    assert_eq!(result, 21);
  }

  #[test]
  fn test_solve_puzzle2() {
    // let test_input = vec![
    //   ".......S.......".to_string(),
    //   "...............".to_string(),
    //   ".......^.......".to_string(),
    //   "...............".to_string(),
    //   "......^.^......".to_string(),
    //   "...............".to_string(),
    //   ".....^.^.^.....".to_string(),
    //   "...............".to_string(),
    // ];
    let result = puzzle2::solve(&TEST_INPUT);
    // println!("Result: {:?}", result);
    assert_eq!(result, 40);
  }
}
