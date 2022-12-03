use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../inputs/day-2/input.txt")?;
    let reader = BufReader::new(file);

    // A X 1 rock
    // B Y 2 paper
    // C Z 3 Scissors
    //
    // 0 lost
    // 3 draw
    // 6 won
    //
    // A X = (1 + 3) = 4
    // A Y = (2 + 6) = 8
    // A Z = (3 + 0) = 3
    //
    // B X = (1 + 0) = 1
    // B Y = (2 + 3) = 5
    // B Z = (3 + 6) = 9
    //
    // C X = (1 + 6) = 7
    // C Y = (2 + 0) = 2
    // C Z = (3 + 3) = 6

    // part 1
    // let score_map = HashMap::from([
        // ("A X", 4),
        // ("A Y", 8),
        // ("A Z", 3),
        // ("B X", 1),
        // ("B Y", 5),
        // ("B Z", 9),
        // ("C X", 7),
        // ("C Y", 2),
        // ("C Z", 6),
    // ]);

    let score_map = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    let mut score = 0;

    for (i, l) in reader.lines().enumerate() {
        let game = l?;
        let s = score_map[game.as_str()];
        score += s;
        println!("i: {i} game: {game} s: {s} score: {score}");
    }

    println!("{score}");

    Ok(())
}
