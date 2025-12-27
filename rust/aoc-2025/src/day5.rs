use std::io;

use crate::shared;

struct PuzzleInput {
  fresh_ranges: Vec<(i64, i64)>,
  ingredients: Vec<i64>,
}

fn parse_input(input: &Vec<String>) -> Result<PuzzleInput, String> {
  let (_, split_index) = input
    .iter()
    .zip(0..)
    .find(|(item, _)| item.trim().is_empty())
    .ok_or("No split found between ranges and ingredients.".to_string())?;

  let mut fresh_ranges = input.clone();
  let ingredients = fresh_ranges.split_off(split_index);
  let fresh_ranges = fresh_ranges;

  let ingredients = ingredients
    .iter()
    .filter(|line| !line.trim().is_empty())
    .map(|line| {
      line
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse ingredient '{}': {}", line, e))
    })
    .collect::<Result<_, _>>()?;

  let fresh_ranges = fresh_ranges
    .iter()
    .filter(|line| !line.trim().is_empty())
    .map(|line| {
      let parts = line
        .trim()
        .split('-')
        .map(|part| part.trim().parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|e| format!("Failed to parse range '{}': {}", line, e))?;

      if parts.len() != 2 {
        return Err(format!("Invalid range format: '{}'", line));
      }

      Ok((parts[0], parts[1]))
    })
    .collect::<Result<_, _>>()?;

  Ok(PuzzleInput {
    fresh_ranges,
    ingredients,
  })
}

mod puzzle1 {
  use super::PuzzleInput;
  pub fn solve(input: &PuzzleInput) -> usize {
    input
      .ingredients
      .iter()
      .filter(|ingredient| {
        input
          .fresh_ranges
          .iter()
          .any(|(start, end)| *ingredient >= start && *ingredient <= end)
      })
      .count()
  }
}

mod puzzle2 {
  use super::PuzzleInput;
  fn union_range(range1: (i64, i64), range2: (i64, i64)) -> Option<(i64, i64)> {
    if range1.1 < range2.0 || range2.1 < range1.0 {
      None
    } else {
      Some((range1.0.min(range2.0), range1.1.max(range2.1)))
    }
  }

  pub fn solve(input: &PuzzleInput) -> usize {
    let mut ranges = input.fresh_ranges.clone();
    ranges.sort();

    let mut merged_ranges = vec![];
    {
      let mut current_range = None;
      for range in ranges {
        match current_range {
          None => current_range = Some(range),
          Some(rng) => match union_range(rng, range) {
            Some(unioned) => current_range = Some(unioned),
            None => {
              merged_ranges.push(rng);
              current_range = Some(range);
            }
          },
        }
      }

      if let Some(current_range) = current_range {
        merged_ranges.push(current_range);
      }
    }

    let merged_ranges = merged_ranges;

    merged_ranges
      .iter()
      .map(|(start, end)| end - start + 1)
      .sum::<i64>() as usize
  }
}

pub fn run() -> io::Result<()> {
  let input_lines = shared::load_input_lines("day5.txt")?;
  let puzzle_input = parse_input(&input_lines).unwrap();

  let result1 = puzzle1::solve(&puzzle_input);
  println!("Day 5, Puzzle 1: {}", result1);
  let result2 = puzzle2::solve(&puzzle_input);
  println!("Day 5, Puzzle 2: {}", result2);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::LazyLock;

  static SAMPLE_INPUT: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
      "3-5".to_string(),
      "10-14".to_string(),
      "16-20".to_string(),
      "12-18".to_string(),
      "".to_string(),
      "1".to_string(),
      "5".to_string(),
      "8".to_string(),
      "11".to_string(),
      "17".to_string(),
      "32".to_string(),
    ]
  });

  #[test]
  fn test_parse_input() {
    let puzzle_input = parse_input(&SAMPLE_INPUT).unwrap();
    assert_eq!(
      puzzle_input.fresh_ranges,
      vec![(3, 5), (10, 14), (16, 20), (12, 18)]
    );
    assert_eq!(puzzle_input.ingredients, vec![1, 5, 8, 11, 17, 32]);
  }

  #[cfg(test)]
  mod puzzle1 {
    use super::super::puzzle1::*;
    use super::super::*;
    use super::SAMPLE_INPUT;

    #[test]
    fn test_solve() {
      let puzzle_input = parse_input(&SAMPLE_INPUT).unwrap();
      let result = solve(&puzzle_input);
      assert_eq!(result, 3);
    }
  }

  mod puzzle2 {
    use super::super::puzzle2::*;
    use super::super::*;
    use super::SAMPLE_INPUT;

    #[test]
    fn test_solve() {
      let puzzle_input = parse_input(&SAMPLE_INPUT).unwrap();
      let result = solve(&puzzle_input);
      assert_eq!(result, 14);
    }
  }
}
