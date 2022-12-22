use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../inputs/day-8/input.txt");
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        let row = line
            .split("")
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();
        grid.push(row);
    }
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    get_visible(&grid, &mut visible, false);
    get_visible(&transpose(&grid), &mut visible, true);

    dbg!(visible.len());
}

fn get_visible(grid: &Vec<Vec<usize>>, set: &mut HashSet<(usize, usize)>, transposed: bool) {
    for (x, row) in grid.iter().enumerate() {
        let mut row_max: usize = row[0];
        for (y, col) in row.iter().enumerate() {
            // add perimeter
            if (x == 0 || x == grid.len() - 1) || (y == 0 || y == row.len() - 1) {
                set.insert((x, y));
                continue;
            }

            // add left to right
            if *col > row_max {
                if transposed {
                    set.insert((y, x));
                } else {
                    set.insert((x, y));
                }
                row_max = *col;
                continue;
            }
        }
    }

    // add right to left
    for (x, row) in grid.iter().enumerate().rev() {
        let mut row_max: usize = row[row.len() - 1];
        for (y, col) in row.iter().enumerate().rev() {
            if *col > row_max {
                if transposed {
                    set.insert((y, x));
                } else {
                    set.insert((x, y));
                }
                row_max = *col;
            }
        }
    }
}

fn transpose(grid: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut gprime: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];

    for (x, _row) in grid.iter().enumerate() {
        for (y, _col) in grid[0].iter().enumerate() {
            gprime[x][y] = grid[y][x]
        }
    }

    gprime
}
