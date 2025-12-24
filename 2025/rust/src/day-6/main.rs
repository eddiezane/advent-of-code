use advent_of_code::util::grid::{Grid, left_transpose};

fn main() {
    let input = include_str!("../../../inputs/day-6/input.txt");
    let grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let tgrid = left_transpose(&grid);

    let mut prob: Vec<u64> = Vec::new();
    let mut answer: u64 = 0;

    for row in tgrid {
        if row.iter().all(|c| c.is_whitespace()) {
            continue;
        }

        let (op, rest) = row.split_last().unwrap();

        let num_str: String = rest.iter().filter(|c| c.is_ascii_digit()).collect();

        if let Ok(num) = num_str.parse::<u64>() {
            prob.push(num);
        }

        if *op != ' ' {
            let sub_answer: u64 = match *op {
                '+' => prob.iter().sum(),
                '*' => prob.iter().product(),
                _ => panic!("unknown op: {op}"),
            };
            answer += sub_answer;
            prob.clear();
        }
    }

    println!("{answer}");
}
