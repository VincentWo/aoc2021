#![feature(drain_filter)]

#[derive(Debug)]
struct BingoNumber {
    number: u32,
    is_marked: bool,
}

impl BingoNumber {
    fn new(number: u32) -> Self {
        BingoNumber {
            number,
            is_marked: false,
        }
    }
}

impl From<u32> for BingoNumber {
    fn from(number: u32) -> Self {
        BingoNumber::new(number)
    }
}

#[derive(Debug)]
struct Board {
    values: Vec<BingoNumber>,
    column_length: usize,
    is_solved: bool,
}

impl Board {
    fn mark_number(&mut self, to_mark: u32) {
        // Mark the numbers and only return the changed indizes of the numbers
        let changed_indizes = self
            .values
            .iter_mut()
            .enumerate()
            .filter_map(|(i, mut bingo_number)| {
                if bingo_number.number == to_mark && !bingo_number.is_marked {
                    bingo_number.is_marked = true;
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for index in changed_indizes {
            let row = index / self.column_length;
            let col = index % self.column_length;

            let row_length = self.values.len() / self.column_length;

            if self.is_solved {
                break;
            }

            self.is_solved = (0..row_length)
                .map(|i| &self.values[row * self.column_length + i])
                .all(|bingo_number| bingo_number.is_marked);

            if self.is_solved {
                break;
            }

            self.is_solved = (0..self.column_length)
                .map(|i| &self.values[i * row_length + col])
                .all(|bingo_number| bingo_number.is_marked);
        }
    }
}

impl<I> FromIterator<I> for Board
where
    I: Iterator<Item = u32>,
{
    fn from_iter<T>(outer: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        let mut values = Vec::<BingoNumber>::new();
        let mut column_length = None;

        for column in outer {
            let old_len = values.len();
            values.extend(column.map(|n| n.into()));
            let current_col_len = values.len() - old_len;
            match column_length {
                None => column_length = Some(current_col_len),
                Some(column_length) if column_length == current_col_len => {
                    /* Do nothing, everything is fine */
                }
                Some(column_length) => panic!(
                    "Mismatched column lengths: {} != {}",
                    column_length, current_col_len
                ),
            }
        }
        Board {
            values,
            column_length: column_length.unwrap_or(0),
            is_solved: false,
        }
    }
}

fn main() {
    let input = include_str!("real_input");

    let parts = input.split("\n\n").collect::<Vec<_>>();

    let numbers = parts[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parts[1..]
        .into_iter()
        .map(|board| {
            board
                .split_terminator('\n')
                .map(|col| col.split_whitespace().map(|s| s.parse::<u32>().unwrap()))
                .collect::<Board>()
        })
        .collect::<Vec<Board>>();

    let mut last_number = 0;

    let mut solved = Vec::new();
    for number in numbers {
        last_number = number;
        for board in &mut boards {
            board.mark_number(number);
        }
        solved.extend(boards.drain_filter(|board| board.is_solved));

        if boards.is_empty() {
            break;
        }
    }
    let last_solved = &solved[solved.len() - 1];

    let unmarked_sum: u32 = last_solved
        .values
        .iter()
        .filter_map(|bn| if !bn.is_marked { Some(bn.number) } else { None })
        .sum();

    println!("{:#?}", unmarked_sum * last_number);
}
