#[aoc(day25, part1)]
fn day25_part1(input: &str) -> u64 {
  let keys = input.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<_>>();
  let loop_sizes = keys
    .iter()
    .copied()
    .map(|pub_key| {
      let mut count = 0_u64;
      let mut value = 7;
      while value != pub_key {
        value *= 7;
        value %= 20201227;
        count += 1;
      }
      count
    })
    .collect::<Vec<_>>();
  // dbg!(&keys);
  // dbg!(&loop_sizes);
  let encryption_keys = keys
    .iter()
    .cycle()
    .skip(1)
    .copied()
    .zip(loop_sizes.iter().copied())
    .map(|(pub_key, ls)| {
      let mut val = pub_key;
      for i in 0..ls {
        val *= pub_key;
        val %= 20201227;
      }
      val
    })
    .collect::<Vec<_>>();
  // dbg!(&encryption_keys);
  encryption_keys[0]
}
