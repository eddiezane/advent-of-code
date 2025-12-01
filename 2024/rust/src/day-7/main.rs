const OPERATIONS: &[&str] = &["+", "*"];

fn main() {
    let input = include_str!("../../../inputs/day-7/example.txt");
    let equations: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            let (sol_s, rest): (&str, &str) = l.split_once(":").unwrap();
            let answer = sol_s.parse::<i32>().unwrap();
            let nums: Vec<i32> = rest
                .split(" ")
                .filter_map(|n| match n.parse::<i32>() {
                    Ok(i) => Some(i),
                    Err(_e) => None,
                })
                .collect();
            std::iter::once(answer).chain(nums).collect()
        })
        .collect();

    dbg!(&equations);

    for eq in equations {
        let (answer, nums) = eq.split_first().unwrap();
        dbg!(answer, nums);
        is_valid_equation(answer, nums);
    }
}

fn is_valid_equation(answer: &i32, nums: &[i32]) -> bool {
    let spots = nums.len() - 1;
    let combs = generate_combinations(spots);
    dbg!(combs);
    true
}

fn generate_combinations<'a>(spots: usize) -> Vec<Vec<&'a str>> {
    let mut result = vec![vec![]];

    for row in result {
        for &op in OPERATIONS {}
    }

    result
}

// fn generate_combinations<'a>(spots: usize) -> Vec<Vec<&'a str>> {
//     if spots == 0 {
//         return vec![vec![]];
//     }
//
//     let mut result = Vec::new();
//
//     for &op in OPERATIONS {
//         let sub_combs = generate_combinations(spots - 1);
//         for mut sub in sub_combs {
//             sub.push(op);
//             result.push(sub);
//         }
//     }
//
//     result
// }
