use std::{
    collections::{HashMap, VecDeque},
    str,
    time::Instant,
};

fn process(data: &str) -> (HashMap<i32, i32>, Vec<String>) {
    let mut cycle = 1;
    let mut cycle_inner = 1;
    let mut cycle_registers = HashMap::new();
    let mut register = 1;
    let mut instructions = VecDeque::from_iter(data.trim().lines());
    let mut next_add: Option<i32> = None;
    let mut pixels: Vec<char> = vec![];
    let mut crt_index = 0;

    while cycle <= 240 {
        if next_add.is_none() && !instructions.is_empty() {
            let instr = instructions.pop_front().unwrap();

            if instr != "noop" {
                next_add = Some(instr[5..].parse().unwrap());
                cycle_inner = 0;
            }
        }

        if crt_index >= register - 1 && crt_index <= register + 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }

        // Track per cycle amounts
        if cycle % 20 == 0 {
            cycle_registers.insert(cycle, register);
        }

        // Increment cycles and apply any registers
        cycle += 1;
        cycle_inner += 1;
        crt_index += 1;

        if crt_index == 40 {
            crt_index = 0;
        }

        if next_add.is_some() && cycle_inner == 2 {
            register += next_add.unwrap();
            next_add = None;
        }
    }

    (
        cycle_registers,
        pixels
            .chunks(40)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>(),
    )
}

fn calculate_signal_strength(registers: HashMap<i32, i32>) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for cycle in cycles {
        if let Some(register) = registers.get(&cycle) {
            sum += cycle * register;
        }
    }

    sum
}

fn part_1() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let (registers, _) = process(data);

    println!("signal strength = {}", calculate_signal_strength(registers));
    println!("{:?}", time.elapsed());
}

fn part_2() {
    let data = include_str!("input.txt");
    let time = Instant::now();
    let (_, crt) = process(data);

    for row in crt {
        println!("{}", row);
    }

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
        let (registers, crt) = process(data);

        dbg!(&registers);
        dbg!(&crt);

        assert_eq!(calculate_signal_strength(registers), 13141);
    }
}
