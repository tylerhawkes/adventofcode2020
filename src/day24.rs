use std::collections::HashSet;
use std::ops::Add;

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Coord {
  x: i16,
  y: i16,
}

impl Coord {
  fn valid(self) -> bool {
    self.x & 1 == self.y & 1
  }
}

impl Add for Coord {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Direction {
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest,
  NorthEast,
}

impl Direction {
  fn grid_change(self) -> Coord {
    match self {
      Self::East => Coord { x: 2, y: 0 },
      Direction::SouthEast => Coord { x: 1, y: 1 },
      Direction::SouthWest => Coord { x: -1, y: 1 },
      Direction::West => Coord { x: -2, y: 0 },
      Direction::NorthWest => Coord { x: -1, y: -1 },
      Direction::NorthEast => Coord { x: 1, y: -1 },
    }
  }
  fn iter() -> impl Iterator<Item = Self> {
    (0_u8..6).map(|i| unsafe { std::mem::transmute(i) })
  }
}

#[derive(Copy, Clone, Debug)]
enum Color {
  Black,
  White,
}

#[derive(Clone, Debug)]
struct Grid {
  black_tiles: HashSet<Coord>,
  directions: Vec<Vec<Direction>>,
}

impl Grid {
  fn set_directions(&mut self) {
    let tiles = &mut self.black_tiles;
    self.directions.iter().for_each(|v| {
      let coord = v.iter().copied().fold(Coord::default(), |c, d| c + d.grid_change());
      if !tiles.insert(coord) {
        tiles.remove(&coord);
      }
    });
  }
  fn grid_area(&self) -> (Coord, Coord) {
    let mut min_coord = Coord::default();
    let mut max_coord = Coord::default();
    self.black_tiles.iter().copied().for_each(|c| {
      min_coord.x = min_coord.x.min(c.x);
      min_coord.y = min_coord.y.min(c.y);
      max_coord.x = max_coord.x.max(c.x);
      max_coord.y = max_coord.y.max(c.y);
    });
    (min_coord, max_coord)
  }
  fn get(&self, coord: Coord) -> Color {
    if self.black_tiles.contains(&coord) {
      Color::Black
    } else {
      Color::White
    }
  }
}

#[aoc_generator(day24)]
fn day24_generator(input: &str) -> Grid {
  enum Modifier {
    South,
    North,
    None,
  };
  let mut m = Modifier::None;
  let directions = input
    .lines()
    .map(|l| {
      l.chars()
        .filter_map(|c| match c {
          'n' => {
            m = Modifier::North;
            None
          }
          's' => {
            m = Modifier::South;
            None
          }
          'e' => {
            let r = match m {
              Modifier::None => Direction::East,
              Modifier::North => Direction::NorthEast,
              Modifier::South => Direction::SouthEast,
            };
            m = Modifier::None;
            Some(r)
          }
          'w' => {
            let r = match m {
              Modifier::None => Direction::West,
              Modifier::North => Direction::NorthWest,
              Modifier::South => Direction::SouthWest,
            };
            m = Modifier::None;
            Some(r)
          }
          _ => unreachable!(),
        })
        .collect()
    })
    .collect::<Vec<_>>();
  Grid {
    black_tiles: HashSet::with_capacity(directions.len()),
    directions,
  }
}

#[aoc(day24, part1)]
fn day24_part1(grid: &Grid) -> usize {
  let mut grid = grid.clone();
  grid.set_directions();
  grid.black_tiles.len()
}

#[aoc(day24, part2)]
fn day24_part2(grid: &Grid) -> usize {
  let mut grid = grid.clone();
  let mut insertions = HashSet::with_capacity(1024);
  let mut deletions = HashSet::with_capacity(1024);
  grid.set_directions();
  // println!("{}", grid.black_tiles.len());
  // dbg!(&grid.black_tiles);
  for i in 0..100 {
    insertions.clear();
    deletions.clear();
    let (min_coord, max_coord) = grid.grid_area();
    for coord in (min_coord.x - 2..=max_coord.x + 2)
      .flat_map(|x| (min_coord.y - 2..max_coord.y + 2).map(move |y| Coord { x, y }))
      .filter(|c| c.valid())
    {
      let tile = grid.get(coord);
      let black_neighbors = Direction::iter()
        .filter(|d| {
          let neighbor = coord + d.grid_change();
          match grid.get(neighbor) {
            Color::White => false,
            Color::Black => true,
          }
        })
        .count();
      match (tile, black_neighbors) {
        (Color::Black, 0) | (Color::Black, 3..=6) => {
          deletions.insert(coord);
        }
        (Color::White, 2) => {
          insertions.insert(coord);
        }
        _ => {}
      }
    }
    deletions.iter().for_each(|d| {
      grid.black_tiles.remove(d);
    });
    insertions.iter().copied().for_each(|d| {
      grid.black_tiles.insert(d);
    });
    // println!("{}: {}", i + 1, grid.black_tiles.len());
    // dbg!(&grid.black_tiles);
  }
  grid.black_tiles.len()
}
