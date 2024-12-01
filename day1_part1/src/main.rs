use common::read_lines;

type RawInput = Vec<(u64, u64)>;

fn run(input: &RawInput) -> u64 {
    // Transpose the input

    let mut first = Vec::with_capacity(input.len());
    let mut second = Vec::with_capacity(input.len());

    for (i1, i2) in input {
        first.push(*i1);
        second.push(*i2);
    }

    first.sort();
    second.sort();

    let mut sum = 0;

    for (s1, s2) in first.into_iter().zip(second.into_iter()) {
        let dist = s2.abs_diff(s1);

        sum += dist
    }

    sum
}

fn main() {
    let lines = read_lines("day1_part1/src/data/input1");
    let parsed: RawInput = lines
        .into_iter()
        .map(|line| {
            let parsed_line = line
                .split_whitespace()
                .map(|splitted| splitted.parse::<u64>().expect("invalid u64"))
                .collect::<Vec<u64>>();

            (
                *parsed_line.get(0).expect("missing input"),
                *parsed_line.get(1).expect("missing input"),
            )
        })
        .collect();

    println!("Read {} lines", parsed.len());

    let result = run(&parsed);

    println!("Result is {}", result);
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_example() {
        use crate::run;

        let raw_input = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        let result = run(&raw_input);

        assert_eq!(11, result);
    }
}
