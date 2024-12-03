#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

fn main() {
    let input = include_str!("../../../inputs/day-2/example.txt");

    let safe_reports = input.lines().filter(|r| is_safe_report(r));

    println!("{}", safe_reports.count());
}

fn is_safe_report(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split_whitespace()
        .map(|c| c.parse::<i32>().expect("a number"))
        .collect();

    let delta = levels[1] - levels[0];

    if delta.abs() > 3 || delta == 0 {
        return false;
    }

    let direction = if delta > 0 {
        Direction::Increasing
    } else {
        Direction::Decreasing
    };

    levels.windows(2).all(|pair| {
        let d = pair[1] - pair[0];
        if d.abs() > 3 || d == 0 {
            return false;
        }
        match direction {
            Direction::Increasing => d > 0,
            Direction::Decreasing => d < 0,
        }
    })
}
