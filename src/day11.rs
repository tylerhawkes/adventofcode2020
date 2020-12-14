#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Space {
  Occupied,
  Empty,
  Floor,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct WaitingArea {
  layout: Vec<Vec<Space>>,
}

impl WaitingArea {
  fn step(&self) -> Self {
    let mut new = self.clone();
    new.layout.iter_mut().enumerate().for_each(|(y, r)| {
      r.iter_mut().enumerate().for_each(|(x, s)| {
        let neighbor_count = self.neighbor_count(x, y);
        // println!("s = {:?}, neighbor_count = {}", s, neighbor_count);
        match s {
          Space::Floor => {}
          Space::Occupied => {
            if neighbor_count >= 4 {
              *s = Space::Empty;
            }
          }
          Space::Empty => {
            if neighbor_count == 0 {
              // println!("Setting {}, {} to occupied", x, y);
              *s = Space::Occupied;
            }
          }
        }
      })
    });
    // new.print();
    new
  }
  fn neighbor_count(&self, x: usize, y: usize) -> usize {
    let x = x as isize;
    let y = y as isize;
    let spaces = [
      self.space_at(x - 1, y - 1),
      self.space_at(x, y - 1),
      self.space_at(x + 1, y - 1),
      self.space_at(x - 1, y),
      self.space_at(x + 1, y),
      self.space_at(x - 1, y + 1),
      self.space_at(x, y + 1),
      self.space_at(x + 1, y + 1),
    ];
    spaces
      .iter()
      .copied()
      .filter_map(|s| s.map(|s| s == Space::Occupied))
      .filter(|x| *x)
      .count()
  }
  fn space_at(&self, x: isize, y: isize) -> Option<Space> {
    if x < 0 || y < 0 {
      None
    } else {
      self.layout.get(y as usize).and_then(|l| l.get(x as usize).copied())
    }
  }
  fn occupied(&self) -> usize {
    self
      .layout
      .iter()
      .map(|l| l.iter().filter(|s| **s == Space::Occupied).count())
      .sum()
  }
  #[allow(unused)]
  fn print(&self) {
    println!("---------------------------------------------------------------------------");
    self.layout.iter().for_each(|l| {
      l.iter().for_each(|s| match s {
        Space::Occupied => print!("#"),
        Space::Empty => print!("L"),
        Space::Floor => print!("."),
      });
      println!();
    })
  }
  fn step2(&self) -> Self {
    let mut new = self.clone();
    new.layout.iter_mut().enumerate().for_each(|(y, r)| {
      r.iter_mut().enumerate().for_each(|(x, s)| {
        let neighbor_count = self.neighbor_count2(x, y);
        // println!("s = {:?}, neighbor_count = {}", s, neighbor_count);
        match s {
          Space::Floor => {}
          Space::Occupied => {
            if neighbor_count >= 5 {
              *s = Space::Empty;
            }
          }
          Space::Empty => {
            if neighbor_count == 0 {
              // println!("Setting {}, {} to occupied", x, y);
              *s = Space::Occupied;
            }
          }
        }
      })
    });
    // new.print();
    new
  }
  fn neighbor_count2(&self, x: usize, y: usize) -> usize {
    let x = x as isize;
    let y = y as isize;
    let directions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    directions
      .iter()
      .copied()
      .filter_map(|(x1, y1)| {
        for i in 1.. {
          let space = self.space_at(x + x1 * i, y + y1 * i);
          match space {
            None => return None,
            Some(Space::Occupied) => return Some(1),
            Some(Space::Empty) => return Some(0),
            Some(Space::Floor) => continue,
          }
        }
        None
      })
      .sum()
  }
}

#[aoc_generator(day11)]
fn day11_generator(input: &str) -> WaitingArea {
  let layout = input
    .lines()
    .map(|l| l.chars().map(|c| if c == 'L' { Space::Empty } else { Space::Floor }).collect())
    .collect();
  WaitingArea { layout }
}

#[aoc(day11, part1)]
fn day11_part1(prev: &WaitingArea) -> usize {
  let mut prev = prev.step();
  let mut waiting_area = prev.step();
  while waiting_area != prev {
    std::mem::swap(&mut prev, &mut waiting_area);
    waiting_area = prev.step();
  }
  waiting_area.occupied()
}

#[aoc(day11, part2)]
fn day11_part2(prev: &WaitingArea) -> usize {
  let mut prev = prev.step2();
  let mut waiting_area = prev.step2();
  println!("{:?}", waiting_area.neighbor_count(8, 8));
  while waiting_area != prev {
    std::mem::swap(&mut prev, &mut waiting_area);
    waiting_area = prev.step2();
  }
  waiting_area.occupied()
}
