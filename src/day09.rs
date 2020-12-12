#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<u64> {
  input.lines().map(|l| l.parse().unwrap()).collect()
}

fn adds_to_i(i: u64, s: &[u64]) -> bool {
  assert_eq!(s.len(), 25);
  s.iter()
    .copied()
    .enumerate()
    .any(|(j, x)| s[j + 1..].iter().copied().any(move |y| x != y && x + y == i))
}

#[aoc(day9, part1)]
fn day9_part1(input: &[u64]) -> u64 {
  input.windows(26).find(|s| !adds_to_i(s[25], &s[..25])).unwrap()[25]
}

#[aoc(day9, part2)]
fn day9_part2(input: &[u64]) -> u64 {
  let target = day9_part1(input);
  'outer: for i in 0..input.len() {
    let slice = &input[i..];
    let mut sum = 0;
    for j in 0..slice.len() {
      sum += slice[j];
      if sum == target {
        let slice = &slice[..=j];
        return slice.iter().min().unwrap() + slice.iter().max().unwrap();
      } else if sum > target {
        continue 'outer;
      }
    }
  }
  unreachable!()
}
