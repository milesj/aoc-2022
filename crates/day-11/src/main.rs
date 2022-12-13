use std::{collections::VecDeque, str::FromStr, time::Instant};

type WorryLevel = u128;

#[derive(Debug)]
enum Value {
    Old,
    New(WorryLevel),
}

impl Value {
    pub fn get_value(&self) -> Option<&WorryLevel> {
        match self {
            Self::Old => None,
            Self::New(value) => Some(value),
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if s == "old" {
            return Ok(Value::Old);
        }

        Ok(Value::New(s.parse().unwrap()))
    }
}

#[derive(Debug, Default)]
enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
    #[default]
    None,
}

#[derive(Debug)]
struct Item(WorryLevel);

#[derive(Debug, Default)]
struct Monkey {
    // pub index: usize,
    pub inspect_count: usize,
    pub items: VecDeque<Item>,
    operation: Operation,
    divisible_by: usize,
    to_truthy: usize,
    to_falsy: usize,
}

impl Monkey {
    pub fn inspect_item(&mut self, old_value: &WorryLevel) -> WorryLevel {
        self.inspect_count += 1;

        match &self.operation {
            Operation::Add(lhs, rhs) => {
                lhs.get_value().unwrap_or(old_value) + rhs.get_value().unwrap_or(old_value)
            }
            Operation::Multiply(lhs, rhs) => {
                lhs.get_value().unwrap_or(old_value) * rhs.get_value().unwrap_or(old_value)
            }
            Operation::None => *old_value,
        }
    }

    pub fn throws_to(&self, worry_level: &WorryLevel) -> usize {
        if worry_level % (self.divisible_by as WorryLevel) == 0 {
            self.to_truthy
        } else {
            self.to_falsy
        }
    }
}

fn parse_monkeys(data: &str) -> (Vec<Monkey>, usize) {
    let mut monkeys = vec![];

    for chunk in data.trim().split("\n\n") {
        let mut monkey = Monkey::default();

        for line in chunk.lines() {
            let line = line.trim();
            let colon_index = line.find(':').unwrap();
            let last_space_index = line.rfind(' ').unwrap();

            if line.starts_with("Starting items:") {
                for item_no in line[colon_index + 2..].split(',') {
                    monkey
                        .items
                        .push_back(Item(item_no.trim().parse().unwrap()));
                }
            } else if line.starts_with("Operation:") {
                let equal_index = line.find('=').unwrap();
                let mut parts = line[equal_index + 2..].split(' ');
                let lhs = Value::from_str(parts.next().unwrap()).unwrap();
                let op = parts.next().unwrap();
                let rhs = Value::from_str(parts.next().unwrap()).unwrap();

                match op {
                    "+" => {
                        monkey.operation = Operation::Add(lhs, rhs);
                    }
                    "*" => {
                        monkey.operation = Operation::Multiply(lhs, rhs);
                    }
                    _ => {}
                }
            } else if line.starts_with("Test:") {
                monkey.divisible_by = line[last_space_index + 1..].parse().unwrap();
            } else if line.starts_with("If true:") {
                monkey.to_truthy = line[last_space_index + 1..].parse().unwrap();
            } else if line.starts_with("If false:") {
                monkey.to_falsy = line[last_space_index + 1..].parse().unwrap();
            }
        }

        monkeys.push(monkey);
    }

    let product: usize = monkeys.iter().map(|m| m.divisible_by).product();

    (monkeys, product)
}

// fn debug_monkeys(monkeys: &Vec<Monkey>) {
//     for (i, monkey) in monkeys.iter().enumerate() {
//         println!("{} = {} items", i, monkey.items.len());
//     }
// }

fn play_with_monkeys<F>(monkeys: &mut Vec<Monkey>, level_reduction: F, rounds: usize)
where
    F: Fn(WorryLevel) -> WorryLevel,
{
    for _ in 0..rounds {
        let mut i = 0;

        while i < monkeys.len() {
            let mut move_items = vec![];

            // Inspect monkey in a block because of mutability
            {
                let monkey = &mut monkeys[i];

                while let Some(item) = monkey.items.pop_front() {
                    let worry_level = level_reduction(monkey.inspect_item(&item.0));

                    move_items.push((monkey.throws_to(&worry_level), Item(worry_level)));
                }
            }

            // Move items around
            for item in move_items {
                let monkey = &mut monkeys[item.0];
                monkey.items.push_back(item.1);
            }

            i += 1;
        }
    }

    monkeys.sort_by(|a, d| d.inspect_count.cmp(&a.inspect_count));
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

fn part_1() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let (mut monkeys, _) = parse_monkeys(data);

    play_with_monkeys(&mut monkeys, |level| level / 3, 20);

    println!("monkey business = {}", calculate_monkey_business(&monkeys));
    println!("{:?}", time.elapsed());
}

fn part_2() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let (mut monkeys, product) = parse_monkeys(data);

    play_with_monkeys(&mut monkeys, |level| level % product as u128, 10000);

    println!("monkey business = {}", calculate_monkey_business(&monkeys));
    println!("{:?}", time.elapsed());
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let data = include_str!("input-test.txt");
        let (mut monkeys, _) = parse_monkeys(data);

        play_with_monkeys(&mut monkeys, |level| level / 3, 20);

        assert_eq!(calculate_monkey_business(&monkeys), 10605);
    }

    #[test]
    fn works_big() {
        let data = include_str!("input-test.txt");
        let (mut monkeys, product) = parse_monkeys(data);

        play_with_monkeys(&mut monkeys, |level| level % product as u128, 10000);

        assert_eq!(calculate_monkey_business(&monkeys), 2713310158);
    }
}
