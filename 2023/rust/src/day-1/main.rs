fn main() {
    let input = include_str!("../../../inputs/day-1/input.txt");
    let sum: u32 = input
        .lines()
        .map(|l| {
            let mut it = l.split("").filter_map(|c| c.parse::<u32>().ok());

            let first = it.next().unwrap();
            let second = it.last().unwrap_or(first);

            format!("{first}{second}").parse::<u32>().ok().unwrap()
        })
        .sum();

    println!("{}", sum)
}
