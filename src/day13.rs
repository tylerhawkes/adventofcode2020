// #[aoc_generator(day13)]
// fn day13_generator(input: &str) -> Vec<u8> {
//
// }

#[aoc(day13, part1)]
fn day13_part1(input: &str) -> u64 {
  let mut lines = input.lines();
  let earliest_time = lines.next().unwrap().parse::<u64>().unwrap();
  let x = lines
    .next()
    .unwrap()
    .split(',')
    .filter(|c| c != &"x")
    .map(|i| i.parse::<u64>().unwrap())
    .map(|b| (b, b - earliest_time % b))
    .min_by_key(|(_, t)| *t)
    .unwrap();
  // dbg!((x, buses));
  x.0 * x.1
}

#[aoc(day13, part2)]
fn day13_part2(input: &str) -> u64 {
  let lines = input.lines();

  let mut earliest_time = 0;
  let mut multiple = 1;
  lines
    .skip(1)
    .next()
    .unwrap()
    .split(',')
    .enumerate()
    .filter(|(_, c)| c != &"x")
    .map(|(i, n)| (i as u64, n.parse::<u64>().unwrap()))
    .for_each(|(i, t)| {
      while (earliest_time + i) % t != 0 {
        earliest_time += multiple;
        // println!("earliest_time: {}, {}, {}, {}", earliest_time, (earliest_time + i) % t, i, t);
      }
      multiple *= t;
      // println!("multiple: {}", multiple);
    });
  // println!("{}, {}", earliest_time, multiple);
  earliest_time
}
