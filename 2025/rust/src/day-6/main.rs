use std::fmt;

type Grid<'a> = Vec<Vec<&'a str>>;

#[allow(dead_code)]
struct DisplayGrid<'a>(&'a Grid<'a>);

impl<'a> fmt::Display for DisplayGrid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            let s: String = row.join(" ");
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../../../inputs/day-6/input.txt");
    let grid: Grid = input
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

fn transpose<'a>(grid: &'a Grid) -> Grid<'a> {
    let width = grid[0].len();

    (0..width)
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect()
}
