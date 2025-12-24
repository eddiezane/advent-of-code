use advent_of_code::util::grid::Grid;
fn main() {
    let input = include_str!("../../../inputs/day-7/input.txt");

    let mut grid: Grid<char> = input.lines().map(|c| c.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut splits = 0;

    for y in 0..height {
        for x in 0..width {
            let char = grid[y][x];
            match char {
                'S' | '|' => continue,
                '^' => {
                    if grid[y - 1][x] == '|' {
                        if grid[y][x - 1] == '.' {
                            grid[y][x - 1] = '|';
                        }
                        if grid[y][x + 1] == '.' {
                            grid[y][x + 1] = '|';
                            splits += 1;
                        }
                    }
                }
                '.' => {
                    if y > 0 {
                        let above = grid[y - 1][x];
                        if above == '|' || above == 'S' {
                            grid[y][x] = '|';
                        }
                    }
                }
                _ => panic!("wtf is this: {char}"),
            }
        }
    }

    println!("{splits}");
}
