use std::fmt::Display;

#[derive(Debug)]
struct Item {
    worry_level: i32,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "worry_level: {}", self.worry_level)
    }
}

struct Monkey {
    id: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Operation,
    inspections: i32,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.id, self.items)
    }
}

type Operation = Box<dyn Fn(i32) -> i32>;

fn main() {
    let input = include_str!("../../../inputs/day-11/input.txt");
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .enumerate()
        .map(|(mn, line)| {
            let s: Vec<&str> = line[1].split(": ").collect();
            let items: Vec<Item> = s[1]
                .split(", ")
                .map(|i| Item {
                    worry_level: i.parse::<i32>().unwrap(),
                })
                .collect();

            let o = line[2].split(": ").last().unwrap();
            let oo = o.split("new = old ").last().unwrap();
            let ooo: Vec<&str> = oo.split(' ').collect();
            let operation: Operation = match ooo[..] {
                ["*", "old"] => Box::new(|i: i32| -> i32 { i * i }),
                ["*", n] => Box::new(|i: i32| -> i32 { i * n.parse::<i32>().unwrap() }),
                ["+", n] => Box::new(|i: i32| -> i32 { i + n.parse::<i32>().unwrap() }),
                _ => panic!("unexpected: {ooo:?}"),
            };

            let div = line[3].split("by ").last().unwrap().parse::<i32>().unwrap();
            let ift = line[4]
                .split("monkey ")
                .last()
                .map(|i| i.parse::<i32>().unwrap())
                .unwrap();
            let iff = line[5]
                .split("monkey ")
                .last()
                .map(|i| i.parse::<i32>().unwrap())
                .unwrap();
            let test: Operation = Box::new(move |i: i32| -> i32 {
                if i % div == 0 {
                    ift
                } else {
                    iff
                }
            });

            Monkey {
                id: mn,
                items,
                operation,
                test,
                inspections: 0,
            }
        })
        .collect();

    for round in 0..20 {
        println!("round: {}", round + 1);
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let mut inspections = 0;
            let items = monkey
                .items
                .drain(..)
                .map(|mut i| {
                    i.worry_level = (monkey.operation)(i.worry_level);
                    inspections += 1;
                    i
                })
                .map(|mut i| {
                    i.worry_level /= 3;
                    i
                })
                .map(|i| {
                    let target = (monkey.test)(i.worry_level) as usize;
                    (i, target)
                })
                .collect::<Vec<(Item, usize)>>();
            monkey.inspections += inspections;

            for (item, target) in items {
                monkeys[target].items.push(item);
            }
        }
        println!();
    }

    let mut inspection_counts: Vec<i32> = monkeys.iter().map(|m| m.inspections).collect();
    inspection_counts.sort();
    inspection_counts.reverse();

    println!("{inspection_counts:?}");

    // 23712 too low
    println!("{}", inspection_counts[0] * inspection_counts[1]);
}
