use std::io;

use crate::shared::*;

struct Grid {
  cells: Vec<Vec<bool>>,
  width: usize,
  height: usize,
}

impl Clone for Grid {
  fn clone(&self) -> Self {
    Grid {
      cells: self.cells.clone(),
      width: self.width,
      height: self.height,
    }
  }
}

fn parse_grid(input: &Vec<String>) -> Result<Grid, String> {
  let mut width = None;
  let cells = input
    .iter()
    .map(|line| {
      width = match width {
        None => Some(line.len()),
        Some(w) => {
          if w != line.len() {
            return Err(format!(
              "Inconsistent line widths: expected {}, got {}",
              w,
              line.len()
            ));
          }
          Some(w)
        }
      };
      Ok(line.chars().map(|char| char == '@').collect::<Vec<bool>>())
    })
    .collect::<Result<Vec<Vec<bool>>, String>>()?;

  let height = cells.len();

  match width {
    Some(w) => Ok(Grid {
      cells,
      width: w,
      height,
    }),
    None => Err("Empty input".to_string()),
  }
}

fn num_adj_paper(x: usize, y: usize, grid: &Grid) -> usize {
  vec![
    (x.checked_sub(1), y.checked_sub(1)),
    (Some(x), y.checked_sub(1)),
    (x.checked_add(1), y.checked_sub(1)),
    (x.checked_sub(1), Some(y)),
    (x.checked_add(1), Some(y)),
    (x.checked_sub(1), y.checked_add(1)),
    (Some(x), y.checked_add(1)),
    (x.checked_add(1), y.checked_add(1)),
  ]
  .iter()
  .filter_map(|(maybex, maybey)| maybex.and_then(|x| maybey.map(|y| (x, y))))
  .filter(|(nx, ny)| *nx < grid.width && *ny < grid.height)
  .map(|(nx, ny)| if grid.cells[ny][nx] { 1 } else { 0 })
  .sum()
}

fn candidate_rolls(grid: &Grid) -> Vec<(usize, usize)> {
  (0..grid.height)
    .map(move |y| (0..grid.width).map(move |x| (x, y)))
    .flatten()
    .filter(|(x, y)| grid.cells[*y][*x] && num_adj_paper(*x, *y, &grid) < 4)
    .collect()
}

mod puzzle1 {

  use super::{Grid, candidate_rolls};

  pub fn solve(grid: &Grid) -> usize {
    candidate_rolls(grid).len()
  }
}

mod puzzle2 {
  use super::{Grid, candidate_rolls};

  pub fn iterate_grid(grid: &mut Grid) -> usize {
    let candidates = candidate_rolls(grid);
    for (x, y) in candidates.iter() {
      grid.cells[*y][*x] = false;
    }
    candidates.len()
  }

  pub fn solve(grid: &mut Grid) -> usize {
    let mut count = 0;
    while candidate_rolls(grid).len() > 0 {
      count += iterate_grid(grid);
    }
    count
  }
}

pub fn run() -> io::Result<()> {
  let input_lines = load_input_lines("day4.txt")?;
  let mut inputs = parse_grid(&input_lines).expect("Unable to parse input file");

  println!("Day 4 Puzzle 1: {}", puzzle1::solve(&inputs));
  println!("Day 4 Puzzle 2: {}", puzzle2::solve(&mut inputs));

  Ok(())
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::sync::LazyLock;

  static SAMPLE_INPUT: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
      "..@@.@@@@.".to_string(),
      "@@@.@.@.@@".to_string(),
      "@@@@@.@.@@".to_string(),
      "@.@@@@..@.".to_string(),
      "@@.@@@@.@@".to_string(),
      ".@@@@@@@.@".to_string(),
      ".@.@.@.@@@".to_string(),
      "@.@@@.@@@@".to_string(),
      ".@@@@@@@@.".to_string(),
      "@.@.@@@.@.".to_string(),
    ]
  });

  static SAMPLE_GRID: LazyLock<Grid> = LazyLock::new(|| parse_grid(&SAMPLE_INPUT).unwrap());

  #[test]
  fn test_parse_grid() {
    let ans = parse_grid(&SAMPLE_INPUT).unwrap();
    assert!(ans.height == 10);
    assert!(ans.width == 10);

    let input = vec![
      "..@@@@.".to_string(),
      "@@@.@.@@".to_string(),
      "@@@@.@.@@".to_string(),
      "@.@@@@..@.".to_string(),
      "@@.@@@.@@".to_string(),
      ".@@@.@.@".to_string(),
      ".@.@@@@".to_string(),
      "@.@@@@@@".to_string(),
      ".@@@@@@@.".to_string(),
      "@.@.@@@.@.".to_string(),
    ];

    let ans = parse_grid(&input);
    assert!(ans.is_err());
  }

  #[test]
  fn test_num_adj_paper() {
    assert_eq!(num_adj_paper(0, 0, &SAMPLE_GRID), 2);
    assert_eq!(num_adj_paper(0, 2, &SAMPLE_GRID), 4);
    assert_eq!(num_adj_paper(4, 4, &SAMPLE_GRID), 8);
  }

  #[test]
  fn test_candidate_rolls() {
    assert_eq!(candidate_rolls(&SAMPLE_GRID).len(), 13);
  }

  #[cfg(test)]
  mod puzzle2 {
    use super::super::puzzle2::*;
    use super::SAMPLE_GRID;

    #[test]
    fn test_solve() {
      let mut grid = SAMPLE_GRID.clone();
      assert_eq!(solve(&mut grid), 43);
    }
  }
}
