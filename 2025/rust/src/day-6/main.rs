use advent_of_code::util::grid::{Grid, transpose};

fn main() {
    let input = include_str!("../../../inputs/day-6/input.txt");
    let grid: Grid<&str> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let tgrid = transpose(&grid);

    let answer = tgrid.into_iter().fold(0_u64, |acc, row| {
        let (op, rest) = row.split_last().unwrap();
        let (first, middle) = rest.split_first().unwrap();
        let mut total: u64 = first.parse().unwrap();

        for col in middle {
            let num: u64 = col.parse().unwrap();
            match *op {
                "+" => total += num,
                "*" => total *= num,
                _ => panic!("got unknown op: {op}"),
            }
        }
        acc + total
    });

    println!("{answer}");
}
