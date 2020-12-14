#[aoc_generator(day14)]
fn day14_generator(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|l| {
      if l.starts_with("mask") {
        let (on, off) = l.chars().skip(7).enumerate().fold((0, !0), |(on, off), (i, c)| match c {
          '1' => (on | 1 << (35 - i), off),
          '0' => (on, off ^ 1 << (35 - i)),
          'X' => (on, off),
          _ => unreachable!(),
        });
        // println!("on:  {:036b}", on);
        // println!("off: {:036b}", off);
        Instruction::Mask {
          on,
          off: (off << 28) >> 28,
        }
      } else {
        let mut split = l[4..].split("] = ");
        Instruction::Write {
          addr: split.next().unwrap().parse().unwrap(),
          value: split.next().unwrap().parse().unwrap(),
        }
      }
    })
    .collect()
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
  Mask { on: u64, off: u64 },
  Write { addr: u64, value: u64 },
}

#[aoc(day14, part1)]
fn day14_part1(instructions: &[Instruction]) -> u64 {
  let mut memory = std::collections::HashMap::new();
  let mut mask_on = 0;
  let mut mask_off = 0;
  instructions.iter().copied().for_each(|i| match i {
    Instruction::Mask { on, off } => {
      mask_on = on;
      mask_off = off;
    }
    Instruction::Write { addr, value } => {
      memory.insert(addr, (value | mask_on) & mask_off);
    }
  });
  memory.values().copied().sum()
}

#[aoc(day14, part2)]
fn day14_part2(instructions: &[Instruction]) -> u64 {
  let mut memory = std::collections::HashMap::new();
  let mut mask_on = 0;
  let mut mask_off = 0;
  instructions.iter().copied().for_each(|i| match i {
    Instruction::Mask { on, off } => {
      mask_on = on;
      mask_off = off;
    }
    Instruction::Write { addr, value } => {
      let floating = ((mask_on ^ mask_off) << 28) >> 28;
      for (on, off) in floating_to_masks(floating) {
        let a = ((addr | mask_on) | on) & off;
        memory.insert(a, value);
      }
    }
  });
  memory.values().copied().sum()
}

fn floating_to_masks(floating: u64) -> Vec<(u64, u64)> {
  let ones_indices = (0..36)
    .filter_map(|i| if (1 << i) & floating > 0 { Some(i) } else { None })
    .collect::<Vec<u64>>();
  (0_u64..1 << floating.count_ones())
    .map(|i| {
      let mut on = 0;
      let mut off = !0 >> 28;
      ones_indices.iter().copied().enumerate().for_each(|(j, k)| {
        if i & (1 << j) > 0 {
          on |= 1 << k;
        } else {
          off ^= 1 << k;
        }
      });
      (on, off)
    })
    .collect()
}
