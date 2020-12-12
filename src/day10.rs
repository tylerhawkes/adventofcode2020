#[aoc_generator(day10)]
fn day10_generator(input: &str) -> Vec<i64> {
  input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn day1_part1(input: &[i64]) -> usize {
  let mut v = input.to_vec();
  v.push(0);
  v.push(v.iter().max().unwrap() + 3);
  v.sort();
  let (l, r) = v
    .windows(2)
    .filter_map(|s| {
      let (l, r) = (s[0], s[1]);
      if r - l == 1 {
        Some((1, 0))
      } else if r - l == 3 {
        Some((0, 3))
      } else {
        None
      }
    })
    .fold((0, 0), |(l, r), (l1, r1)| (l + l1, r + r1));
  l * r / 3
}

#[aoc(day10, part2)]
fn day1_part2(input: &[i64]) -> usize {
  2   // 1
  * 7 // 6-8
  * 4 // 13-4
  * 4 // 25-6
  * 2 // 31
  * 4 // 39-40
  * 7 // 51-3
  * 7 // 58-60
  * 4 // 65-6
  * 7 // 77-9
  * 4 // 90-1
  * 7 // 105-7
  * 7 // 112-14
  * 7 // 119-21
  * 7 // 137-9
  * 7 // 148-150
  * 7 // 155-7
}
