use std::collections::HashSet;

use common::read_lines;

type RawInput = String;

trait Parser<Input, T> {
    fn parse(&self, input: Input) -> Result<(T, Input), String>;
}

struct CharParser {
    c: char,
}

impl Parser<String, char> for CharParser {
    fn parse(&self, input: String) -> Result<(char, String), String> {
        if let Some(c) = input.chars().next() {
            if c == self.c {
                return Ok((c, input.chars().skip(1).collect()));
            }
            return Err(format!("expected char '{}', found '{}'", self.c, c));
        }
        return Err("expected char, found empty string".to_owned());
    }
}

struct DigitParser;

impl Parser<String, u64> for DigitParser {
    fn parse(&self, input: String) -> Result<(u64, String), String> {
        if let Some(c) = input.chars().next() {
            if let Some(digit) = c.to_digit(10) {
                return Ok((digit as u64, input.chars().skip(1).collect()));
            }
            return Err(format!("expected digit, found {}", c));
        }
        return Err("expected digit, found empty string".to_owned());
    }
}

struct AndThenParser<P1, P2> {
    parser1: P1,
    parser2: P2,
}

impl<Input, T1, T2, P1, P2> Parser<Input, (T1, T2)> for AndThenParser<P1, P2>
where
    P1: Parser<Input, T1>,
    P2: Parser<Input, T2>,
{
    fn parse(&self, input: Input) -> Result<((T1, T2), Input), String> {
        let (parsed, rest) = self.parser1.parse(input)?;
        let (parsed_second, rest_second) = self.parser2.parse(rest)?;

        Ok(((parsed, parsed_second), rest_second))
    }
}

struct ManyParser<P> {
    min: u64,
    max: u64,
    parser: P,
}

impl<T, P> Parser<String, Vec<T>> for ManyParser<P>
where
    P: Parser<String, T>,
{
    fn parse(&self, input: String) -> Result<(Vec<T>, String), String> {
        let mut cur = input.clone();
        let mut result = Vec::new();

        while !cur.is_empty() {
            if result.len() as u64 == self.max {
                break;
            }

            let value = self.parser.parse(cur.clone());
            match value {
                Ok((parsed, rest)) => {
                    result.push(parsed);
                    cur = rest
                }
                Err(_) => break,
            }
        }

        if self.min <= result.len() as u64 && result.len() as u64 <= self.max {
            return Ok((result, cur));
        }

        Err(format!(
            "Expected between {} and {} occurences, found {}",
            self.min,
            self.max,
            result.len()
        ))
    }
}

struct EmptyParser;

impl Parser<String, ()> for EmptyParser {
    fn parse(&self, input: String) -> Result<((), String), String> {
        return Ok(((), input));
    }
}

fn run(input: RawInput) -> u64 {
    // mul(X,Y), where X and Y are each 1-3 digit numbers

    let mulParser = AndThenParser {
        parser1: AndThenParser {
            parser1: AndThenParser {
                parser1: CharParser { c: 'm' },
                parser2: CharParser { c: 'u' },
            },
            parser2: CharParser { c: 'l' },
        },
        parser2: AndThenParser {
            parser1: AndThenParser {
                parser1: CharParser { c: '(' },
                parser2: AndThenParser {
                    parser1: ManyParser {
                        min: 1,
                        max: 3,
                        parser: DigitParser,
                    },
                    parser2: AndThenParser {
                        parser1: CharParser { c: ',' },
                        parser2: AndThenParser {
                            parser1: ManyParser {
                                min: 1,
                                max: 3,
                                parser: DigitParser,
                            },
                            parser2: CharParser { c: ')' },
                        },
                    },
                },
            },
            parser2: EmptyParser,
        },
    };

    let mut inp = input.clone();

    let mut result: Vec<(u64, u64)> = Vec::new();

    while !inp.is_empty() {
        // we backtrack by hand
        let s = inp.clone();

        match mulParser.parse(s) {
            Ok((((('m', 'u'), 'l'), (('(', (x1, (',', (x2, ')')))), ())), rest)) => {
                inp = rest;
                result.push((
                    x1.iter().fold(0, |acc, digit| 10 * acc + *digit),
                    x2.iter().fold(0, |acc, digit| 10 * acc + *digit),
                ));
            }
            Ok(x) => {
                panic!("unexpected result {:?}", x)
            }
            Err(_) => inp = inp.chars().skip(1).collect(),
        }
    }

    result.iter().fold(0, |acc, (x1, x2)| acc + (*x1 * *x2))
}

fn main() {
    let lines = read_lines("day3_part1/src/data/input_day3");

    println!("Read {} lines", lines.len());

    let mut total = 0;

    for line in lines {
        let res = run(line);
        println!("Found intermediate result {}", res);
        total += res;
    }

    println!("total is {}", total);
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_example_day3_part1() {
        use crate::run;

        assert_eq!(
            161,
            run(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_owned()
            )
        );
    }

    #[test]
    pub fn test_day3_parsers() {
        use crate::*;

        let p = CharParser { c: 'x' };

        assert_eq!(
            Err("expected char 'x', found 'b'".to_owned()),
            p.parse("blabla".to_owned())
        );

        assert_eq!(
            Ok(('x', "enakis".to_owned())),
            p.parse("xenakis".to_owned())
        );

        let d = DigitParser;

        assert_eq!(
            Err("expected digit, found b".to_owned()),
            d.parse("blabla".to_owned())
        );

        assert_eq!(Ok((1, "234".to_owned())), d.parse("1234".to_owned()));

        let combined = AndThenParser {
            parser1: p,
            parser2: d,
        };

        assert_eq!(
            Ok((('x', 2), "567".to_owned())),
            combined.parse("x2567".to_owned())
        );

        let many = ManyParser {
            parser: combined,
            min: 1,
            max: 3,
        };

        assert_eq!(
            Ok((vec![('x', 2), ('x', 3), ('x', 4)], "567".to_owned())),
            many.parse("x2x3x4567".to_owned())
        );

        let mulParser = AndThenParser {
            parser1: AndThenParser {
                parser1: AndThenParser {
                    parser1: CharParser { c: 'm' },
                    parser2: CharParser { c: 'u' },
                },
                parser2: CharParser { c: 'l' },
            },
            parser2: AndThenParser {
                parser1: AndThenParser {
                    parser1: CharParser { c: '(' },
                    parser2: AndThenParser {
                        parser1: ManyParser {
                            min: 1,
                            max: 3,
                            parser: DigitParser,
                        },
                        parser2: AndThenParser {
                            parser1: CharParser { c: ',' },
                            parser2: AndThenParser {
                                parser1: ManyParser {
                                    min: 1,
                                    max: 3,
                                    parser: DigitParser,
                                },
                                parser2: CharParser { c: ')' },
                            },
                        },
                    },
                },
                parser2: EmptyParser,
            },
        };

        assert_eq!(
            Ok((
                (
                    (('m', 'u'), 'l'),
                    (('(', (vec![1, 2, 3], (',', (vec![2, 3], ')')))), ())
                ),
                "".to_owned()
            )),
            mulParser.parse("mul(123,23)".to_owned())
        );
    }
}
