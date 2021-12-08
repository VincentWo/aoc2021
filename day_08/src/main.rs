use std::collections::HashMap;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b   git
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 *
 *
 *
 *
 */
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct UnknownDigit {
    segments: Vec<Segment>,
}

impl UnknownDigit {
    fn from_segments(mut segments: Vec<Segment>) -> Self {
        segments.sort();

        UnknownDigit { segments }
    }
}

#[derive(Debug)]
struct UnknownSegmentError;

impl TryFrom<char> for Segment {
    type Error = UnknownSegmentError;
    fn try_from(c: char) -> Result<Segment, UnknownSegmentError> {
        Ok(match c {
            'a' | 'A' => Segment::A,
            'b' | 'B' => Segment::B,
            'c' | 'C' => Segment::C,
            'd' | 'D' => Segment::D,
            'e' | 'E' => Segment::E,
            'f' | 'F' => Segment::F,
            'g' | 'G' => Segment::G,
            _unknown => return Err(UnknownSegmentError),
        })
    }
}
#[derive(Debug)]
struct IncompleteSegmentConfig(HashMap<Segment, Vec<Segment>>);

#[derive(Debug)]
struct SegmentConfig(HashMap<Segment, Segment>);

#[derive(Debug)]
struct SegmentInputBuilder {
    one: Option<[Segment; 2]>,
    four: Option<[Segment; 4]>,
    seven: Option<[Segment; 3]>,
    // We don't have to save eight cause it doesn't give us any infos
    // Save all the numbers having 6 segments: 0, 6, 9
    six_segment_numbers: Vec<[Segment; 6]>,
    // Save all the numbers having 5 segments: 2, 3, 5,
    five_segment_numbers: Vec<[Segment; 5]>,
}

#[derive(Debug)]
struct SegmentInput {
    one: [Segment; 2],
    four: [Segment; 4],
    // Save all the numbers having 5 segments: 2, 3, 5,
    five_segment_numbers: [[Segment; 5]; 3],
}

impl From<SegmentInputBuilder> for SegmentInput {
    fn from(builder: SegmentInputBuilder) -> SegmentInput {
        SegmentInput {
            one: builder.one.unwrap(),
            four: builder.four.unwrap(),
            five_segment_numbers: builder.five_segment_numbers.try_into().unwrap(),
        }
    }
}

impl SegmentInputBuilder {
    fn new() -> Self {
        SegmentInputBuilder {
            one: None,
            seven: None,
            four: None,
            five_segment_numbers: Vec::new(),
            six_segment_numbers: Vec::new(),
        }
    }
    fn include_digit(&mut self, digit: UnknownDigit) {
        let segments = &digit.segments;

        match segments.len() {
            2 => self.one = Some(digit.segments.try_into().unwrap()),
            3 => self.seven = Some(digit.segments.try_into().unwrap()),
            4 => self.four = Some(digit.segments.try_into().unwrap()),
            5 => self
                .five_segment_numbers
                .push(digit.segments.try_into().unwrap()),
            6 => self
                .six_segment_numbers
                .push(digit.segments.try_into().unwrap()),
            7 => { /* The eight does not give us any infos*/ }
            _ => unimplemented!("This should not happen: {:#?}", digit),
        }
    }
}

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|line| {
            line.split('|')
                .map(|s| {
                    s.trim()
                        .split(' ')
                        .map(|number| {
                            number
                                .chars()
                                .map(|d| Segment::try_from(d).unwrap())
                                .collect::<Vec<_>>()
                        })
                        .map(UnknownDigit::from_segments)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let input: u32 = input
        .into_iter()
        .map(|mut data| {
            let output = data.pop().unwrap();
            let input = data.pop().unwrap();
            let classifier = Classifier::train(
                input
                    .into_iter()
                    .fold(SegmentInputBuilder::new(), |mut config, digit| {
                        config.include_digit(digit);
                        config
                    })
                    .try_into()
                    .unwrap(),
            );
            output
                .into_iter()
                .map(|output| classifier.classify(&output.segments).unwrap().to_char())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("{:#?}", input);
}

#[derive(Debug, PartialEq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn to_char(&self) -> char {
        match self {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        }
    }
}
#[derive(Debug)]
struct Classifier {
    one: [Segment; 2],
    four: [Segment; 4],
    three: Option<[Segment; 5]>,
}

impl Classifier {
    fn train(input: SegmentInput) -> Self {
        let mut incomplete_classifier = Self {
            one: input.one,
            four: input.four,
            three: None,
        };
        let three = input
            .five_segment_numbers
            .into_iter()
            .filter(|unknown| {
                let digit = incomplete_classifier.classify(unknown).unwrap();
                digit == Digit::Three
            })
            .next()
            .unwrap();
        incomplete_classifier.three = Some(three);

        incomplete_classifier
    }

    fn classify(&self, unknown: &[Segment]) -> Option<Digit> {
        Some(match unknown.len() {
            2 => Digit::One,
            3 => Digit::Seven,
            4 => Digit::Four,
            5 => {
                let overlap_with_one = unknown.iter().filter(|s| self.one.contains(s)).count();
                if overlap_with_one == 2 {
                    Digit::Three
                } else {
                    let overlap_with_four =
                        unknown.iter().filter(|s| self.four.contains(s)).count();
                    match overlap_with_four {
                        2 => Digit::Two,
                        3 => Digit::Five,
                        _ => panic!("should not happen"),
                    }
                }
            }
            6 => {
                let overlap_with_one = unknown.iter().filter(|s| self.one.contains(s)).count();
                if overlap_with_one == 2 {
                    let three = self.three.as_ref()?;
                    let overlap_with_three = unknown.iter().filter(|s| three.contains(s)).count();
                    match overlap_with_three {
                        4 => Digit::Zero,
                        5 => Digit::Nine,
                        _ => panic!("should not happen"),
                    }
                } else {
                    Digit::Six
                }
            }
            7 => Digit::Eight,
            _ => panic!("impossible digit: {:#?}", unknown),
        })
    }
}
