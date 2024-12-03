fn main() {
    let input = include_str!("../../../inputs/day-2/input.txt");

    let safe_reports = input.lines().filter(|report| {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|c| c.parse::<i32>().expect("a number"))
            .collect();
        is_safe_report_with_dampener(&levels)
    });

    println!("{}", safe_reports.count());
}

fn is_safe_report(levels: &[i32]) -> bool {
    let delta = levels[1] - levels[0];

    if delta.abs() > 3 || delta == 0 {
        return false;
    }

    let increasing = delta > 0;

    levels.windows(2).all(|pair| {
        let d = pair[1] - pair[0];
        if d.abs() > 3 || d == 0 {
            return false;
        }
        increasing == (d > 0)
    })
}

fn is_safe_report_with_dampener(levels: &Vec<i32>) -> bool {
    if is_safe_report(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut copy = levels.clone();
        copy.remove(i);
        if is_safe_report(&copy) {
            return true;
        }
    }
    return false;
}
