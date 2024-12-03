use common::read_lines;

type RawInput = Vec<Vec<u64>>;

fn run(input: &RawInput) -> u64 {
    let mut sum = 0;

    for report in input {
        let all_increasing = report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(n1, n2)| *n1 < *n2);

        let all_decreasing = report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(n1, n2)| *n1 > *n2);

        if !(all_increasing || all_decreasing) {
            continue;
        }

        if report.iter().zip(report.iter().skip(1)).all(|(n1, n2)| {
            let d = (*n1).abs_diff(*n2);

            d >= 1 && d <= 3
        }) {
            sum += 1;
        }
    }

    sum
}

fn main() {
    let lines = read_lines("day2_part1/src/data/input_day2");
    let parsed: RawInput = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|splitted| splitted.parse::<u64>().expect("invalid u64"))
                .collect::<Vec<u64>>()
        })
        .collect();

    println!("Read {} lines", parsed.len());

    let result = run(&parsed);

    println!("Result is {}", result);
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_example_day2_part1() {
        use crate::run;

        let raw_input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let result = run(&raw_input);

        assert_eq!(2, result);
    }
}
