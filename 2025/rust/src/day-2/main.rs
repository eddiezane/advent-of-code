fn main() {
    let input = include_str!("../../../inputs/day-2/input.txt").trim();

    let ranges = input.split(",");

    let mut sum: i64 = 0;

    for range in ranges {
        let mut r = range.trim().split("-");
        let start = r.next().unwrap().parse::<i64>().unwrap();
        let end = r.next().unwrap().parse::<i64>().unwrap();

        (start..=end).for_each(|n| {
            let sn = n.to_string();
            if sn.len() % 2 == 0 {
                let (first, second) = sn.split_at(sn.len() / 2);
                if first == second {
                    sum += n;
                }
            }
        });
    }

    println!("{sum}");
}
