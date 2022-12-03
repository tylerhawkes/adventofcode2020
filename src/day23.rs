fn solver_step(cups: &mut [u32], on: u32) -> u32 {
  let val = cups[on as usize];
  val
}

#[aoc(day23, part1)]
fn day23_part1(input: &str) -> String {
  let mut order = input.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>();
  let mut index = 0;
  for _ in 0..100 {
    let val = order[index];
    let (a, b, c) = (
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
    );
    let insert_cup = order
      .iter()
      .copied()
      .filter(|o| *o < val)
      .max()
      .unwrap_or_else(|| order.iter().copied().max().unwrap());
    let insert_index = order.iter().position(|o| *o == insert_cup).unwrap() + 1;
    order.insert(insert_index, c);
    order.insert(insert_index, b);
    order.insert(insert_index, a);
    index = (order.iter().copied().position(|o| o == val).unwrap() + 1) % order.len();
  }
  let pos = order.iter().position(|o| *o == 1).unwrap();
  order
    .iter()
    .chain(order.iter())
    .skip(pos + 1)
    .take(order.len() - 1)
    .map(|o| format!("{}", o))
    .collect::<String>()
}

#[aoc(day23, part2)]
fn day23_part2(input: &str) -> u64 {
  let mut order = input.chars().map(|c| (c as u8 - b'0') as u32).collect::<Vec<_>>();
  order.extend(10..=1_000_000);
  let mut index = 0;
  for _l_p in 0..100 {
    // println!("{:3} order: {:?}", l_p, order);
    let val = order[index];
    let (a, b, c) = (
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
      order.remove(if index + 1 >= order.len() { 0 } else { index + 1 }),
    );
    let insert_cup = order
      .iter()
      .copied()
      .filter(|o| *o < val)
      .max()
      .unwrap_or_else(|| order.iter().copied().max().unwrap());
    let insert_index = order.iter().position(|o| *o == insert_cup).unwrap() + 1;
    println!(
      "{:7} - val: {:6}, index: {:6}, abc: {:6}, {:6}, {:6}, order: {:3?}, ins: {}",
      _l_p,
      val,
      index,
      a,
      b,
      c,
      &order[index.saturating_sub(4)..index + 5],
      insert_index
    );
    order.insert(insert_index, c);
    order.insert(insert_index, b);
    order.insert(insert_index, a);
    index = (order.iter().copied().position(|o| o == val).unwrap() + 1) % order.len();
  }
  let pos = order.iter().position(|o| *o == 1).unwrap();
  order[(pos + 1) % order.len()] as u64 * order[(pos + 2) % order.len()] as u64
}
