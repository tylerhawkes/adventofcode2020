use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default)]
struct BagGraph {
  parents: HashMap<String, Vec<String>>,
  children: HashMap<String, Vec<(String, usize)>>,
}

#[aoc_generator(day7)]
fn day7_generator(input: &str) -> BagGraph {
  let mut bag = BagGraph::default();
  let input = input.replace("bags", "").replace("bag", "").replace(".", "");
  input.lines().for_each(|l| {
    let mut split = l.split(" contain ");
    let parent = split.next().unwrap().trim();
    let children = split.next().unwrap();
    let children = children
      .split(",")
      .map(|c| {
        let c = c.trim();
        if c == "no other" {
          return ("".to_string(), 0);
        }
        let num_split = c.find(char::is_whitespace).unwrap();
        let count = c[..num_split]
          .parse::<usize>()
          .unwrap_or_else(|_| panic!("Tried unwrapping on {:?}", &c[..num_split]));
        let color = c[num_split + 1..].trim().to_string();
        (color, count)
      })
      .collect::<Vec<_>>();
    children.iter().for_each(|(color, _)| match bag.parents.entry(color.to_string()) {
      Entry::Occupied(mut o) => {
        o.get_mut().push(parent.to_string());
      }
      Entry::Vacant(v) => {
        v.insert(vec![parent.to_string()]);
      }
    });
    bag.children.insert(parent.to_string(), children);
  });
  bag
}

#[aoc(day7, part1)]
fn day7_part1(graph: &BagGraph) -> usize {
  fn get(parents: &mut HashSet<String>, graph: &BagGraph, lookup: &str) {
    for parent in graph.parents.get(lookup).unwrap_or(&vec![]).iter() {
      if parents.contains(parent) {
        continue;
      }
      parents.insert(parent.to_string());
      get(parents, graph, parent);
    }
  }
  let mut parents = HashSet::new();
  get(&mut parents, graph, "shiny gold");
  parents.len()
}

#[aoc(day7, part2)]
fn day7_part2(graph: &BagGraph) -> usize {
  fn get(graph: &BagGraph, color: &str) -> usize {
    let bags = graph
      .children
      .get(color)
      .unwrap()
      .iter()
      .map(|(color, count)| {
        if color == "" {
          return *count;
        }
        let bags = get(graph, color) * count;
        bags
      })
      .sum::<usize>() + 1;
    bags
  }
  let bags = get(graph, "shiny gold") - 1;
  bags
}
