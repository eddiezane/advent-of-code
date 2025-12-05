fn main() {
    let input = include_str!("../../../inputs/day-1/input.txt");

    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    for line in input.lines() {
        let (dir, rest) = line.split_at(1);

        let mut delta = rest.parse::<i32>().unwrap();

        if dir == "L" {
            delta = -delta;
        }

        let new_pos = pos + delta;

        let mut wraps = (new_pos / 100).abs();

        if pos != 0 && new_pos <= 0 {
            wraps += 1;
        }

        pos = new_pos.rem_euclid(100);

        count += wraps;
    }

    println!("{count}");
}
