// #[aoc_generator(day13)]
// fn day13_generator(input: &str) -> Vec<u8> {
//
// }

#[aoc(day13, part1)]
fn day13_part1(input: &str) -> u64 {
  let mut lines = input.lines();
  let earliest_time = lines.next().unwrap().parse::<u64>().unwrap();
  let buses = lines
    .next()
    .unwrap()
    .split(',')
    .filter(|c| c != &"x")
    .map(|i| i.parse().unwrap())
    .collect::<Vec<u64>>();
  let x = buses
    .iter()
    .copied()
    .map(|b| (b, b - earliest_time % b))
    .min_by_key(|(_, t)| *t)
    .unwrap();
  dbg!((x, buses));
  x.0 * x.1
}

#[aoc(day13, part2)]
fn day13_part2(input: &str) -> u64 {
  let lines = input.lines();
  // let mut earliest_time = lines.next().unwrap().parse::<u64>().unwrap();

  let buses = lines
    .skip(1)
    .next()
    .unwrap()
    .split(',')
    .enumerate()
    .filter(|(_, c)| c != &"x")
    .map(|(i, n)| (i, n.parse().unwrap()))
    .collect::<Vec<(usize, u64)>>();
  dbg!(&buses);
  buses.iter().copied().map(|(_, i)| i).product()
}
