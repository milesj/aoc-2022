use std::collections::HashMap;

fn get_priority_map() -> HashMap<char, u32> {
    let mut map = HashMap::with_capacity(52);
    let mut count = 1;

    for c in 'a'..='z' {
        map.insert(c, count);
        count += 1;
    }

    for c in 'A'..='Z' {
        map.insert(c, count);
        count += 1;
    }

    map
}

fn find_shared_item_type(comp1: &str, comp2: &str) -> Option<char> {
    for c in comp1.chars() {
        if comp2.contains(c) {
            return Some(c);
        }
    }

    None
}

fn find_group_shared_item_type(comps: &[&str]) -> Option<char> {
    for c in comps[0].chars() {
        if comps[1].contains(c) && comps[2].contains(c) {
            return Some(c);
        }
    }

    None
}

fn part_1() {
    let data = include_str!("input.txt");
    let priorities = get_priority_map();

    let sum: u32 = data
        .trim()
        .split("\n")
        .map(|rucksack| {
            let half = rucksack.len() / 2;
            let comp1 = &rucksack[0..half];
            let comp2 = &rucksack[half..];
            let item =
                find_shared_item_type(comp1, comp2).expect("Unable to find a common item type!");

            priorities
                .get(&item)
                .expect(&format!("Cannot find priority for char {}", item))
        })
        .sum();

    println!("sum = {}", sum);
}

fn part_2() {
    let data = include_str!("input.txt");
    let priorities = get_priority_map();
    let rucksacks = data.trim().split("\n").collect::<Vec<_>>();
    let mut groups = vec![];
    let mut count = 0;

    while count < rucksacks.len() {
        groups.push(&rucksacks[count..(count + 3)]);
        count += 3;
    }

    let sum: u32 = groups
        .iter()
        .map(|comps| {
            let item =
                find_group_shared_item_type(comps).expect("Unable to find a common item type!");

            priorities
                .get(&item)
                .expect(&format!("Cannot find priority for char {}", item))
        })
        .sum();

    println!("sum = {}", sum);
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}
