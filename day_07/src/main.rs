#![feature(int_abs_diff)]
fn main() {
    let input = include_str!("input").trim().split(',').map(|s| s.parse().unwrap()).collect::<Vec<u32>>();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    println!("{}",
        (min..max)
            .map(|move_to| input.iter().map(|pos| {
                let diff = pos.abs_diff(move_to);
                (diff.pow(2) + diff) / 2
            }).sum::<u32>())
            .min()
            .unwrap())
}
