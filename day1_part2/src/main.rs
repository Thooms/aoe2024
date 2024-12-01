use std::collections::HashMap;

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

    // Compute the occurences for each number in the second list.

    let mut second_list_occurences: HashMap<u64, u64> = HashMap::with_capacity(input.len()); // worst case all the elements appear only once

    for element in second {
        second_list_occurences
            .entry(element)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut sum = 0;

    for s1 in first {
        sum += s1 * second_list_occurences.get(&s1).map_or(0, |&c| c);
    }

    sum
}

fn main() {
    // Reuse the input from part 1.
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
    pub fn test_example_day1_part2() {
        use crate::run;

        let raw_input = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        let result = run(&raw_input);

        assert_eq!(31, result);
    }
}
