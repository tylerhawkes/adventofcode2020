struct Forest {
  grid: Vec<Vec<Grid>>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Grid {
  Tree,
  Empty,
}

#[aoc_generator(day3)]
fn parse_forest(input: &str) -> Forest {
  let grid = input
    .lines()
    .map(|l| {
      l.chars()
        .map(|c| match c {
          '.' => Grid::Empty,
          '#' => Grid::Tree,
          _ => unreachable!(),
        })
        .collect()
    })
    .collect();
  Forest { grid }
}

#[aoc(day3, part1)]
fn day3_part1(forest: &Forest) -> usize {
  forest
    .grid
    .iter()
    .enumerate()
    .filter(|(i, l)| {
      let x = i * 3;
      l[x.wrapping_rem_euclid(l.len())] == Grid::Tree
    })
    .count()
}

#[aoc(day3, part2)]
fn day3_part2(forest: &Forest) -> usize {
  let x_1_3_5_7 = forest
    .grid
    .iter()
    .enumerate()
    .map(|(i, l)| {
      let mut res = [0_usize, 0, 0, 0];
      for j in [1, 3, 5, 7].iter().copied() {
        let x = i * j;
        if l[x.wrapping_rem_euclid(l.len())] == Grid::Tree {
          res[j / 2] = 1;
        }
      }
      res
    })
    .fold([0, 0, 0, 0], |l, r| [l[0] + r[0], l[1] + r[1], l[2] + r[2], l[3] + r[3]]);
  let x_2_1 = forest
    .grid
    .iter()
    .enumerate()
    .filter(|(i, _)| *i & 1 == 0)
    .enumerate()
    .filter(|(i, (_, l))| {
      let x = *i;
      l[x.wrapping_rem_euclid(l.len())] == Grid::Tree
    })
    .count();
  x_1_3_5_7.iter().copied().product::<usize>() * x_2_1
}
