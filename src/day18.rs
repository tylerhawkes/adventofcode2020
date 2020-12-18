#[derive(Debug, Clone)]
enum Expr {
  Int(u64),
  Multiply,
  Add,
  Expr(Box<[Expr]>),
}

impl Expr {
  fn parse_from_chars(chars: &mut dyn Iterator<Item = char>) -> Self {
    let mut e = Vec::new();
    loop {
      match chars.next() {
        Some(c) => match c {
          '0'..='9' => e.push(Expr::Int((c as u8 - b'0') as u64)),
          ' ' => continue,
          '*' => e.push(Expr::Multiply),
          '+' => e.push(Expr::Add),
          '(' => e.push(Expr::parse_from_chars(chars)),
          ')' => return Expr::Expr(e.into_boxed_slice()),
          _ => unreachable!(),
        },
        None => return Expr::Expr(e.into_boxed_slice()),
      }
    }
  }
  fn eval(&self) -> u64 {
    match self {
      Self::Expr(e) => {
        let mut e = &e[..];
        let mut operation = Self::Add;
        let mut lhs = 0;
        while !e.is_empty() {
          match operation {
            Self::Add => {
              lhs += Self::eval(&e[0]);
            }
            Self::Multiply => {
              lhs *= Self::eval(&e[0]);
            }
            _ => unreachable!(),
          }
          if e.len() > 1 {
            operation = match &e[1] {
              Self::Multiply => Self::Multiply,
              Self::Add => Self::Add,
              _ => unreachable!(),
            };
            e = &e[2..];
          } else {
            return lhs;
          }
        }
        lhs
      }
      Self::Int(u) => *u,
      _ => unreachable!(),
    }
  }
}

#[aoc_generator(day18, part1)]
fn day18_generator_part1(input: &str) -> Vec<Expr> {
  input.lines().map(|l| Expr::parse_from_chars(&mut l.chars())).collect()
}

#[aoc(day18, part1)]
fn day18_part1(e: &[Expr]) -> u64 {
  e.iter().map(Expr::eval).sum()
}

#[derive(Debug, Clone)]
enum TreeExpr {
  Int(u64),
  Multiply(Box<TreeExpr>, Box<TreeExpr>),
  Add(Box<TreeExpr>, Box<TreeExpr>),
  Expr(Box<TreeExpr>),
}

// I'm sure there is a better way to do this, but it worked.
impl TreeExpr {
  fn parse_from_chars(chars: &mut dyn Iterator<Item = char>) -> Self {
    Self::Expr(Box::new(Self::parse_from_chars_inner(chars)))
  }
  fn parse_from_chars_inner(chars: &mut dyn Iterator<Item = char>) -> Self {
    let mut left = Self::Int(0);
    loop {
      match chars.next() {
        Some(c) => match c {
          '0'..='9' => left = Self::Int((c as u8 - b'0') as u64),
          ' ' => continue,
          '*' => return Self::Multiply(Box::new(left), Box::new(Self::parse_from_chars_inner(chars))),
          '+' => return Self::Add(Box::new(left), Box::new(Self::parse_from_chars_inner(chars))),
          '(' => left = Self::Expr(Box::new(Self::parse_from_chars_inner(chars))),
          ')' => return left,
          _ => unreachable!(),
        },
        None => return left,
      }
    }
  }
  fn eval(mut self) -> u64 {
    loop {
      match self.eval_inner() {
        Self::Int(u) => return u,
        a => self = a.eval_inner(),
      }
    }
  }
  fn eval_inner(mut self) -> Self {
    match self {
      Self::Int(u) => Self::Int(u),
      Self::Expr(e) => Self::Int(e.eval()),
      Self::Add(l, r) => match *r {
        r @ TreeExpr::Int(_) | r @ TreeExpr::Expr(_) => return Self::Int(l.eval() + r.eval()),
        TreeExpr::Add(l1, r1) => {
          self = Self::Add(Box::new(TreeExpr::Int(l.eval() + l1.eval())), r1);
          self.eval_inner()
        }
        TreeExpr::Multiply(l1, r1) => {
          self = Self::Multiply(Box::new(TreeExpr::Int(l.eval() + l1.eval())), r1);
          self.eval_inner()
        }
      },
      Self::Multiply(l, r) => {
        return Self::Int(l.eval() * r.eval());
      }
    }
  }
}

#[aoc_generator(day18, part2)]
fn day18_generator(input: &str) -> Vec<TreeExpr> {
  input.lines().map(|l| TreeExpr::parse_from_chars(&mut l.chars())).collect()
}

#[aoc(day18, part2)]
fn day18_part2(e: &[TreeExpr]) -> u64 {
  e.to_vec().into_iter().map(TreeExpr::eval).sum()
}
