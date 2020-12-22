use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
enum Player {
  P1,
  P2,
}

#[derive(Debug, Clone)]
struct Game {
  player1: VecDeque<u8>,
  player2: VecDeque<u8>,
}

impl Game {
  fn play(&mut self) {
    while !self.player1.is_empty() && !self.player2.is_empty() {
      let p1 = self.player1.pop_front().unwrap();
      let p2 = self.player2.pop_front().unwrap();
      if p1 > p2 {
        self.player1.push_back(p1);
        self.player1.push_back(p2);
      } else {
        self.player2.push_back(p2);
        self.player2.push_back(p1);
      }
    }
  }
  fn play_recursive(&mut self) -> Player {
    // println!("playing new game: {:?}", self);

    let mut previously_seen = HashSet::new();
    loop {
      // println!("Decks: {:?}, {:?}", self.player1, self.player2);
      if self.player1.is_empty() || self.player2.is_empty() {
        break;
      }
      let state = (self.player1.clone(), self.player2.clone());
      if previously_seen.contains(&state) {
        return Player::P1;
      }
      previously_seen.insert(state);
      let p1 = self.player1.pop_front().unwrap();
      let p2 = self.player2.pop_front().unwrap();
      let winner = if self.player1.len() >= p1 as usize && self.player2.len() >= p2 as usize {
        let mut new_game = Game {
          player1: self.player1.iter().copied().take(p1 as usize).collect(),
          player2: self.player2.iter().copied().take(p2 as usize).collect(),
        };
        new_game.play_recursive()
      } else if p1 > p2 {
        Player::P1
      } else {
        Player::P2
      };
      match winner {
        Player::P1 => {
          self.player1.push_back(p1);
          self.player1.push_back(p2);
        }
        Player::P2 => {
          self.player2.push_back(p2);
          self.player2.push_back(p1);
        }
      }
    }

    let winner = if self.player1.is_empty() { Player::P2 } else { Player::P1 };
    // println!("Finished game in favor of {:?}", winner);
    winner
  }
}

#[aoc_generator(day22)]
fn day22_generator(input: &str) -> Game {
  let mut cards = input
    .split("\n\n")
    .map(|p| p.lines().skip(1).map(|l| l.parse::<u8>().unwrap()).collect::<VecDeque<_>>())
    .collect::<Vec<_>>()
    .into_iter();
  Game {
    player1: cards.next().unwrap(),
    player2: cards.next().unwrap(),
  }
}

#[aoc(day22, part1)]
fn day22_part1(game: &Game) -> u64 {
  let mut game = game.clone();
  game.play();
  let player = if !game.player1.is_empty() { &game.player1 } else { &game.player2 };
  player
    .iter()
    .copied()
    .rev()
    .enumerate()
    .map(|(i, c)| c as u64 * (1 + i as u64))
    .sum()
}

#[aoc(day22, part2)]
fn day22_part2(game: &Game) -> u64 {
  let mut game = game.clone();
  game.play_recursive();
  let player = if !game.player1.is_empty() { &game.player1 } else { &game.player2 };
  player
    .iter()
    .copied()
    .rev()
    .enumerate()
    .map(|(i, c)| c as u64 * (1 + i as u64))
    .sum()
}
