#[derive(Default, Debug, Clone)]
struct Passport {
  // Birth Year
  byr: Option<String>,
  // Issue Year
  iyr: Option<String>,
  // Expiration Year
  eyr: Option<String>,
  // Height
  hgt: Option<String>,
  // Hair Color
  hcl: Option<String>,
  // Eye Color
  ecl: Option<String>,
  // Passport ID
  pid: Option<String>,
  // Country ID
  cid: Option<String>,
}

impl Passport {
  fn has_all_fields(&self) -> bool {
    self.byr.is_some()
      && self.iyr.is_some()
      && self.eyr.is_some()
      && self.hgt.is_some()
      && self.hcl.is_some()
      && self.ecl.is_some()
      && self.pid.is_some()
  }
  fn is_valid(&self) -> bool {
    match self.byr.as_ref().map(|s| s.parse::<u16>()) {
      Some(Ok(v)) if v >= 1920 && v <= 2002 => {}
      _ => return false,
    };
    match self.iyr.as_ref().map(|s| s.parse::<u16>()) {
      Some(Ok(v)) if v >= 2010 && v <= 2020 => {}
      _ => return false,
    };
    match self.eyr.as_ref().map(|s| s.parse::<u16>()) {
      Some(Ok(v)) if v >= 2020 && v <= 2030 => {}
      _ => return false,
    };
    match self
      .hgt
      .as_ref()
      .map(|s| (s[..s.len().saturating_sub(2)].parse::<u16>(), &s[s.len().saturating_sub(2)..]))
    {
      Some((Ok(h), m)) if m == "in" && h >= 59 && h <= 76 => {}
      Some((Ok(h), m)) if m == "cm" && h >= 150 && h <= 193 => {}
      _ => return false,
    };
    match self.hcl.as_ref() {
      Some(v) if v.as_bytes()[0] == b'#' && v[1..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) => {}
      _ => return false,
    }
    match self.ecl.as_deref() {
      Some("amb") => {}
      Some("blu") => {}
      Some("brn") => {}
      Some("gry") => {}
      Some("grn") => {}
      Some("hzl") => {}
      Some("oth") => {}
      _ => return false,
    }
    match self.pid.as_deref() {
      Some(v) if v.len() == 9 && v.chars().all(|c| c >= '0' && c <= '9') => {}
      _ => return false,
    }
    true
  }
}

#[aoc_generator(day4)]
fn day4_generator(input: &str) -> Vec<Passport> {
  let mut passport = Passport::default();
  let mut v = Vec::new();
  for line in input.lines() {
    if line.is_empty() {
      // println!("{:?}", passport);
      let mut new_passport = Passport::default();
      std::mem::swap(&mut new_passport, &mut passport);
      v.push(new_passport);
      continue;
      // println!("-------------------------------");
    }
    // println!("line: {}", line);
    for val in line.split_ascii_whitespace() {
      match &val[..3] {
        "byr" => passport.byr = Some(val[4..].to_string()),
        "iyr" => passport.iyr = Some(val[4..].to_string()),
        "eyr" => passport.eyr = Some(val[4..].to_string()),
        "hgt" => passport.hgt = Some(val[4..].to_string()),
        "hcl" => passport.hcl = Some(val[4..].to_string()),
        "ecl" => passport.ecl = Some(val[4..].to_string()),
        "pid" => passport.pid = Some(val[4..].to_string()),
        "cid" => passport.cid = Some(val[4..].to_string()),
        _ => unreachable!(),
      }
    }
  }
  v.push(passport);
  v
}

#[aoc(day4, part1)]
fn day4_part1(i: &[Passport]) -> usize {
  i.iter().filter(|p| p.has_all_fields()).count()
}

#[aoc(day4, part2)]
fn day4_part2(i: &[Passport]) -> usize {
  i.iter().filter(|p| p.is_valid()).count()
}
