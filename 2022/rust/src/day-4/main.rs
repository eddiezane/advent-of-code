#[allow(clippy::get_first)]

fn main() {
    let input = include_str!("../../../inputs/day-4/input.txt");
    let lines = input.lines();

    let mut overlaps = 0;
    for l in lines {
        let elves: Vec<(i32, i32)> = l
            .split(',')
            .map(|elf| {
                let assignments: Vec<&str> = elf.split('-').collect();
                let first = assignments.get(0).unwrap().parse::<i32>().unwrap();
                let last = assignments.get(1).unwrap().parse::<i32>().unwrap();
                (first, last)
            })
            .collect();
        let (x1, x2) = elves.get(0).unwrap();
        let (y1, y2) = elves.get(1).unwrap();

        // if (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2) {
        //     overlaps += 1;
        // }

        if (x1 <= y1 && x2 >= y1) || (y1 <= x1 && y2 >= x1) {
            overlaps += 1;
        }
    }
    println!("{overlaps}");
}
