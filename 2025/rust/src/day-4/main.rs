type Grid = Vec<Vec<char>>;

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

    let mut grid: Grid = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut accessible = 0_i32;

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut to_update: Vec<(usize, usize)> = Vec::new();

    loop {
        to_update.clear();
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != '@' {
                    continue;
                }

                let adj = DELTAS
                    .iter()
                    .filter(|&&(dx, dy)| {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < width && ny >= 0 && ny < height {
                            grid[ny as usize][nx as usize] == '@'
                        } else {
                            false
                        }
                    })
                    .count();

                if adj < 4 {
                    to_update.push((x, y));
                }
            }
        }

        if to_update.is_empty() {
            break;
        }

        for (x, y) in &to_update {
            accessible += 1;
            grid[*y][*x] = 'X';
        }
    }
    println!("{accessible}");
}
