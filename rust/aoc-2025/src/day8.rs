use std::collections::HashMap;

use crate::shared;

struct GraphManager<T> {
  graphs: Vec<Graph<T>>,
  lookup: HashMap<T, usize>,
}

struct Graph<T> {
  nodes: Vec<GraphNode<T>>,
  size: usize,
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
    self.graphs.push(Graph {
      nodes: Vec::new(),
      size: 0,
    });
    id
  }

  fn add_node(&mut self, graph_id: usize, value: T) {
    let node = GraphNode { value, graph_id };
    self.lookup.insert(node.value.clone(), graph_id);
    self.graphs[graph_id].nodes.push(node);
    self.graphs[graph_id].size += 1;
  }

  fn merge_graphs(&mut self, target_id: usize, source_id: usize) {
    if target_id == source_id {
      return;
    }
    let source_nodes = std::mem::take(&mut self.graphs[source_id].nodes);
    let source_size = self.graphs[source_id].size;

    self.graphs[target_id].size += source_size;
    for mut node in source_nodes {
      node.graph_id = target_id; // Update the graph reference
      self.lookup.insert(node.value.clone(), target_id);
      self.graphs[target_id].nodes.push(node);
    }

    self.graphs[source_id].size = 0;
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

fn closest_n_pairs(input: &Vec<Junction>, n: usize) -> Vec<(&Junction, &Junction)> {
  let mut pairs = input
    .iter()
    .flat_map(|a| {
      input
        .iter()
        .filter(|b| a != *b)
        .map(|b| (a, b, distance(a, b)))
        .collect::<Vec<(&Junction, &Junction, f64)>>()
    })
    .collect::<Vec<_>>();

  pairs.sort_by(|(_, _, a_distance), (_, _, b_distance)| a_distance.total_cmp(b_distance));

  let pairs_with_dupes = pairs.iter().take(n * 2).map(|(a, b, _)| (*a, *b));

  pairs_with_dupes
    .zip(0..)
    .filter(|(_, idx)| idx % 2 == 0)
    .map(|(pair, _)| pair)
    .collect::<Vec<(&Junction, &Junction)>>()
}

fn solve(input: &mut Vec<Junction>, n_connections: usize) -> usize {
  let mut manager = GraphManager::<Junction>::new();

  let pairs = closest_n_pairs(input, n_connections);

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

  sorted_graphs.sort_by(|a, b| b.size.cmp(&a.size));
  let three_largest_graphs = sorted_graphs[0..3].to_vec();
  let size = three_largest_graphs
    .iter()
    .map(|graph| graph.size)
    .product::<usize>();

  size
}

pub fn run() {
  let input = shared::load_input_lines("day8.txt").unwrap();
  let mut parsed_input = parse_input(&input).unwrap();

  let result = solve(&mut parsed_input, 1000);
  println!("Day 8, Result: {}", result);
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
    let result = closest_n_pairs(&PARSED_INPUT, 2)
      .iter()
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

  #[test]
  fn test_solve_puzzle1() {
    let mut input = PARSED_INPUT.clone();
    let result = solve(&mut input, 10);
    assert_eq!(result, 40);
  }
}
