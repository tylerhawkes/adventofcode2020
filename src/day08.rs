use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Program {
  instructions: Vec<Instruction>,
  global_value: isize,
  visited: HashSet<usize>,
  offset: usize,
}

impl Program {
  fn exec(&mut self) -> Result<isize, isize> {
    self.global_value = 0;
    self.visited.clear();
    self.offset = 0;
    while !self.visited.contains(&self.offset) && self.offset < self.instructions.len() {
      self.visited.insert(self.offset);
      self.offset = self.instructions[self.offset].exec(self.offset, &mut self.global_value);
    }
    if self.offset < self.instructions.len() {
      Err(self.global_value)
    } else {
      Ok(self.global_value)
    }
  }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
  Acc(isize),
  Jmp(isize),
  Nop(isize),
}

impl Instruction {
  fn exec(self, offset: usize, global_value: &mut isize) -> usize {
    match self {
      Self::Acc(i) => {
        *global_value += i;
        offset + 1
      }
      Self::Nop(_) => offset + 1,
      Self::Jmp(i) => (offset as isize + i) as usize,
    }
  }
  fn is_acc(&self) -> bool {
    match self {
      Self::Acc(_) => true,
      _ => false,
    }
  }
  fn swap(&mut self) {
    *self = match self {
      Self::Nop(i) => Self::Jmp(*i),
      Self::Jmp(i) => Self::Nop(*i),
      _ => unreachable!(),
    }
  }
}

#[aoc_generator(day8)]
fn day8_generator(input: &str) -> Program {
  let instructions = input
    .lines()
    .map(|l| {
      let i = l[4..].parse().unwrap();
      match &l[..3] {
        "acc" => Instruction::Acc(i),
        "jmp" => Instruction::Jmp(i),
        "nop" => Instruction::Nop(i),
        _ => unreachable!(),
      }
    })
    .collect();
  Program {
    instructions,
    global_value: 0,
    visited: HashSet::new(),
    offset: 0,
  }
}

#[aoc(day8, part1)]
fn day8_part1(input: &Program) -> isize {
  input.clone().exec().unwrap_err()
}

#[aoc(day8, part2)]
fn day8_part2(input: &Program) -> isize {
  for i in 0..input.instructions.len() {
    if !input.instructions[i].is_acc() {
      let mut test = input.clone();
      test.instructions[i].swap();
      if let Ok(i) = test.exec() {
        return i;
      }
    }
  }
  unreachable!()
}
