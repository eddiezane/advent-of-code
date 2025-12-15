use std::{cmp::max, ops::RangeInclusive};

fn main() {
    let input = include_str!("../../../inputs/day-5/input.txt");

    let (range_block, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = range_block
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    ranges.sort_by_key(|r| *r.start());

    let merged = ranges
        .into_iter()
        .fold(Vec::<RangeInclusive<u64>>::new(), |mut acc, new| {
            if let Some(last) = acc.last_mut()
                && *new.start() <= *last.end() + 1
            {
                let new_end = max(*last.end(), *new.end());
                *last = *last.start()..=new_end;
                return acc;
            }

            acc.push(new);
            acc
        });

    let count: u64 = merged.into_iter().map(|r| r.end() - r.start() + 1).sum();
    println!("{count}");
}
