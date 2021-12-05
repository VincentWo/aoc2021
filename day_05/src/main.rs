use std::{collections::HashMap, convert::Infallible, str::FromStr, io::{BufRead, Cursor}};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}
impl FromStr for Coordinate {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i32> = s.split(',').map(|c| c.parse().unwrap()).collect();
        Ok(Coordinate {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn included_coordinates(&self) -> CoordinateIterator {
        CoordinateIterator::new(self.start.clone(), self.end.clone())
    }
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<Coordinate> = s.split(" -> ").map(|c| c.parse().unwrap()).collect();
        Ok(Line{
            start: coordinates[0].clone(),
            end: coordinates[1].clone(),
        })
    }

}

#[derive(Debug)]
struct CoordinateIterator {
    start: Coordinate,
    end: Coordinate,
    current: Coordinate,
}

impl CoordinateIterator {
    fn new(start: Coordinate, end: Coordinate) -> Self {
        let current = start.clone();
        CoordinateIterator {
            start,
            end,
            current,
        }
    }
}

impl Iterator for CoordinateIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let value = self.current.clone();

        if self.start.x < self.end.x {
            if self.current.x <= self.end.x {
                self.current.x += 1;
            } else {
                return None;
            }
        }
        if self.start.x > self.end.x {
            if self.current.x >= self.end.x {
                self.current.x -= 1;
            } else {
                return None;
            }
        }
        if self.start.y < self.end.y {
            if self.current.y <= self.end.y {
                self.current.y += 1;
            } else {
                return None;
            }
        }
        if self.start.y > self.end.y {
            if self.current.y >= self.end.y {
                self.current.y -= 1;
            } else {
                return None;
            }
        }
        if self.start == self.end {
            // If start == end, we have a one point line so we just check whether start and current
            // are the same
            if self.start == self.current {
                self.current.x += 1;
                self.current.y += 1;
            } else {
                return None;
            }
        }

        Some(value)
    }
}

fn main() {
    let input = Cursor::new(include_str!("real_input")).lines().collect::<Result<Vec<_>, _>>().unwrap();
    println!("{:#?}",
        input
        .iter()
        .map(|l| l.parse::<Line>().unwrap())
        .map(|l| l.included_coordinates())
        .flatten()
        .fold(
            HashMap::<Coordinate, u32>::new(),
            |mut counts, coordinate| {
                let count = counts.entry(coordinate).or_insert(0);
                *count += 1;

                counts
            })
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .count());
}
