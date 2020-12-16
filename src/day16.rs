use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct TicketInfo {
  my_ticket: Vec<u64>,
  other_tickets: Vec<Vec<u64>>,
  rules: HashMap<String, Vec<RangeInclusive<u64>>>,
}

impl TicketInfo {
  fn is_valid(&self, u: u64) -> bool {
    self.rules.values().flat_map(|v| v.iter()).any(|r| r.contains(&u))
  }
}

#[aoc_generator(day16)]
fn day16_generator(input: &str) -> TicketInfo {
  let mut split = input.split("\n\n");
  let rules = split.next().unwrap();
  let rules = rules
    .lines()
    .map(|l| {
      let mut split = l.split(": ");
      let name = split.next().unwrap().to_string();
      let ranges = split.next().unwrap();
      let ranges = ranges
        .split(" or ")
        .map(|r| r.split("-").map(|i| i.parse().unwrap()).collect::<Vec<u64>>())
        .map(|v| RangeInclusive::new(v[0], v[1]))
        .collect();
      (name, ranges)
    })
    .collect();
  let my_ticket = split.next().unwrap();
  let my_ticket = my_ticket
    .lines()
    .skip(1)
    .map(|l| l.split(',').map(|i| i.parse().unwrap()).collect())
    .next()
    .unwrap();
  let other_tickets = split.next().unwrap();
  let other_tickets = other_tickets
    .lines()
    .skip(1)
    .map(|l| l.split(',').map(|i| i.parse().unwrap()).collect())
    .collect();
  TicketInfo {
    my_ticket,
    other_tickets,
    rules,
  }
}

#[aoc(day16, part1)]
fn day16_part1(info: &TicketInfo) -> u64 {
  info
    .other_tickets
    .iter()
    .filter_map(|v| v.iter().copied().find(|i| !info.is_valid(*i)))
    .sum()
}

#[aoc(day16, part2)]
fn day16_part2(info: &TicketInfo) -> u64 {
  let len = info.my_ticket.len();
  let mut valid_tickets = info
    .other_tickets
    .iter()
    .filter(|v| v.iter().copied().all(|i| info.is_valid(i)))
    .collect::<Vec<_>>();
  // println!("Valid tickets: {}, total: {}", valid_tickets.len(), info.other_tickets.len());
  valid_tickets.push(&info.my_ticket);
  let mut possibles = info
    .rules
    .iter()
    .map(|(k, v)| {
      let indexes = (0..len)
        .filter(|i| valid_tickets.iter().all(|ns| v.iter().any(|r| r.contains(&ns[*i as usize]))))
        .map(|i| 1 << i)
        .sum::<usize>();
      (k, indexes)
    })
    .collect::<Vec<_>>();
  let mut found = (1_usize << len) - 1;
  while found > 0 {
    // println!("found: {:#b}", found);
    possibles.iter_mut().for_each(|(_, i)| {
      if i.count_ones() == 1 {
        found &= usize::MAX ^ *i;
      } else {
        *i &= found;
      }
    })
  }
  // dbg!(possibles);
  possibles
    .iter()
    .filter_map(|(k, i)| {
      if k.starts_with("departure") {
        Some(i.trailing_zeros())
      } else {
        None
      }
    })
    .map(|i| info.my_ticket[i as usize])
    .product()
}
