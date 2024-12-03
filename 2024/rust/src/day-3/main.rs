use regex::Regex;

fn main() {
    let input = include_str!("../../../inputs/day-3/input.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res: i32 = input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line).map(|caps| {
                let (_, [x, y]) = caps.extract();
                x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
            })
        })
        .sum();
    println!("{}", res)
}
