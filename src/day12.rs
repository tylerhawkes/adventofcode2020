#[derive(Copy, Clone, Debug)]
enum Instruction {
  East(i32),
  West(i32),
  North(i32),
  South(i32),
  Forward(i32),
  Left(i32),
  Right(i32),
}

#[aoc_generator(day12)]
fn day12_generator(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|l| {
      let instruction = l.chars().next().unwrap();
      let distance = l[1..].parse().unwrap();
      match instruction as char {
        'W' => Instruction::West(distance),
        'E' => Instruction::East(distance),
        'N' => Instruction::North(distance),
        'S' => Instruction::South(distance),
        'F' => Instruction::Forward(distance),
        'L' => Instruction::Left(distance),
        'R' => Instruction::Right(distance),
        _ => unreachable!(),
      }
    })
    .collect()
}

#[aoc(day12, part1)]
fn day12_part1(input: &[Instruction]) -> i32 {
  let mut direction = Instruction::East(0);
  let (mut x, mut y) = (0_i32, 0_i32);
  input.iter().copied().for_each(|i| match i {
    Instruction::East(d) => x += d,
    Instruction::West(d) => x -= d,
    Instruction::North(d) => y -= d,
    Instruction::South(d) => y += d,
    Instruction::Forward(d) => match direction {
      Instruction::East(_) => x += d,
      Instruction::West(_) => x -= d,
      Instruction::North(_) => y -= d,
      Instruction::South(_) => y += d,
      _ => unreachable!(),
    },
    Instruction::Left(d) => match (direction, d) {
      (Instruction::East(_), 90) => direction = Instruction::North(0),
      (Instruction::West(_), 90) => direction = Instruction::South(0),
      (Instruction::North(_), 90) => direction = Instruction::West(0),
      (Instruction::South(_), 90) => direction = Instruction::East(0),
      (Instruction::East(_), 180) => direction = Instruction::West(0),
      (Instruction::West(_), 180) => direction = Instruction::East(0),
      (Instruction::North(_), 180) => direction = Instruction::South(0),
      (Instruction::South(_), 180) => direction = Instruction::North(0),
      (Instruction::East(_), 270) => direction = Instruction::South(0),
      (Instruction::West(_), 270) => direction = Instruction::North(0),
      (Instruction::North(_), 270) => direction = Instruction::East(0),
      (Instruction::South(_), 270) => direction = Instruction::West(0),
      _ => unreachable!(),
    },
    Instruction::Right(d) => match (direction, d) {
      (Instruction::East(_), 90) => direction = Instruction::South(0),
      (Instruction::West(_), 90) => direction = Instruction::North(0),
      (Instruction::North(_), 90) => direction = Instruction::East(0),
      (Instruction::South(_), 90) => direction = Instruction::West(0),
      (Instruction::East(_), 180) => direction = Instruction::West(0),
      (Instruction::West(_), 180) => direction = Instruction::East(0),
      (Instruction::North(_), 180) => direction = Instruction::South(0),
      (Instruction::South(_), 180) => direction = Instruction::North(0),
      (Instruction::East(_), 270) => direction = Instruction::North(0),
      (Instruction::West(_), 270) => direction = Instruction::South(0),
      (Instruction::North(_), 270) => direction = Instruction::West(0),
      (Instruction::South(_), 270) => direction = Instruction::East(0),
      _ => unreachable!(),
    },
  });
  x.abs() + y.abs()
}

#[aoc(day12, part2)]
fn day12_part2(input: &[Instruction]) -> i32 {
  let (mut wayx, mut wayy) = (10, -1);
  let (mut x, mut y) = (0_i32, 0_i32);
  input.iter().copied().for_each(|i| {
    let tmp = wayx;
    match i {
      Instruction::East(d) => wayx += d,
      Instruction::West(d) => wayx -= d,
      Instruction::North(d) => wayy -= d,
      Instruction::South(d) => wayy += d,
      Instruction::Forward(d) => {
        x += wayx * d;
        y += wayy * d;
      }
      Instruction::Left(d) => match d {
        90 => {
          wayx = wayy;
          wayy = -tmp;
        }
        180 => {
          wayx = -wayx;
          wayy = -wayy;
        }
        270 => {
          wayx = -wayy;
          wayy = tmp;
        }
        _ => unreachable!(),
      },
      Instruction::Right(d) => match d {
        90 => {
          wayx = -wayy;
          wayy = tmp;
        }
        180 => {
          wayx = -wayx;
          wayy = -wayy;
        }
        270 => {
          wayx = wayy;
          wayy = -tmp;
        }
        _ => unreachable!(),
      },
    }
    println!("{:?}, {}, {}, {}, {}", i, x, y, wayx, wayy);
  });
  x.abs() + y.abs()
}
