use regex::Regex;

fn main() {
    // clean new lines
    let input: String = include_str!("../../../inputs/day-3/input.txt")
        .lines()
        .collect();

    // capture groups for mul(12,23)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // delete all don't ungreedy
    let redo = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    // delete last don't to end ungreedy
    let reend = Regex::new(r"don't\(\).*?$").unwrap();

    let t = redo.replace_all(&input, "");
    let text = reend.replace_all(&t, "");

    let answer: i32 = re
        .captures_iter(&text)
        .map(|caps| {
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            x * y
        })
        .sum();

    println!("{}", answer);
}
