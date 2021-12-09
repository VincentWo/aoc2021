use std::collections::HashSet;

fn calculate_neighbours(
    (x, y): (usize, usize),
    (x_len, y_len): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if x < x_len - 1 {
        neighbours.push((x + 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < y_len - 1 {
        neighbours.push((x, y + 1));
    }

    neighbours
}

fn calculate_basin_points(
    values: &[Vec<usize>],
    start_point @ (start_x, start_y): (usize, usize),
) -> HashSet<(usize, usize)> {
    calculate_neighbours(start_point, (values.len(), values[0].len()))
        .into_iter()
        .filter_map(|(x, y)| {
            if values[x][y] > values[start_x.clone()][start_y.clone()] && values[x][y] != 9 {
                Some(calculate_basin_points(values, (x, y)))
            } else {
                None
            }
        })
        .flatten()
        .chain(std::iter::once(start_point))
        .collect()
}

fn calculate_basin(values: &[Vec<usize>], start_point: (usize, usize)) -> usize {
    calculate_basin_points(values, start_point).len()
}
fn main() {
    let input = include_str!("real_input")
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut low_points = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let neighbours = calculate_neighbours((i, j), (input.len(), input[i].len()));

            let low_point = neighbours
                .iter()
                .map(|&(x, y)| &input[x][y])
                .all(|&v| v > input[i][j]);
            if low_point {
                low_points.push((i, j));
            }
        }
    }

    let mut basins = low_points
        .into_iter()
        .map(|low_point| calculate_basin(&input, low_point))
        .collect::<Vec<_>>();
    basins.sort_by_key(|b| std::cmp::Reverse(*b));

    println!("{}", basins.into_iter().take(3).product::<usize>());
}
