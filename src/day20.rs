use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tiles {
  tiles: Vec<Tile>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
  id: u64,
  // top left = 0, top right = 9
  // bottom left = 90, bottom right = 99
  pixels: u128,
}

impl Tile {
  // rotates counter clockwise
  fn rotate(self) -> Self {
    let mut n = 0;
    for row in 0..10 {
      for column in 0..10 {
        let bit = 1 << (row * 10 + column);
        if (self.pixels & bit) > 0 {
          n |= 1 << (9 - row + column * 10);
        }
      }
    }
    Self { id: self.id, pixels: n }
  }
  // fn rotate_180(self) -> Self {
  //   Self {self.id, pixels: self.pixels.reverse_bits() >> 28}
  // }
  fn flip(self) -> Self {
    let mut n = 0;
    for row in 0..10 {
      let r = self.row(row) as u128;
      n |= r << (90 - (row * 10));
    }
    Self { id: self.id, pixels: n }
  }
  fn row(self, row: u8) -> u16 {
    const ROW_MASK: u128 = (1 << 10) - 1;
    let row_start = row * 10;
    ((self.pixels & (ROW_MASK << row_start)) >> row_start) as u16
  }
  fn image_row(self, row: u8) -> u8 {
    ((self.row(row + 1) >> 1) & 0xff) as u8
  }
  fn top(self) -> u16 {
    self.row(0)
  }
  fn bottom(self) -> u16 {
    self.row(9)
  }
  fn column(self, column: u8) -> u16 {
    let mut n = 0;
    for right in 0..10 {
      if self.pixels & (1 << ((right * 10) + column)) > 0 {
        n |= 1 << right;
      }
    }
    n
  }
  fn left(self) -> u16 {
    self.column(0)
  }
  fn right(self) -> u16 {
    self.column(9)
  }
  fn rotations(self) -> [Self; 8] {
    let two = self.rotate();
    let three = two.rotate();
    let four = three.rotate();
    let five = self.flip();
    let six = five.rotate();
    let seven = six.rotate();
    let eight = seven.rotate();
    [self, two, three, four, five, six, seven, eight]
  }
}

struct Puzzle {
  tiles: Vec<Vec<Tile>>,
  size: usize,
}

#[aoc_generator(day20)]
fn day20_generator(input: &str) -> Puzzle {
  let tiles = input
    .split("\n\n")
    .map(|t| {
      let id = t
        .split(":")
        .next()
        .unwrap()
        .split("Tile ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
      let pixels = t
        .lines()
        .skip(1)
        .flat_map(|l| l.chars())
        .enumerate()
        .map(|(i, c)| if c == '#' { 1_u128 << i } else { 0 })
        .sum::<u128>();
      Tile { id, pixels }
    })
    .collect();
  let tiles = Tiles { tiles };
  let sides = tiles
    .tiles
    .iter()
    .copied()
    .filter(|t| {
      let t = *t;
      let top = t.top();
      !tiles
        .tiles
        .iter()
        .copied()
        .filter(|m| m.id != t.id)
        .any(|m| m.rotations().iter().copied().any(|r| r.top() == top))
    })
    .collect::<Vec<_>>();
  // dbg!(&sides);
  // dbg!(sides.len());
  let corners = sides
    .iter()
    .copied()
    .filter(|t| {
      let t = *t;
      let left = t.left();
      !tiles
        .tiles
        .iter()
        .copied()
        .filter(|m| m.id != t.id)
        .any(|m| m.rotations().iter().copied().any(|r| r.left() == left))
    })
    .collect::<Vec<_>>();
  let top_left = corners[0];
  let size = (tiles.tiles.len() as f64).sqrt() as usize;
  let mut puzzle = vec![vec![]; size];
  let mut used = HashSet::with_capacity(tiles.tiles.len());
  puzzle[0].push(top_left);
  used.insert(top_left.id);
  assert_eq!(size * size, tiles.tiles.len());
  for row in 0..size {
    for column in 0..size {
      let available = tiles.tiles.iter().copied().filter(|t| !used.contains(&t.id));
      match (row, column) {
        (0, 0) => continue,
        (0, _) => {
          //top row
          let prev = puzzle[0][column - 1];
          let prev_right = prev.right();
          let next_matches = available
            .filter_map(|a| a.rotations().iter().copied().find(|a| a.left() == prev_right))
            .collect::<Vec<_>>();
          if next_matches.len() != 1 {
            dbg!(&next_matches);
          }
          let next = next_matches[0];
          puzzle[0].push(next);
          used.insert(next.id);
        }
        (_, 0) => {
          let prev = puzzle[row - 1][0];
          let prev_bottom = prev.bottom();
          let next_matches = available
            .filter_map(|a| a.rotations().iter().copied().find(|a| a.top() == prev_bottom))
            .collect::<Vec<_>>();
          if next_matches.len() != 1 {
            dbg!(&next_matches);
          }
          let next = next_matches[0];
          puzzle[row].push(next);
          used.insert(next.id);
        }
        (_, _) => {
          //needs to match top and left
          let prev_top = puzzle[row - 1][column];
          let prev_bottom = prev_top.bottom();
          let prev_left = puzzle[row][column - 1];
          let prev_right = prev_left.right();
          let next_matches = available
            .filter_map(|a| {
              a.rotations()
                .iter()
                .copied()
                .find(|a| a.top() == prev_bottom && a.left() == prev_right)
            })
            .collect::<Vec<_>>();
          if next_matches.len() != 1 {
            dbg!(&next_matches);
          }
          let next = next_matches[0];
          puzzle[row].push(next);
          used.insert(next.id);
        }
      }
    }
  }
  Puzzle { tiles: puzzle, size }
}

#[aoc(day20, part1)]
fn day20_part1(puzzle: &Puzzle) -> u64 {
  // dbg!(&puzzle);
  let size = puzzle.size;
  puzzle.tiles[0][0].id * puzzle.tiles[0][size - 1].id * puzzle.tiles[size - 1][0].id * puzzle.tiles[size - 1][size - 1].id
}

#[aoc(day20, part2)]
fn day20_part2(puzzle: &Puzzle) -> usize {
  let sea_monster = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   "
    .lines()
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
    .filter_map(|(x, y, c)| if c == '#' { Some((x, y)) } else { None })
    .collect::<Vec<_>>();
  let (x_window, y_window) = sea_monster.iter().fold((0, 0), |(ax, ay), &(x, y)| (ax.max(x + 1), ay.max(y + 1)));
  let mut image = vec![];
  for row in puzzle.tiles.iter() {
    for tile_row in 0..8 {
      let mut image_row = vec![];
      row
        .iter()
        .map(|t| t.image_row(tile_row))
        .for_each(|r| (0..8).for_each(|i| image_row.push(((r >> i) & 1) > 0)));
      image.push(image_row);
    }
  }
  // image.iter().for_each(|r| {
  //   println!();
  //   r.iter().copied().for_each(|p| print!("{}", if p { '#' } else { '.' }));
  // });
  // println!();
  let sea_monsters = image_rotations(image.clone())
    .iter()
    .map(|r| {
      r.windows(y_window)
        .map(|y| {
          (0..y[0].len() - x_window)
            .filter(|&offset| sea_monster.iter().copied().all(|(smx, smy)| y[smy][smx + offset]))
            .count()
        })
        .sum()
    })
    .collect::<Vec<usize>>();

  // dbg!(&sea_monsters);
  let activated_pixels = image.iter().map(|row| row.iter().filter(|p| **p).count()).sum::<usize>();
  activated_pixels - sea_monsters.iter().max().unwrap() * sea_monster.len()
}

fn rotate(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  let mut new = image.clone();
  let x_max = image[0].len() - 1;
  let y_max = image.len() - 1;
  assert_eq!(x_max, y_max);
  image.iter().enumerate().for_each(|(y, row)| {
    row.iter().copied().enumerate().for_each(|(x, pixel)| {
      new[x][x_max - y] = pixel;
    })
  });
  new
}

fn image_rotations(image: Vec<Vec<bool>>) -> [Vec<Vec<bool>>; 8] {
  let two = rotate(&image);
  let three = rotate(&two);
  let four = rotate(&three);
  let mut five = image.clone();
  five.reverse();
  let six = rotate(&five);
  let seven = rotate(&six);
  let eight = rotate(&seven);
  [image, two, three, four, five, six, seven, eight]
}

// 1    5
// #..  ..#
// .#.  .#.
// .##  ##.
// 2    6
// ..#  #..
// ##.  .##
// #..  ..#
// 3    7
// ##.  .##
// .#.  .#.
// ..#  #..
// 4    8
// ..#  #..
// .##  ##.
// #..  ..#
