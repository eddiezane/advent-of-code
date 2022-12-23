use std::collections::HashSet;

#[derive(Debug)]
struct Move {
    direction: Direction,
    number: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    tail_visits: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        let mut rope = Rope {
            head: (0, 0),
            tail: (0, 0),
            tail_visits: HashSet::new(),
        };

        rope.tail_visits.insert((0, 0));

        rope
    }

    fn apply(&mut self, m: &Move) {
        let delta = match m.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let old_x = self.head.0;
        let old_y = self.head.1;

        self.head.0 += delta.0;
        self.head.1 += delta.1;

        if self.need_move_tail() {
            self.tail.0 = old_x;
            self.tail.1 = old_y;
            self.tail_visits.insert((self.tail.0, self.tail.1));
        }
    }

    fn need_move_tail(&self) -> bool {
        let dx = self.head.0.abs_diff(self.tail.0);
        let dy = self.head.1.abs_diff(self.tail.1);
        dx > 1 || dy > 1
    }
}

fn main() {
    let input = include_str!("../../../inputs/day-9/input.txt");
    let moves: Vec<Move> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let direction: Direction = match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unexpected direction"),
            };

            Move {
                direction,
                number: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let mut rope = Rope::new();

    for m in moves {
        for _ in 0..m.number {
            rope.apply(&m);
        }
    }

    println!("{}", rope.tail_visits.len());
}
