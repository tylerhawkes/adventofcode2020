use std::collections::HashSet;

#[aoc(day6, part1)]
fn day6_part1(input: &str) -> i32 {
  let mut unique = HashSet::new();
  let mut answers = Vec::new();
  input.lines().for_each(|l| {
    if l.is_empty() {
      answers.push(unique.len() as u8);
      unique.clear();
    }
    l.chars().for_each(|c| {
      unique.insert(c);
    });
  });
  answers.push(unique.len() as u8);
  answers.iter().map(|u| *u as i32).sum::<i32>()
}

#[aoc(day6, part2)]
fn day6_part2(input: &str) -> u32 {
  let mut unique = u32::MAX;
  let mut answers = Vec::new();
  input.lines().for_each(|l| {
    if l.is_empty() {
      answers.push(unique.count_ones());
      unique = u32::MAX;
      return;
    }
    let mut unique_row = 0;
    l.chars().for_each(|c| unique_row |= 1_u32 << (c as u8 - b'a'));
    unique = unique & unique_row;
  });
  answers.push(unique.count_ones());
  answers.iter().sum::<u32>()
}
