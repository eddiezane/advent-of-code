use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../../inputs/day-8/input.txt");
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let row = line
            .split("")
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        grid.push(row);
    }
    let mut visible: HashSet<(i32, i32)> = HashSet::new();

    get_visible(&grid, &mut visible, false);
    get_visible(&transpose(&grid), &mut visible, true);

    dbg!(visible.len());

    let scenic_scores: HashMap<(i32, i32), i32> = get_scenic_scores(&grid, &visible);
    dbg!(scenic_scores.values().max().unwrap());
}

fn get_visible(grid: &Vec<Vec<i32>>, set: &mut HashSet<(i32, i32)>, transposed: bool) {
    for (y, row) in grid.iter().enumerate() {
        let mut row_max: i32 = row[0];
        for (x, col) in row.iter().enumerate() {
            // add perimeter
            if (y == 0 || y == grid.len() - 1) || (x == 0 || x == row.len() - 1) {
                set.insert((x as i32, y as i32));
                continue;
            }

            // add left to right
            if *col > row_max {
                if transposed {
                    set.insert((y as i32, x as i32));
                } else {
                    set.insert((x as i32, y as i32));
                }
                row_max = *col;
                continue;
            }
        }
    }

    // add right to left
    for (y, row) in grid.iter().enumerate().rev() {
        let mut row_max: i32 = row[row.len() - 1];
        for (x, col) in row.iter().enumerate().rev() {
            if *col > row_max {
                if transposed {
                    set.insert((y as i32, x as i32));
                } else {
                    set.insert((x as i32, y as i32));
                }
                row_max = *col;
            }
        }
    }
}

fn transpose(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut gprime: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    for (y, _row) in grid.iter().enumerate() {
        for (x, _col) in grid[0].iter().enumerate() {
            gprime[y][x] = grid[x][y]
        }
    }

    gprime
}

const DELTAS: &[(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];

fn get_scenic_scores(
    grid: &Vec<Vec<i32>>,
    visible: &HashSet<(i32, i32)>,
) -> HashMap<(i32, i32), i32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for &(x, y) in visible {
        let mut score: i32 = 1;
        let tree = grid[y as usize][x as usize];

        for &(dx, dy) in DELTAS {
            let mut tree_score: i32 = 0;
            let mut ndx = x + dx;
            let mut ndy = y + dy;

            while (ndx >= 0 && ndx < grid.len() as i32)
                && (ndy >= 0 && ndy < grid[y as usize].len() as i32)
            {
                let ntree = grid[ndy as usize][ndx as usize];
                tree_score += 1;
                if ntree >= tree {
                    break;
                }

                ndx += dx;
                ndy += dy;
            }
            score *= tree_score;
        }
        map.insert((x, y), score);
    }

    map
}
