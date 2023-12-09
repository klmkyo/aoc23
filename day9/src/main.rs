fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let data: Vec<Vec<i64>> = file
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            return numbers;
        })
        .collect();

    // this can be optimized to keep only first/last number of each diff layer
    let data_with_diffs: Vec<Vec<Vec<i64>>> = data
        .into_iter()
        .map(|numbers| {
            let mut diffs: Vec<Vec<i64>> = vec![numbers];

            while !diffs.last().unwrap().iter().all(|d| *d == 0) {
                diffs.push(
                    diffs
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|pair| pair[1] - pair[0])
                        .collect(),
                );
            }

            return diffs;
        })
        .collect();

    let part1_predictions: Vec<i64> = data_with_diffs
        .iter()
        .map(|data| {
            let prediction: i64 = data.iter().map(|layer| layer.last().unwrap()).sum();

            return prediction;
        })
        .collect();

    let part2_predictions: Vec<i64> = data_with_diffs
        .iter()
        .map(|data| {
            let prediction: i64 =
                data.iter()
                    .map(|layer| layer.first().unwrap())
                    .rev()
                    .fold(0, |acc, x| x - acc);

            return prediction;
        })
        .collect();

    println!(
        "part 1 solution: {}",
        part1_predictions.iter().sum::<i64>()
    );
    println!(
        "part 2 solution: {}",
        part2_predictions.iter().sum::<i64>()
    );
}
