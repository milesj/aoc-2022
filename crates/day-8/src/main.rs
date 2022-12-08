use std::time::Instant;

#[derive(Debug)]
struct Forest<'l> {
    trees: Vec<&'l [u8]>,
    max_x: usize,
    max_y: usize,
}

impl<'l> Forest<'l> {
    pub fn new(s: &'l str) -> Self {
        let mut x = 0;
        let grid: Vec<&[u8]> = s
            .lines()
            .map(|l| {
                if x == 0 {
                    x = l.len();
                }

                l.as_bytes()
            })
            .collect();
        let y = grid.len();

        Forest {
            trees: grid,
            max_x: x - 1,
            max_y: y - 1,
        }
    }

    pub fn find_scenic_score(&self) -> usize {
        let mut score = 0;
        let mut x = 0;
        let mut y = 0;

        while y <= self.max_y {
            while x <= self.max_x {
                let maybe_score = self.calc_score(x, y);

                if maybe_score > score {
                    score = maybe_score;
                }

                x += 1;
            }

            y += 1;
            x = 0;
        }

        score
    }

    pub fn find_visible_trees(&self) -> usize {
        let mut visible = 0;
        let mut x = 0;
        let mut y = 0;

        while y <= self.max_y {
            while x <= self.max_x {
                if self.is_visible(x, y) {
                    visible += 1;
                }

                x += 1;
            }

            y += 1;
            x = 0;
        }

        visible
    }

    pub fn get_row(&self, y: usize) -> &&[u8] {
        self.trees.get(y).expect("Unknown Y row.")
    }

    pub fn calc_score(&self, x: usize, y: usize) -> usize {
        if self.is_edge(x, y) {
            return 0;
        }

        let (before, after) = self.is_x_visible(x, y, true);
        let (above, below) = self.is_y_visible(x, y, true);

        before * after * above * below
    }

    pub fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || x == self.max_x || y == 0 || y == self.max_y
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return true;
        }

        let (before, after) = self.is_x_visible(x, y, false);
        let x_visible = before == x || after == self.max_x - x;

        if x_visible {
            return true;
        }

        let (above, below) = self.is_y_visible(x, y, false);
        let y_visible = above == y || below == self.max_y - y;

        y_visible
    }

    fn is_x_visible(&self, x: usize, y: usize, count_blocker: bool) -> (usize, usize) {
        let row = self.get_row(y);
        let tree = &row[x];

        // Check before
        let mut before = 0;
        let mut bx = x - 1;

        loop {
            if &row[bx] >= tree {
                if count_blocker {
                    before += 1;
                }

                break;
            }

            before += 1;

            if bx == 0 {
                break;
            } else {
                bx -= 1;
            }
        }

        // Check after
        let mut after = 0;
        let mut ax = x + 1;

        while ax <= self.max_x {
            if &row[ax] >= tree {
                if count_blocker {
                    after += 1;
                }

                break;
            }

            after += 1;
            ax += 1;
        }

        (before, after)
    }

    fn is_y_visible(&self, x: usize, y: usize, count_blocker: bool) -> (usize, usize) {
        let tree = &self.get_row(y)[x];

        // Check above
        let mut above = 0;
        let mut ay = y - 1;

        loop {
            if &self.get_row(ay)[x] >= tree {
                if count_blocker {
                    above += 1;
                }

                break;
            }

            above += 1;

            if ay == 0 {
                break;
            } else {
                ay -= 1;
            }
        }

        // Check below
        let mut below = 0;
        let mut by = y + 1;

        while by <= self.max_y {
            if &self.get_row(by)[x] >= tree {
                if count_blocker {
                    below += 1;
                }

                break;
            }

            below += 1;

            by += 1;
        }

        (above, below)
    }
}

fn part_1() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let forest = Forest::new(data);

    println!("visible = {}", forest.find_visible_trees());
    println!("{:?}", time.elapsed());
}

fn part_2() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let forest = Forest::new(data);

    println!("score = {}", forest.find_scenic_score());
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
    fn visible() {
        let forest = Forest::new("30373\n25512\n65332\n33549\n35390");

        // Edges
        assert!(forest.is_visible(0, 0));
        assert!(forest.is_visible(1, 0));
        assert!(forest.is_visible(2, 0));
        assert!(forest.is_visible(3, 0));
        assert!(forest.is_visible(4, 0));

        assert!(forest.is_visible(0, 1));
        assert!(forest.is_visible(4, 1));

        assert!(forest.is_visible(0, 2));
        assert!(forest.is_visible(4, 2));

        assert!(forest.is_visible(0, 3));
        assert!(forest.is_visible(4, 3));

        assert!(forest.is_visible(0, 4));
        assert!(forest.is_visible(1, 4));
        assert!(forest.is_visible(2, 4));
        assert!(forest.is_visible(3, 4));
        assert!(forest.is_visible(4, 4));

        // Inner
        assert!(forest.is_visible(1, 1));
        assert!(forest.is_visible(2, 1));
        assert!(!forest.is_visible(3, 1));

        assert!(forest.is_visible(1, 2));
        assert!(!forest.is_visible(2, 2));
        assert!(forest.is_visible(3, 2));

        assert!(!forest.is_visible(1, 3));
        assert!(forest.is_visible(2, 3));
        assert!(!forest.is_visible(3, 3));
    }

    #[test]
    fn score() {
        let forest = Forest::new("30373\n25512\n65332\n33549\n35390");

        assert_eq!(forest.find_scenic_score(), 8);
    }
}
