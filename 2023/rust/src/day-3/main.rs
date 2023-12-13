fn main() {
    let input = include_str!("../../../inputs/day-3/example.txt");

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut row: Vec<char> = vec![];

        for c in line.chars() {
            row.push(c);
        }

        grid.push(row);
    }

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 1), (1, 0), (0, -1), (-1, 0)];
}
