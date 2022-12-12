fn main() {
    let input = include_str!("../../../inputs/day-5/input.txt");
    let mut lines = input.lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    // read to stack number line
    for line in lines.by_ref() {
        if line.starts_with(" 1") {
            break;
        }
        let mut chars = line.chars();
        let it = chars.by_ref();
        let mut idx: usize = 0;
        loop {
            let c: Vec<char> = it.take(3).collect();
            print!("stack {idx} ");
            println!("{:?}", c);
            if stacks.len() <= idx {
                let s: Vec<char> = Vec::new();
                stacks.push(s);
            }
            let letter = c.iter().nth(1).unwrap();
            let stack = stacks.get_mut(idx).unwrap();
            if *letter != ' ' {
                stack.push(*letter);
            }

            if it.next().is_none() {
                break;
            }

            idx += 1;
        }
    }
    println!("\n{:?}", stacks);

    // skip blank line
    lines.next();

    dbg!(&stacks);

    // read instrctions
    for line in lines {
        let inst: Vec<&str> = line.split(" ").collect();
        let amount = inst.get(1).unwrap().parse::<usize>().unwrap();
        let from = inst.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = inst.get(5).unwrap().parse::<usize>().unwrap() - 1;

        // let mut from_stack = &mut stacks[from];
        // let mut to_stack = &mut stacks[to];

        println!("move {amount} from {} to {}", from + 1, to + 1);
        dbg!(&stacks[from]);

        let xfer = stacks[from].drain(0..amount).rev().collect::<Vec<char>>();

        dbg!(&xfer);

        for s in xfer {
            stacks[to].insert(0, s);
        }
        dbg!(&stacks);
    }

    let out: String = stacks.iter().map(|s| s.first().unwrap()).collect();
    println!("{out}");
}
