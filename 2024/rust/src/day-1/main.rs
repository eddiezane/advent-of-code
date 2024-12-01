use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../inputs/day-1/input.txt");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut numbers = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().expect("a num"));
        left_list.push(numbers.next().expect("a left num"));
        right_list.push(numbers.next().expect("a right num"));
    }

    left_list.sort();
    right_list.sort();

    let freq = right_list.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let score: i32 = left_list
        .iter()
        .map(|i| i * freq.get(i).unwrap_or(&0))
        .sum();

    println!("{}", score);
}
