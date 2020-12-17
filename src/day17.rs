use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Coord3D(i32, i32, i32);

impl Coord3D {
  fn neighbors(self) -> impl Iterator<Item = Self> {
    (self.0 - 1..=self.0 + 1)
      .flat_map(move |x| (self.1 - 1..=self.1 + 1).map(move |y| (x, y)))
      .flat_map(move |(x, y)| (self.2 - 1..=self.2 + 1).map(move |z| Coord3D(x, y, z)))
      .filter(move |c| c != &self)
  }
}

#[derive(Debug, Clone)]
struct PocketDimension {
  dimensions: HashSet<Coord3D>,
  min_x: i32,
  min_y: i32,
  min_z: i32,
  max_x: i32,
  max_y: i32,
  max_z: i32,
  additions: HashSet<Coord3D>,
  deletions: HashSet<Coord3D>,
}

impl PocketDimension {
  fn set(&mut self, c: Coord3D) {
    // println!("Inserting {:?}", c);
    self.additions.insert(c);
    self.min_x = self.min_x.min(c.0);
    self.min_y = self.min_y.min(c.1);
    self.min_z = self.min_z.min(c.2);
    self.max_x = self.max_x.max(c.0);
    self.max_y = self.max_y.max(c.1);
    self.max_z = self.max_z.max(c.2);
  }
  fn unset(&mut self, c: Coord3D) {
    // println!("Removing {:?}", c);
    self.deletions.insert(c);
  }
  fn step(&mut self) {
    // println!("Stepping: {}, {}", self.additions.len(), self.deletions.len());
    assert_eq!(self.additions.intersection(&self.deletions).count(), 0);
    let dimensions = &mut self.dimensions;
    self
      .additions
      .drain()
      // .inspect(|c| println!("Adding {:?}", c))
      .for_each(|c| assert!(dimensions.insert(c)));
    self
      .deletions
      .drain()
      // .inspect(|c| println!("Deleting {:?}", c))
      .for_each(|c| assert!(dimensions.remove(&c)));
  }
  fn iter_space(&self) -> impl Iterator<Item = Coord3D> {
    let x_range = self.min_x - 1..=self.max_x + 1;
    let y_range = self.min_y - 1..=self.max_y + 1;
    let z_range = self.min_z - 1..=self.max_z + 1;
    x_range
      .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
      .flat_map(move |(x, y)| z_range.clone().map(move |z| Coord3D(x, y, z)))
  }
}

#[aoc_generator(day17, part1)]
fn day17_generator(input: &str) -> PocketDimension {
  let mut dimensions = PocketDimension {
    dimensions: HashSet::with_capacity(10000),
    min_x: 0,
    min_y: 0,
    min_z: 0,
    max_x: 0,
    max_y: 0,
    max_z: 0,
    additions: Default::default(),
    deletions: Default::default(),
  };
  input.lines().enumerate().for_each(|(x, l)| {
    l.chars().enumerate().for_each(|(y, c)| {
      if c == '#' {
        dimensions.set(Coord3D(x as i32, y as i32, 0));
      }
    })
  });
  dimensions.step();
  dimensions
}

#[aoc(day17, part1)]
fn day17_part1(dimensions: &PocketDimension) -> usize {
  let mut dimensions = dimensions.clone();
  for _ in 0..6 {
    // println!("iter_space: {}", dimensions.iter_space().count());
    dimensions.iter_space().for_each(|c| {
      let active_neighbors = c.neighbors().filter(|c| dimensions.dimensions.contains(c)).count();
      if dimensions.dimensions.contains(&c) {
        match active_neighbors {
          2 | 3 => {}
          _ => dimensions.unset(c),
        }
      } else {
        if active_neighbors == 3 {
          dimensions.set(c);
        }
      }
    });
    dimensions.step();
  }
  // dbg!(&dimensions);
  dimensions.dimensions.len()
}

//////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Coord4D(i32, i32, i32, i32);

impl Coord4D {
  fn neighbors(self) -> impl Iterator<Item = Self> {
    (self.0 - 1..=self.0 + 1)
      .flat_map(move |x| (self.1 - 1..=self.1 + 1).map(move |y| (x, y)))
      .flat_map(move |(x, y)| (self.2 - 1..=self.2 + 1).map(move |z| (x, y, z)))
      .flat_map(move |(x, y, z)| (self.3 - 1..=self.3 + 1).map(move |w| Self(x, y, z, w)))
      .filter(move |c| c != &self)
  }
}

#[derive(Debug, Clone)]
struct PocketDimension4D {
  dimensions: HashSet<Coord4D>,
  min_x: i32,
  min_y: i32,
  min_z: i32,
  min_w: i32,
  max_x: i32,
  max_y: i32,
  max_z: i32,
  max_w: i32,
  additions: HashSet<Coord4D>,
  deletions: HashSet<Coord4D>,
}

impl PocketDimension4D {
  fn set(&mut self, c: Coord4D) {
    // println!("Inserting {:?}", c);
    self.additions.insert(c);
    self.min_x = self.min_x.min(c.0);
    self.min_y = self.min_y.min(c.1);
    self.min_z = self.min_z.min(c.2);
    self.min_w = self.min_w.min(c.3);
    self.max_x = self.max_x.max(c.0);
    self.max_y = self.max_y.max(c.1);
    self.max_z = self.max_z.max(c.2);
    self.max_w = self.max_w.max(c.3);
  }
  fn unset(&mut self, c: Coord4D) {
    // println!("Removing {:?}", c);
    self.deletions.insert(c);
  }
  fn step(&mut self) {
    // println!("Stepping: {}, {}", self.additions.len(), self.deletions.len());
    assert_eq!(self.additions.intersection(&self.deletions).count(), 0);
    let dimensions = &mut self.dimensions;
    self
      .additions
      .drain()
      // .inspect(|c| println!("Adding {:?}", c))
      .for_each(|c| assert!(dimensions.insert(c)));
    self
      .deletions
      .drain()
      // .inspect(|c| println!("Deleting {:?}", c))
      .for_each(|c| assert!(dimensions.remove(&c)));
  }
  fn iter_space(&self) -> impl Iterator<Item = Coord4D> {
    let x_range = self.min_x - 1..=self.max_x + 1;
    let y_range = self.min_y - 1..=self.max_y + 1;
    let z_range = self.min_z - 1..=self.max_z + 1;
    let w_range = self.min_w - 1..=self.max_w + 1;
    x_range
      .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
      .flat_map(move |(x, y)| z_range.clone().map(move |z| (x, y, z)))
      .flat_map(move |(x, y, z)| w_range.clone().map(move |w| Coord4D(x, y, z, w)))
  }
}

#[aoc_generator(day17, part2)]
fn day17_part2_generator(input: &str) -> PocketDimension4D {
  let mut dimensions = PocketDimension4D {
    dimensions: HashSet::with_capacity(10000),
    min_x: 0,
    min_y: 0,
    min_z: 0,
    min_w: 0,
    max_x: 0,
    max_y: 0,
    max_z: 0,
    max_w: 0,
    additions: Default::default(),
    deletions: Default::default(),
  };
  input.lines().enumerate().for_each(|(x, l)| {
    l.chars().enumerate().for_each(|(y, c)| {
      if c == '#' {
        dimensions.set(Coord4D(x as i32, y as i32, 0, 0));
      }
    })
  });
  dimensions.step();
  dimensions
}

#[aoc(day17, part2)]
fn day17_part2(dimensions: &PocketDimension4D) -> usize {
  let mut dimensions = dimensions.clone();
  for _ in 0..6 {
    // println!("iter_space: {}", dimensions.iter_space().count());
    dimensions.iter_space().for_each(|c| {
      let active_neighbors = c.neighbors().filter(|c| dimensions.dimensions.contains(c)).count();
      if dimensions.dimensions.contains(&c) {
        match active_neighbors {
          2 | 3 => {}
          _ => dimensions.unset(c),
        }
      } else {
        if active_neighbors == 3 {
          dimensions.set(c);
        }
      }
    });
    dimensions.step();
  }
  // dbg!(&dimensions);
  dimensions.dimensions.len()
}
