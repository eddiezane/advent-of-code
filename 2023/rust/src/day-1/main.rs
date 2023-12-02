fn main() {
    let input = include_str!("../../../inputs/day-1/input.txt");
    let sum: u32 = input
        .lines()
        .map(|l| {
            let first: String = process_line(l, false);
            let second: String = process_line(l.chars().rev().collect::<String>().as_str(), true);
            format!("{first}{second}")
                .parse::<u32>()
                .expect("the to have a first and last number")
        })
        .sum();

    println!("{}", sum)
}

fn process_line(line: &str, rev: bool) -> String {
    let mut it = line.chars();
    let mut result = String::from("");
    let mut buf = String::from("");
    loop {
        let c = it.next().expect("the line to have enough characters");
        if c.is_ascii_digit() {
            result = c.to_string();
        } else {
            if rev {
                buf.insert(0, c);
            } else {
                buf.push(c);
            }
            result = match parse_word_to_num(&buf) {
                Some(d) => d,
                None => result,
            }
        }
        if !result.is_empty() {
            return result;
        }
    }
}

fn parse_word_to_num(s: &String) -> Option<String> {
    match s {
        b if b.contains("one") => Some(String::from("1")),
        b if b.contains("two") => Some(String::from("2")),
        b if b.contains("three") => Some(String::from("3")),
        b if b.contains("four") => Some(String::from("4")),
        b if b.contains("five") => Some(String::from("5")),
        b if b.contains("six") => Some(String::from("6")),
        b if b.contains("seven") => Some(String::from("7")),
        b if b.contains("eight") => Some(String::from("8")),
        b if b.contains("nine") => Some(String::from("9")),
        _ => None,
    }
}
