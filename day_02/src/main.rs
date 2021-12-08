use std::{convert::Infallible, error::Error, io::BufRead, iter::Sum, ops::Add, str::FromStr};

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Velocity {
    horizontal: i32,
    aim: i32,
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl Velocity {
    fn new() -> Self {
        Velocity {
            horizontal: 0,
            aim: 0,
        }
    }
}

impl Position {
    fn new() -> Self {
        Position {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }
}

impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, velocity: Velocity) -> Self::Output {
        Position {
            horizontal: self.horizontal + velocity.horizontal,
            vertical: self.vertical + self.aim * velocity.horizontal,
            aim: self.aim + velocity.aim,
        }
    }
}

#[derive(Debug)]
struct UnknownDirectionError(String);

impl FromStr for Direction {
    type Err = UnknownDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            unknown => Err(UnknownDirectionError(String::from(unknown)))?,
        })
    }
}

impl Sum<Velocity> for Position {
    fn sum<I: Iterator<Item = Velocity>>(iter: I) -> Position {
        iter.fold(Position::new(), |a, b| a + b)
    }
}

impl FromStr for Velocity {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(' ');
        let direction = components.next().unwrap().parse::<Direction>().unwrap();
        let amount = components.next().unwrap().parse::<i32>().unwrap();

        Ok(match direction {
            Direction::Forward => Velocity {
                horizontal: amount,
                aim: 0,
            },
            Direction::Down => Velocity {
                horizontal: 0,
                aim: amount,
            },
            Direction::Up => Velocity {
                horizontal: 0,
                aim: -amount,
            },
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let input = std::io::Cursor::new(input);

    let Position {
        horizontal,
        vertical,
        ..
    } = input
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|s| s.parse::<Velocity>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<Position>();

    println!("{}", horizontal * vertical);
    Ok(())
}
