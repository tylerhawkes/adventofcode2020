#[aoc_generator(day1)]
pub fn day1_parser(input: &str) -> Vec<i32> {
  input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
  input
    .iter()
    .copied()
    .enumerate()
    .flat_map(|(i, n)| input.iter().copied().skip(i).map(move |j| (j, n)))
    .find(|(i, j)| i + j == 2020)
    .map(|(i, j)| i * j)
    .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
  input
    .iter()
    .copied()
    .enumerate()
    .flat_map(|(i, n)| {
      input
        .iter()
        .copied()
        .skip(i)
        .flat_map(move |j| input.iter().copied().map(move |k| (j, n, k)))
    })
    .find(|(i, j, k)| i + j + k == 2020)
    .map(|(i, j, k)| i * j * k)
    .unwrap()
}
