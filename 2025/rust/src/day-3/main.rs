fn main() {
    let input = include_str!("../../../inputs/day-3/input.txt");

    let mut total_output: i32 = 0;

    for line in input.lines() {
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        let mut first: i32 = 0;
        let mut first_i: usize = 0;
        let mut second: i32 = 0;

        for (i, bat) in bank.iter().enumerate() {
            if *bat > first && i < bank.len() - 1 {
                first = *bat;
                first_i = i;
            }
        }

        for bat in bank.iter().skip(first_i + 1) {
            if *bat > second {
                second = *bat;
            }
        }

        total_output += format!("{first}{second}").parse::<i32>().unwrap();
    }

    println!("{total_output}");
}
