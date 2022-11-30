use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string(filename)?;
    let numbers = input
        .split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut store: HashMap<i32, (i32, i32)> = HashMap::new();

    for (i, n) in numbers.iter().enumerate() {
        for nn in numbers.iter().skip(i + 1) {
            let sum = n + nn;
            store.entry(sum).or_insert((*n, *nn));
        }
    }

    for n in numbers {
        let difference = 2020 - n;

        if store.contains_key(&difference) {
            let (nn, nnn) = store.get(&difference).unwrap();
            let product = n * nn * nnn;
            println!("{n} + {nn} + {nnn} == 2020; {n} * {nn} * {nnn} == {product}");
            break;
        }
    }

    Ok(())
}
