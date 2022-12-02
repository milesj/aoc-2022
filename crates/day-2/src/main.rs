#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    // part 2
    pub fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown value"),
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    pub fn infer_hand(&self, opponent: Hand) -> Hand {
        match self {
            Outcome::Loss => match opponent {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            Outcome::Draw => opponent.clone(),
            Outcome::Win => match opponent {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
        }
    }
}

#[derive(Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    // part 1
    pub fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Unknown value"),
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn play(&self, opponent: Hand) -> Outcome {
        match self {
            Hand::Rock => match opponent {
                Hand::Rock => Outcome::Draw,
                Hand::Paper => Outcome::Loss,
                Hand::Scissors => Outcome::Win,
            },
            Hand::Paper => match opponent {
                Hand::Rock => Outcome::Win,
                Hand::Paper => Outcome::Draw,
                Hand::Scissors => Outcome::Loss,
            },
            Hand::Scissors => match opponent {
                Hand::Rock => Outcome::Loss,
                Hand::Paper => Outcome::Win,
                Hand::Scissors => Outcome::Draw,
            },
        }
    }
}

fn part_1() {
    let data = include_str!("input.txt");
    let mut score: u32 = 0;

    for row in data.trim().split("\n") {
        let opponent = Hand::from(&row[0..1]);
        let me = Hand::from(&row[2..3]);
        let outcome = me.play(opponent);

        score += outcome.points() + me.points();
    }

    println!("score = {}", score);
}

fn part_2() {
    let data = include_str!("input.txt");
    let mut score: u32 = 0;

    for row in data.trim().split("\n") {
        let opponent = Hand::from(&row[0..1]);
        let outcome = Outcome::from(&row[2..3]);
        let me = outcome.infer_hand(opponent);

        score += outcome.points() + me.points();
    }

    println!("score = {}", score);
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}
