use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../../inputs/day-5/input.txt");

    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let db: Vec<RangeInclusive<i64>> = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let available: Vec<i64> = ids.lines().map(|line| line.parse().unwrap()).collect();

    let len = available
        .iter()
        .filter(|&item| db.iter().any(|range| range.contains(item)))
        .count();

    println!("{len}");
}
