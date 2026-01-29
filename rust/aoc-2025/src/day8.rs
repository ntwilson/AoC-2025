use std::collections::HashMap;

use crate::shared;

struct GraphManager<T> {
  graphs: Vec<Graph<T>>,
  lookup: HashMap<T, usize>,
}

struct Graph<T> {
  nodes: Vec<GraphNode<T>>,
}
impl<T> Graph<T> {
  fn size(&self) -> usize {
    self.nodes.len()
  }
}

#[derive(Debug, Clone, PartialEq)]
struct GraphNode<T> {
  value: T,
  graph_id: usize,
}

impl<T> GraphManager<T>
where
  T: Eq + std::hash::Hash + Clone,
{
  fn new() -> Self {
    Self {
      graphs: Vec::new(),
      lookup: HashMap::new(),
    }
  }

  fn create_graph(&mut self) -> usize {
    let id = self.graphs.len();
    self.graphs.push(Graph { nodes: Vec::new() });
    id
  }

  fn add_node(&mut self, graph_id: usize, value: T) {
    let node = GraphNode { value, graph_id };
    self.lookup.insert(node.value.clone(), graph_id);
    self.graphs[graph_id].nodes.push(node);
  }

  fn merge_graphs(&mut self, target_id: usize, source_id: usize) {
    if target_id == source_id {
      return;
    }
    let source_nodes = std::mem::take(&mut self.graphs[source_id].nodes);
    for mut node in source_nodes {
      node.graph_id = target_id; // Update the graph reference
      self.lookup.insert(node.value.clone(), target_id);
      self.graphs[target_id].nodes.push(node);
    }

    self.graphs[source_id].nodes.clear();
  }
}

type Junction = (i64, i64, i64);

fn parse_input(input: &Vec<String>) -> Result<Vec<Junction>, String> {
  input
    .iter()
    .map(|line| {
      let parts = line.split(',').collect::<Vec<&str>>();
      let x = parts[0]
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse x: {}", e))?;
      let y = parts[1]
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse y: {}", e))?;
      let z = parts[2]
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse z: {}", e))?;
      Ok((x, y, z))
    })
    .collect()
}

fn distance(a: &Junction, b: &Junction) -> f64 {
  let (ax, ay, az) = *a;
  let (bx, by, bz) = *b;
  f64::sqrt(
    (ax - bx).abs().pow(2) as f64 + (ay - by).abs().pow(2) as f64 + (az - bz).abs().pow(2) as f64,
  )
}

fn closest_pairs(input: &Vec<Junction>) -> Vec<(&Junction, &Junction)> {
  let mut pairs = input
    .iter()
    .flat_map(|a| {
      input
        .iter()
        .filter(move |b| a != *b)
        .map(move |b| (a, b, distance(a, b)))
    })
    .collect::<Vec<_>>();

  pairs.sort_by(|(_, _, a_distance), (_, _, b_distance)| a_distance.total_cmp(b_distance));

  let pairs_with_dupes = pairs.iter().map(|(a, b, _)| (*a, *b));

  pairs_with_dupes
    .zip(0..)
    .filter(|(_, idx)| idx % 2 == 0)
    .map(|(pair, _)| pair)
    .collect::<Vec<(&Junction, &Junction)>>()
}

mod puzzle1 {
  use super::*;

  pub fn solve(input: &Vec<Junction>, n_connections: usize) -> usize {
    let mut manager = GraphManager::<Junction>::new();

    let pairs = closest_pairs(input);

    for pair in pairs.iter().take(n_connections) {
      let (a, b) = pair;

      let a_graph_id = manager.lookup.get(a);
      let b_graph_id = manager.lookup.get(b);

      match (a_graph_id, b_graph_id) {
        (Some(a_graph), Some(b_graph)) => manager.merge_graphs(*a_graph, *b_graph),
        (Some(a_graph), None) => manager.add_node(*a_graph, **b),
        (None, Some(b_graph)) => manager.add_node(*b_graph, **a),
        (None, None) => {
          let new_graph_id = manager.create_graph();
          manager.add_node(new_graph_id, **a);
          manager.add_node(new_graph_id, **b);
        }
      };
    }

    let mut sorted_graphs = manager
      .graphs
      .iter()
      .clone()
      .collect::<Vec<&Graph<Junction>>>();

    sorted_graphs.sort_by(|a, b| b.size().cmp(&a.size()));
    let three_largest_graphs = sorted_graphs[0..3].to_vec();
    let size = three_largest_graphs
      .iter()
      .map(|graph| graph.size())
      .product::<usize>();

    size
  }
}

mod puzzle2 {
  use std::collections::VecDeque;

  use super::*;

  pub fn solve(input: &Vec<Junction>) -> Result<i64, String> {
    let mut manager = GraphManager::<Junction>::new();

    let mut pairs = VecDeque::from(closest_pairs(input));

    let mut last_pair = None;
    while last_pair == None
      || manager
        .graphs
        .iter()
        .any(|graph| 0 < graph.size() && graph.size() < input.len())
    {
      let (a, b) = match pairs.pop_front() {
        Some(pair) => pair,
        None => return Err("No more pairs to process".to_string()),
      };

      last_pair = Some((a, b));

      let a_graph_id = manager.lookup.get(a);
      let b_graph_id = manager.lookup.get(b);

      match (a_graph_id, b_graph_id) {
        (Some(a_graph), Some(b_graph)) => manager.merge_graphs(*a_graph, *b_graph),
        (Some(a_graph), None) => manager.add_node(*a_graph, *b),
        (None, Some(b_graph)) => manager.add_node(*b_graph, *a),
        (None, None) => {
          let new_graph_id = manager.create_graph();
          manager.add_node(new_graph_id, *a);
          manager.add_node(new_graph_id, *b);
        }
      };
    }

    match last_pair {
      Some(((ax, _, _), (bx, _, _))) => Ok(*ax * *bx),
      None => Err("No pairs were processed".to_string()),
    }
  }
}

pub fn run() {
  let input = shared::load_input_lines("day8.txt").unwrap();
  let parsed_input = parse_input(&input).unwrap();

  let result1 = puzzle1::solve(&parsed_input, 1000);
  println!("Day 8, Puzzle 1: {}", result1);

  let result2 = puzzle2::solve(&parsed_input).unwrap();
  println!("Day 8, Puzzle 2: {}", result2);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::LazyLock;
  static TEST_INPUT: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
      "162,817,812".to_string(),
      "57,618,57".to_string(),
      "906,360,560".to_string(),
      "592,479,940".to_string(),
      "352,342,300".to_string(),
      "466,668,158".to_string(),
      "542,29,236".to_string(),
      "431,825,988".to_string(),
      "739,650,466".to_string(),
      "52,470,668".to_string(),
      "216,146,977".to_string(),
      "819,987,18".to_string(),
      "117,168,530".to_string(),
      "805,96,715".to_string(),
      "346,949,466".to_string(),
      "970,615,88".to_string(),
      "941,993,340".to_string(),
      "862,61,35".to_string(),
      "984,92,344".to_string(),
      "425,690,689".to_string(),
    ]
  });

  static PARSED_INPUT: LazyLock<Vec<Junction>> =
    LazyLock::new(|| parse_input(&TEST_INPUT).unwrap());

  #[test]
  fn test_closest_pairs() {
    let result = closest_pairs(&PARSED_INPUT)
      .iter()
      .take(2)
      .map(|(a, b)| ((**a).clone(), (**b).clone()))
      .collect::<Vec<(Junction, Junction)>>();
    assert_eq!(
      result,
      vec![
        ((162, 817, 812), (425, 690, 689)),
        ((162, 817, 812), (431, 825, 988))
      ]
    );
  }

  mod puzzle1 {
    use super::super::puzzle1::solve;
    use super::*;

    #[test]
    fn test_solve_puzzle1() {
      let result = solve(&PARSED_INPUT, 10);
      assert_eq!(result, 40);
    }
  }

  mod puzzle2 {
    use super::super::puzzle2::solve;
    use super::*;

    #[test]
    fn test_solve_puzzle2() {
      let result = solve(&PARSED_INPUT).unwrap();
      assert_eq!(result, 25272);
    }
  }
}
