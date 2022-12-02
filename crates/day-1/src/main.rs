fn part_1() {
    let data = include_str!("input.txt");
    let mut highest = 0;

    for elf in data.split("\n\n") {
        let elf_calories: i32 = elf
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<i32>().unwrap())
            .sum();

        println!("elf = {}", elf_calories);

        if elf_calories > highest {
            highest = elf_calories;
        }
    }

    println!("highest = {}", highest);
}

fn part_2() {
    let data = include_str!("input.txt");
    let mut calories = vec![];

    for elf in data.split("\n\n") {
        let elf_calories: i32 = elf
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<i32>().unwrap())
            .sum();

        println!("elf = {}", elf_calories);

        calories.push(elf_calories);
    }

    calories.sort();

    let top3 = &calories[(calories.len() - 3)..];
    let top_calories: i32 = top3.iter().sum();

    println!("top 3 = {:?}", top3);
    println!("highest = {}", top_calories);
}

fn main() {
    part_2();
}
