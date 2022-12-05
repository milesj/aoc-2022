use std::{
    collections::{BTreeMap, VecDeque},
    str::FromStr,
};

type Cargo<'l> = BTreeMap<u8, VecDeque<&'l str>>;

#[derive(Debug)]
struct Move {
    count: u8,
    from: u8,
    to: u8,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let mut i = 1;
        let mut data = Move {
            count: 0,
            from: 0,
            to: 0,
        };

        while i <= 3 {
            parts.next(); // Skip word
            let value = parts.next().unwrap().parse::<u8>().unwrap();

            match i {
                1 => {
                    data.count = value;
                }
                2 => {
                    data.from = value;
                }
                3 => {
                    data.to = value;
                }
                _ => {}
            }

            i += 1;
        }

        Ok(data)
    }
}

fn parse_input<'l>() -> (Cargo<'l>, Vec<Move>) {
    let data = include_str!("input.txt");
    let mut cargo: Cargo = BTreeMap::new();
    let mut moves = vec![];
    let mut parse_moves = false;

    for line in data.lines() {
        // End of cargo, now parsing moves
        if line.is_empty() {
            parse_moves = true;
            continue;
        }

        // Parse moves
        if parse_moves {
            moves.push(line.parse::<Move>().unwrap());

        // Parse cargo (janky but works)
        } else if !line.starts_with(" 1") {
            let mut curr_col = 0;
            let mut cargo_col = 1;

            while curr_col < line.len() {
                // Remove brackets
                let item = &line[(curr_col + 1)..(curr_col + 2)];

                if !item.trim().is_empty() {
                    if let Some(items) = cargo.get_mut(&cargo_col) {
                        items.push_front(item);
                    } else {
                        cargo.insert(cargo_col, VecDeque::from([item]));
                    }
                }

                curr_col += 4;
                cargo_col += 1;
            }
        }
    }

    (cargo, moves)
}

fn get_result(cargo: &Cargo) -> String {
    cargo
        .values()
        .map(|stack| stack.back().unwrap().to_owned())
        .collect::<Vec<_>>()
        .join("")
}

fn part_1() {
    let (mut cargo, moves) = parse_input();

    for action in moves {
        let mut buffer = {
            let from = cargo.get_mut(&action.from).expect("Missing from!");

            // Extract tail up to count
            from.split_off(from.len() - action.count as usize)
        };

        {
            let to = cargo.get_mut(&action.to).expect("Missing to!");

            // Push in reverse
            while !buffer.is_empty() {
                to.push_back(buffer.remove(buffer.len() - 1).unwrap());
            }
        }
    }

    let result = get_result(&cargo);

    println!("result = {}", result);
}

fn part_2() {
    let (mut cargo, moves) = parse_input();

    for action in moves {
        let mut buffer = {
            let from = cargo.get_mut(&action.from).expect("Missing from!");

            // Extract tail up to count
            from.split_off(from.len() - action.count as usize)
        };

        {
            let to = cargo.get_mut(&action.to).expect("Missing to!");
            to.append(&mut buffer);
        }
    }

    let result = get_result(&cargo);

    println!("result = {}", result);
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}
