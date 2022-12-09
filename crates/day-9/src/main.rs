use std::{collections::HashSet, str::FromStr, time::Instant};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!(),
        })
    }
}

type Move = (Direction, u16);

#[derive(Debug)]
struct Bridge {
    head_pos: (u16, u16),
    head_history: Vec<(u16, u16)>,
    tail_pos: (u16, u16),
    tail_history: Vec<(u16, u16)>,
}

impl Bridge {
    pub fn new(x: u16, y: u16) -> Self {
        Bridge {
            head_pos: (x, y),
            head_history: vec![],
            tail_pos: (x, y),
            tail_history: vec![],
        }
    }

    // 0,0  1,0  2,0  3,0
    // 0,1  1,1  2,1  3,1
    // 0,2  1,2  2,2  3,2
    // 0,3  1,3  2,3  3,3
    fn is_head_tail_touching(&self) -> bool {
        let (hx, hy) = self.head_pos;
        let (tx, ty) = self.tail_pos;

        // Check X
        if hx < tx - 1 || hx > tx + 1 {
            return false;
        }

        // Check Y
        if hy < ty - 1 || hy > ty + 1 {
            return false;
        }

        true
    }

    pub fn count_tail_positions(&self) -> usize {
        let mut set: HashSet<&(u16, u16)> = HashSet::from_iter(&self.tail_history);
        set.insert(&self.tail_pos);
        set.len()
    }

    pub fn run_moves(&mut self, moves: Vec<Move>) {
        for (dir, count) in moves {
            for _ in 1..=count {
                self.head_history.push(self.head_pos);

                match dir {
                    Direction::Up => {
                        self.head_pos.1 -= 1;
                    }
                    Direction::Right => {
                        self.head_pos.0 += 1;
                    }
                    Direction::Down => {
                        self.head_pos.1 += 1;
                    }
                    Direction::Left => {
                        self.head_pos.0 -= 1;
                    }
                }

                if !self.is_head_tail_touching() {
                    self.tail_history.push(self.tail_pos);
                    self.tail_pos = *self.head_history.last().unwrap();
                }
            }
        }
    }
}

fn parse_moves(data: &str) -> Vec<Move> {
    data.trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let letter = parts.next().unwrap();
            let count: u16 = parts.next().unwrap().parse().unwrap();

            (letter.parse::<Direction>().unwrap(), count)
        })
        .collect::<Vec<_>>()
}

fn part_1() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let mut bridge = Bridge::new(255, 255);

    bridge.run_moves(parse_moves(data));

    println!("tail positions = {}", bridge.count_tail_positions());
    println!("{:?}", time.elapsed());
}

// fn part_2() {
//     let data = include_str!("input.txt");
//     let time = Instant::now();
//     let bridge = Bridge::new(100, 100);
//     let moves = parse_moves(data);

//     println!("{:?}", time.elapsed());
// }

fn main() {
    part_1();
    println!("-----");
    // part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_example() {
        let mut bridge = Bridge::new(10, 14);
        let moves = parse_moves(
            r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
        );

        bridge.run_moves(moves);

        assert_eq!(bridge.count_tail_positions(), 13);
    }
}
