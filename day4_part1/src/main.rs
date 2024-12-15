use common::read_lines;

type RawInput = Vec<Vec<char>>;

fn run(input: &RawInput) -> u64 {
    let i_max = input.len();
    let j_max = input[0].len();

    let mut total = 0;

    for i in 0..i_max {
        for j in 0..j_max {
            total += neighborhood()
                .into_iter()
                .map(|(d_i, d_j)| {
                    search(
                        input,
                        Vec::with_capacity(4),
                        'X',
                        i as i64,
                        j as i64,
                        d_i as i64,
                        d_j as i64,
                        i_max as i64,
                        j_max as i64,
                    )
                })
                .sum::<u64>();
        }
    }

    total
}

fn next_char_in_xmas(c: char) -> Option<char> {
    match c {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        'S' => None,
        _ => None,
    }
}

// For initial search, returns d_i, d_j
fn neighborhood() -> Vec<(i64, i64)> {
    vec![-1i64, 0, 1]
        .iter()
        .flat_map(|d_i| {
            let d_js = vec![-1i64, 0, 1];
            d_js.into_iter().map(|d_j| (*d_i, d_j))
        })
        .filter(|(d_i, d_j)| !(*d_i == 0 && *d_j == 0))
        .collect()
}

// SUCH NEIGHBOURHOOD SEARCH
fn search(
    input: &RawInput,
    current_path: Vec<(i64, i64)>,
    c: char,
    i: i64,
    j: i64,
    // Keep things in the same direction
    d_i: i64,
    d_j: i64,
    // Limits
    i_max: i64,
    j_max: i64,
) -> u64 {
    if !(i >= 0 && i < i_max && j >= 0 && j < j_max) {
        return 0;
    }

    let current_char = input[i as usize][j as usize];

    if current_char != c {
        return 0;
    }

    // Current char matches

    let mut path = current_path.clone();
    path.push((i, j));

    match next_char_in_xmas(c) {
        Some(next_char) => {
            // launch a search in the neighborhood
            return search(
                input,
                path.clone(),
                next_char,
                i + d_i,
                j + d_j,
                d_i,
                d_j,
                i_max,
                j_max,
            );
        }
        None => {
            println!("Found path {:?}", path);
            // we're at the end of the sequence, and we found the end of the word
            return 1;
        }
    }
}

fn main() {
    let lines = read_lines("day4_part1/src/data/input_day4");
    let parsed: RawInput = lines
        .into_iter()
        .map(|l| l.chars().into_iter().collect())
        .collect::<Vec<_>>();

    println!("Read {} lines", parsed.len());

    let result = run(&parsed);

    println!("Result is {}", result);
}

#[cfg(test)]
mod test {
    use crate::neighborhood;

    #[test]
    pub fn test_example_day2_part1() {
        use crate::run;

        let raw_input = vec![
            "MMMSXXMASM".to_owned().chars().into_iter().collect(),
            "MSAMXMSMSA".to_owned().chars().into_iter().collect(),
            "AMXSXMAAMM".to_owned().chars().into_iter().collect(),
            "MSAMASMSMX".to_owned().chars().into_iter().collect(),
            "XMASAMXAMM".to_owned().chars().into_iter().collect(),
            "XXAMMXXAMA".to_owned().chars().into_iter().collect(),
            "SMSMSASXSS".to_owned().chars().into_iter().collect(),
            "SAXAMASAAA".to_owned().chars().into_iter().collect(),
            "MAMMMXMMMM".to_owned().chars().into_iter().collect(),
            "MXMXAXMASX".to_owned().chars().into_iter().collect(),
        ];

        let result = run(&raw_input);

        assert_eq!(18, result);
    }

    #[test]
    pub fn test_neighbourhood() {
        assert_eq!(
            vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ],
            neighborhood()
        );
    }
}
