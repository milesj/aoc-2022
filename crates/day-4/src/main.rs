type Assignment = (u8, u8);

fn get_assignment_range(pair: &str) -> Assignment {
    let dash = pair.find('-').unwrap();

    (
        pair[0..dash].parse::<u8>().unwrap(),
        pair[(dash + 1)..].parse::<u8>().unwrap(),
    )
}

fn in_range(value: u8, range: &Assignment) -> bool {
    value >= range.0 && value <= range.1
}

fn part_1() {
    let data = include_str!("input.txt");

    let total = data
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line.split(',');

            (
                get_assignment_range(parts.next().unwrap()),
                get_assignment_range(parts.next().unwrap()),
            )
        })
        .filter(|assignments| {
            let a = assignments.0;
            let b = assignments.1;

            a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1
        })
        .collect::<Vec<_>>();

    println!("total = {}", total.len());
}

fn part_2() {
    let data = include_str!("input.txt");

    let total = data
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line.split(',');

            (
                get_assignment_range(parts.next().unwrap()),
                get_assignment_range(parts.next().unwrap()),
            )
        })
        .filter(|assignments| {
            let a = assignments.0;
            let b = assignments.1;

            in_range(b.0, &a) || in_range(b.1, &a) || in_range(a.0, &b) || in_range(a.1, &b)
        })
        .collect::<Vec<_>>();

    println!("total = {}", total.len());
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}
