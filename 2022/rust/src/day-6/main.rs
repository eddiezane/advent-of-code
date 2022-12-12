use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../inputs/day-6/input.txt").trim();
    let iter = input.chars().collect::<Vec<char>>();
    let windows = iter.windows(14);

    for (i, w) in windows.enumerate() {
        let s: HashSet<&char> = HashSet::from_iter(w);
        if s.len() == 14 {
            println!("{} {:?} {:?}", i + 14, w, s);
            break;
        }
    }
}
