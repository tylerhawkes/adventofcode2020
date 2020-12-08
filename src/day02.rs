struct Password {
  min: u8,
  max: u8,
  c: char,
  password: String,
}

#[aoc_generator(day2)]
fn day2_parser(input: &str) -> Vec<Password> {
  input
    .lines()
    .map(|l| {
      let splits = l.split_ascii_whitespace().collect::<Vec<_>>();
      let min_max = splits[0].split("-").map(|n| n.parse().unwrap()).collect::<Vec<u8>>();
      let character = splits[1].split(":").next().unwrap().chars().next().unwrap();
      let password = splits[2].to_string();
      Password {
        min: min_max[0],
        max: min_max[1],
        c: character,
        password,
      }
    })
    .collect()
}

#[aoc(day2, part1)]
fn day2_part1(passwords: &[Password]) -> usize {
  passwords
    .iter()
    .filter(|p| {
      let count = p.password.chars().filter(|c| *c == p.c).count();
      count >= p.min as usize && count <= p.max as usize
    })
    .count()
}

#[aoc(day2, part2)]
fn day2_part2(passwords: &[Password]) -> usize {
  passwords
    .iter()
    .filter(|p| (p.password.as_bytes()[p.min as usize - 1] as char == p.c) ^ (p.password.as_bytes()[p.max as usize - 1] as char == p.c))
    .count()
}
