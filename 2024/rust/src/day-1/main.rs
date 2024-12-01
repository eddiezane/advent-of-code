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

    let sum: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("{}", sum);
}
