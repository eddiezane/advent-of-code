use std::{collections::HashSet, fmt::Display};

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

type Knot = (i32, i32);

struct Rope {
    knots: Vec<Knot>,
    tail_visits: HashSet<Knot>,
}

impl Rope {
    fn new() -> Rope {
        let mut rope = Rope {
            knots: vec![(0, 0); 10],
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

        self.knots[0].0 += delta.0;
        self.knots[0].1 += delta.1;

        for head in 0..self.knots.len() - 1 {
            let tail = head + 1;

            if need_move_tail(&self.knots[head], &self.knots[tail]) {
                let mut x = self.knots[head].0 - self.knots[tail].0;
                let mut y = self.knots[head].1 - self.knots[tail].1;

                if x != 0 {
                    x /= x.abs();
                }

                if y != 0 {
                    y /= y.abs();
                }

                self.knots[tail].0 += x;
                self.knots[tail].1 += y;

                if head == 8 {
                    self.tail_visits.insert(self.knots[tail]);
                }
            }
        }
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.knots)
    }
}

fn need_move_tail(head: &Knot, tail: &Knot) -> bool {
    let dist = (((head.0 - tail.0) as f64).powf(2.0) + ((head.1 - tail.1) as f64).powf(2.0)).sqrt();
    dist > 2f64.sqrt()
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
