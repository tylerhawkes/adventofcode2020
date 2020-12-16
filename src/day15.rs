use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn answer(input: &str, count: usize) -> usize {
  let mut numbers = input.split(',').map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
  let mut last = numbers.pop().unwrap();
  let mut turns = numbers
    .into_iter()
    .enumerate()
    .map(|(t, n)| (n, t))
    .collect::<HashMap<usize, usize>>();
  for i in turns.len()..count - 1 {
    match turns.entry(last) {
      Entry::Occupied(mut o) => {
        last = i - *o.get();
        *o.get_mut() = i;
      }
      Entry::Vacant(v) => {
        last = 0;
        v.insert(i);
      }
    }
    // println!("Last: {}", last);
  }
  last
}

#[aoc(day15, part1)]
fn day15_part1(input: &str) -> usize {
  answer(input, 2020)
}

#[aoc(day15, part2)]
fn day15_part2(input: &str) -> usize {
  answer(input, 30_000_000)
}
