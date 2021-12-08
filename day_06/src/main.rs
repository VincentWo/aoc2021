use std::collections::HashMap;

fn main() {
    let first_generation = include_str!("input")
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .fold(HashMap::new(), |mut counts, fish_days_left| {
            let fish_count = counts.entry(fish_days_left).or_insert(0 as u64);
            *fish_count += 1;
            counts
        });

    let mut current_generation = first_generation;

    for _ in 0..256 {
        current_generation = current_generation
            .into_iter()
            .map(|(days_left, count)| {
                if days_left == 0 {
                    vec![(6, count), (8, count)]
                } else {
                    vec![(days_left - 1, count)]
                }
            })
            .flatten()
            .fold(HashMap::new(), |mut counts, (days_left, new_fish_count)| {
                let old_fish_count = counts.entry(days_left).or_insert(0);
                *old_fish_count += new_fish_count;
                counts
            });
    }

    println!(
        "{:#?}",
        current_generation
            .into_iter()
            .map(|(_, fish_count)| fish_count)
            .sum::<u64>()
    );
}
