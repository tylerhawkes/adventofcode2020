use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Food {
  ingredients: HashSet<String>,
  allergens: HashSet<String>,
}

#[aoc_generator(day21)]
fn day21_generator(input: &str) -> Vec<Food> {
  input
    .replace('(', "")
    .replace(')', "")
    .lines()
    .map(|l| {
      let mut split = l.split(" contains ");
      let ingredients = split.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
      let allergens = split.next().unwrap().split(", ").map(|s| s.to_string()).collect();
      Food { ingredients, allergens }
    })
    .collect()
}

#[aoc(day21, part1)]
fn day21_part1(foods: &[Food]) -> usize {
  let mut all_allergens = HashSet::new();
  let mut all_ingredients = HashSet::new();
  foods.iter().for_each(|f| {
    f.allergens.iter().for_each(|a| {
      all_allergens.insert(a.clone());
    });
    f.ingredients.iter().for_each(|i| {
      all_ingredients.insert(i.clone());
    });
  });
  let mut allergen_names = all_allergens
    .iter()
    .map(|a| {
      let ingredients = foods
        .iter()
        .filter(|f| f.allergens.contains(a))
        .fold(all_ingredients.clone(), |a, f| a.intersection(&f.ingredients).cloned().collect());
      (a.clone(), ingredients)
    })
    .collect::<HashMap<_, _>>();
  while !allergen_names.values().all(|i| i.len() == 1) {
    let single_possibilities = allergen_names
      .values()
      .filter_map(|i| if i.len() == 1 { i.iter().cloned().next() } else { None })
      .collect::<Vec<_>>();
    allergen_names.values_mut().filter(|v| v.len() != 1).for_each(|i| {
      single_possibilities.iter().for_each(|p| {
        i.remove(p);
      });
    });
  }
  let mut non_allergenic_ingredients = all_ingredients.clone();
  allergen_names.values().for_each(|v| {
    v.iter().for_each(|i| {
      non_allergenic_ingredients.remove(i);
    });
  });
  let mut allergens = allergen_names
    .iter()
    .map(|(k, v)| (k.clone(), v.iter().cloned().next().unwrap()))
    .collect::<Vec<_>>();
  allergens.sort_unstable_by_key(|(k, _)| k.clone());
  allergens.iter().for_each(|(_, v)| print!("{},", v));
  println!();
  foods
    .iter()
    .map(|f| f.ingredients.intersection(&non_allergenic_ingredients).count())
    .sum()
}

#[aoc(day21, part2)]
fn day21_part2(_foods: &[Food]) -> &'static str {
  "See output from part 1"
}
