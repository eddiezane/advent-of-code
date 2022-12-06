use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("../inputs/day-3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lower: HashMap<char, usize> = ('a'..='z').enumerate().map(|(i, c)| (c, i + 1)).collect();
    let upper: HashMap<char, usize> = ('A'..='Z').enumerate().map(|(i, c)| (c, i + 27)).collect();
    let scores: HashMap<char, usize> = lower.into_iter().chain(upper).collect();

    let mut total = 0;
    let lines: Vec<Result<String, std::io::Error>> = reader.lines().collect();
    assert_eq!(lines.len() % 3, 0);
    for group in lines.chunks(3) {
        let a = group.get(0).unwrap().as_deref().unwrap();
        let b = group.get(1).unwrap().as_deref().unwrap();
        let c = group.get(2).unwrap().as_deref().unwrap();

        let first: HashSet<char> = HashSet::from_iter(a.chars());
        let second: HashSet<char> = HashSet::from_iter(b.chars());
        let third: HashSet<char> = HashSet::from_iter(c.chars());
        let i = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>();
        let intersection = third.intersection(&i).copied().collect::<Vec<_>>();
        total += scores[intersection.first().unwrap()];
    }

    println!("{total}");
}
