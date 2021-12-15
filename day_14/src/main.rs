#![feature(array_windows)]

use std::collections::HashMap;

fn main() {
    let input: Vec<_> = include_str!("input").split("\n\n").collect();

    let rules: HashMap<(u8, u8), u8> = input[1]
        .trim()
        .split('\n')
        .map(|rule| {
            let pair_one = rule.as_bytes()[0];
            let pair_two = rule.as_bytes()[1];

            let result = rule.as_bytes()[6];

            ((pair_one, pair_two), result)
        })
        .collect();

    let template = input[0];
    let mut pairs =
        template
            .as_bytes()
            .array_windows::<2>()
            .fold(HashMap::new(), |mut counts, pair| {
                let count = counts.entry((pair[0], pair[1])).or_insert(0);
                *count += 1;

                counts
            });

    for _ in 0..40 {
        pairs = pairs
            .into_iter()
            .map(|(pair, count)| {
                if let Some(result) = rules.get(&pair) {
                    vec![((pair.0, *result), count), ((*result, pair.1), count)]
                } else {
                    vec![(pair, count)]
                }
            })
            .flatten()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                let existing_count = counts.entry(pair).or_insert(0u64);
                *existing_count += count;
                counts
            });
    }

    let element_count = pairs
        .into_iter()
        .map(|(pair, count)| [(pair.0, count), (pair.1, count)])
        .flatten()
        .fold(HashMap::new(), |mut counts, (element, count)| {
            let existing_count = counts.entry(element).or_insert(0);
            *existing_count += count;
            counts
        })
        .into_iter()
        .map(|(element, count)| (element as char, count / 2 + count % 2))
        .collect::<HashMap<_, _>>();

    let min_occurences = element_count
        .iter()
        .min_by_key(|(_, count)| *count)
        .unwrap()
        .1;
    let max_occurences = element_count
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .1;

    println!("{:#?}", max_occurences - min_occurences);
}
