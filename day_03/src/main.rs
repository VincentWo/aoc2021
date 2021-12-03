use std::{
    io::{BufRead, Cursor},
    iter::Sum,
    ops::Add,
};

#[derive(Debug)]
struct ColumnResult {
    one_count: u32,
    zero_count: u32,
}

impl ColumnResult {
    fn new() -> Self {
        Default::default()
    }
}

impl Add for ColumnResult {
    type Output = ColumnResult;
    fn add(self, rhs: Self) -> Self {
        ColumnResult {
            one_count: self.one_count + rhs.one_count,
            zero_count: self.zero_count + rhs.zero_count,
        }
    }
}

impl Default for ColumnResult {
    fn default() -> Self {
        ColumnResult {
            one_count: 0,
            zero_count: 0,
        }
    }
}

impl Sum for ColumnResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ColumnResult::new(), |a, b| a + b)
    }
}

fn result_for_column<'a, I: Iterator<Item = &'a String>>(
    position: usize,
    lines: I,
) -> ColumnResult {
    lines
        .into_iter()
        .map(|row| {
            let byte = row.as_bytes()[position];
            match byte {
                b'1' => ColumnResult {
                    one_count: 1,
                    zero_count: 0,
                },
                b'0' => ColumnResult {
                    one_count: 0,
                    zero_count: 1,
                },
                unsupported_digit => panic!("Unsupported digit: {}", unsupported_digit),
            }
        })
        .sum()
}

fn filter_for_byte(mut input: Vec<String>, get_filter_byte: impl Fn(ColumnResult) -> u8) -> u32 {
    for i in 0.. {
        let column_result = result_for_column(i, input.iter());

        let filter_byte = get_filter_byte(column_result);

        input = input
            .into_iter()
            .filter(|s| s.as_bytes()[i] == filter_byte)
            .collect();
        if input.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&input[0], 2).unwrap()
}

fn main() {
    let input = Cursor::new(include_str!("input")).lines();

    let input = input.map(|line| line.unwrap()).collect::<Vec<_>>();

    let oxygen_rating = filter_for_byte(input.clone(), |column_result| {
        if column_result.zero_count > column_result.one_count {
            b'0'
        } else {
            b'1'
        }
    });

    let co2_scrubber_rating = filter_for_byte(input.clone(), |column_result| {
        if column_result.one_count < column_result.zero_count {
            b'1'
        } else {
            b'0'
        }
    });

    println!("{}", oxygen_rating * co2_scrubber_rating);
}
