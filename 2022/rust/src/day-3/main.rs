use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::{HashSet, HashMap},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../inputs/day-3/example.txt")?;
    let reader = BufReader::new(file);

    let lower: HashMap<char, usize> = ('a'..='z').enumerate().map(|(i, c)| (c, i+1)).collect();
    let upper: HashMap<char, usize> = ('A'..='Z').enumerate().map(|(i, c)| (c, i+27)).collect();
    let scores: HashMap<char, usize> = lower.into_iter().chain(upper).collect();

    let mut total = 0;
    for line in reader.lines() {
        let l = line?;
        let mid = l.len();
        let (a, b) = l.split_at(mid/2);
        let first: HashSet<char> = HashSet::from_iter(a.chars());
        let second: HashSet<char> = HashSet::from_iter(b.chars());
        let intersection: Vec<&char> = first.intersection(&second).collect();
        total += scores[intersection.first().unwrap()];
    }

    println!("{total}");

    Ok(())
}
