use std::fmt;

type Grid = Vec<Vec<String>>;

struct DisplayGrid(Grid);

impl fmt::Display for DisplayGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            write!(f, "{}\n", row.join(" "))?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../../../inputs/day-4/example.txt");
    let mut grid: Grid = Vec::new();

    for line in input.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        grid.push(row);
    }
    print_grid(&grid);
}

fn print_grid(grid: &Grid) {
    println!("{}", DisplayGrid(grid.to_vec()));
}
