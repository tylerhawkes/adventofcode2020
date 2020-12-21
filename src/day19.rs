use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule<'a> {
  Final(char),
  Exact(Cow<'a, [u8]>),
  Or(Cow<'a, [u8]>, Cow<'a, [u8]>),
}

#[derive(Debug, Clone)]
struct Evaluator {
  rules: Vec<Rule<'static>>,
  messages: Vec<String>,
}

impl Evaluator {
  fn matches(&self, s: &str) -> bool {
    self.matches_inner(&self.rules[0], s) == Some(s.len())
  }
  fn matches_inner(&self, r: &Rule, s: &str) -> Option<usize> {
    if s.is_empty() {
      return None;
    }
    match r {
      Rule::Final(c) => s.chars().next().filter(|s| s == c).map(|_| 1),
      Rule::Exact(rules) => {
        let mut used = 0;
        for rule in rules.iter().copied() {
          if let Some(matched) = self.matches_inner(&self.rules[rule as usize], &s[used..]) {
            used += matched;
          } else {
            return None;
          }
        }
        Some(used)
      }
      Rule::Or(first, second) => self
        .matches_inner(&Rule::Exact(first.as_ref().into()), s)
        .or_else(|| self.matches_inner(&Rule::Exact(second.as_ref().into()), s)),
    }
  }
}

#[aoc_generator(day19)]
fn day19_generator(input: &str) -> Evaluator {
  let mut split = input.split("\n\n");
  let mut rules = split
    .next()
    .unwrap()
    .lines()
    .map(|l| {
      let mut split = l.split(": ");
      let id = split.next().unwrap().parse::<u8>().unwrap();
      let rule = split.next().unwrap();
      let rule = if rule.contains(" | ") {
        let mut r = rule
          .split(" | ")
          .map(|l| l.split_whitespace().map(|i| i.parse::<u8>().unwrap()).collect());
        Rule::Or(r.next().unwrap(), r.next().unwrap())
      } else if rule.contains('"') {
        Rule::Final(rule.chars().filter(|c| *c != '"').next().unwrap())
      } else {
        Rule::Exact(rule.split_whitespace().map(|i| i.parse::<u8>().unwrap()).collect())
      };
      (id, rule)
    })
    .collect::<Vec<_>>();

  rules.sort_unstable_by_key(|(id, _)| *id);
  let rules = rules.into_iter().map(|(_, rule)| rule).collect();

  let messages = split.next().unwrap().lines().map(str::to_string).collect();
  Evaluator { rules, messages }
}

#[aoc(day19, part1)]
fn day19_part1(e: &Evaluator) -> usize {
  e.messages.iter().filter(|m| e.matches(m)).count()
}

#[aoc(day19, part2)]
fn day19_part2(e: &Evaluator) -> usize {
  let mut e = e.clone();
  // 42 | 42 8
  // 11: 42 31 | 42 11 31
  e.rules[8] = Rule::Or(vec![30].into(), vec![30, 8].into());
  e.rules[11] = Rule::Or(vec![30, 29].into(), vec![30, 11, 29].into());
  e.messages
    .iter()
    .filter(|m| {
      dbg!(m);
      dbg!(e.matches(m))
    })
    .count()
}
