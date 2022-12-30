use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Noop,
    Addx,
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    value: Option<i32>,
}

fn main() {
    let input = include_str!("../../../inputs/day-10/input.txt");
    let program: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let op = match parts[0] {
                "noop" => Op::Noop,
                "addx" => Op::Addx,
                _ => panic!("unexpected op"),
            };

            let value = parts.get(1).map(|n| n.parse::<i32>().unwrap());

            Instruction { op, value }
        })
        .collect();

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut interesting: HashMap<i32, i32> =
        HashMap::from([(20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)]);

    for inst in program {
        cycle += 1;
        if check_cycle(cycle) {
            println!("cycle: {} register: {}", cycle, register);
            interesting.insert(cycle, cycle * register);
        }
        match inst.op {
            Op::Noop => {}
            Op::Addx => {
                cycle += 1;
                if check_cycle(cycle) {
                    println!("cycle: {} register: {}", cycle, register);
                    interesting.insert(cycle, cycle * register);
                }
                register += inst.value.unwrap();
            }
        };
    }

    let sum: i32 = interesting.values().sum();
    println!("{:?}", interesting);
    println!("{sum}");
}

fn check_cycle(cycle: i32) -> bool {
    cycle == 20 || cycle % 40 == 20
}
