use std::fmt;

type Grid = Vec<Vec<String>>;

struct DisplayGrid<'a>(&'a Grid);

impl<'a> fmt::Display for DisplayGrid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            writeln!(f, "{}", row.join(""))?;
        }
        Ok(())
    }
}

const DELTAS: &[(i32, i32)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

fn main() {
    let input = include_str!("../../../inputs/day-4/input.txt");

    let grid: Grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().map(String::from).collect())
        .collect();

    // println!("{}", DisplayGrid(&grid));

    let mut accessible = 0_i32;

    for (y, row) in grid.iter().enumerate() {
        // println!("row");
        for (x, cell) in row.iter().enumerate() {
            // println!("cell");
            let mut adj = 0_i32;
            // println!("{x},{y},{cell}\n===");
            for &(dx, dy) in DELTAS {
                let ndx = x as i32 + dx;
                let ndy = y as i32 + dy;
                // println!("{ndx},{ndy}");
                if ndx >= 0 && ndy >= 0 && ndx < row.len() as i32 && ndy < grid.len() as i32 {
                    let c = &grid[ndy as usize][ndx as usize];
                    // println!("adj: {c}");
                    if c == "@" {
                        adj += 1;
                    }
                }
            }
            // println!();
            if cell == "@" && adj < 4 {
                // println!("accessible");
                accessible += 1;
            }
            // println!("end col");
        }
        // println!("end row\n\n\n");
    }
    println!("{accessible}");
}
