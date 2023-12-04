#[derive(Debug)]
struct Game {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let input = include_str!("../../../inputs/day-2/input.txt");

    let mut answer = 0;

    for (idx, line) in input.lines().enumerate() {
        let game = parse_game(line);
        let power = game.r * game.g * game.b;
        answer += power;
    }

    println!("{answer}");
}

fn parse_game(l: &str) -> Game {
    let mut game = Game { r: 0, g: 0, b: 0 };
    let pulls = l.split(':').skip(1).next().expect("a round").split(';');
    for pull in pulls {
        for cube in pull.split(',') {
            let mut parts = cube.trim().split(' ');
            let count = parts
                .next()
                .expect("number to start")
                .parse::<u32>()
                .expect("a number");
            let color = parts.next().expect("color to follow");
            match color {
                "red" => {
                    if game.r < count {
                        game.r = count;
                    }
                }
                "green" => {
                    if game.g < count {
                        game.g = count;
                    }
                }
                "blue" => {
                    if game.b < count {
                        game.b = count;
                    }
                }
                _ => {
                    panic!("unexpected color");
                }
            };
        }
    }
    game
}
