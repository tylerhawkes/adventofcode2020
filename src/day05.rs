#[aoc_generator(day5)]
fn day5_generator(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|l| {
      l.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
          let v = match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => unreachable!(),
          };
          v << i
        })
        .sum::<u32>()
    })
    .collect()
}

#[aoc(day5, part1)]
fn day5_part1(seats: &[u32]) -> u32 {
  *seats.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn day5_part2(seats: &[u32]) -> u32 {
  let mut sorted = seats.to_vec();
  sorted.sort();
  sorted
    .iter()
    .copied()
    .zip(sorted.iter().copied().skip(1))
    .find(|(l, r)| r - l > 1)
    .map(|(l, _)| l + 1)
    .unwrap()
}
