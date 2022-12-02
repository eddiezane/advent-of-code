use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../inputs/day-1/input.txt")?;
    let reader = BufReader::new(file);

    let mut list: Vec<i32> = vec![0];
    let mut elf = 0;
    for l in reader.lines() {
        let line = l?;
        if line.is_empty() {
            elf += 1;
            list.insert(elf, 0);
            continue;
        }
        let n: i32 = line.parse()?;
        list[elf] += n;
    }

    list.sort();
    println!("{}", &list.as_slice()[list.len() - 3..].iter().sum::<i32>());

    Ok(())
}
