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
            interesting.insert(cycle, cycle * register);
        }
        draw(cycle, register);

        match inst.op {
            Op::Noop => {}
            Op::Addx => {
                cycle += 1;
                if check_cycle(cycle) {
                    interesting.insert(cycle, cycle * register);
                }
                draw(cycle, register);

                register += inst.value.unwrap();
            }
        };
    }

    let sum: i32 = interesting.values().sum();
    println!("\n\n{sum}");
}

fn check_cycle(cycle: i32) -> bool {
    cycle == 20 || cycle % 40 == 20
}

fn draw(cycle: i32, register: i32) {
    if (cycle - 1) % 40 == 0 {
        println!();
    }
    let head = register - 1;
    let tail = register + 1;
    let pos = (cycle % 40) - 1;

    if pos == register || pos == head || pos == tail {
        print!("#");
    } else {
        print!(".");
    }
}
