const INPUT: &'static str = include_str!("../inputs/day3.txt");

pub fn run() {
    let res = parse1(INPUT);
    println!("day3 part1: {:?}", res);
    let res = parse2(INPUT);
    println!("day3 part2: {:?}", res);
}

fn parse1(input: &str) -> usize {
    let threshold = input.lines().count() / 2;
    let line_count = input.lines().take(1).next().unwrap().len();

    let count = input.lines().fold(vec![0; line_count], |mut acc, line| {
        for (idx, c) in line.char_indices() {
            if c.to_string() == "1" {
                acc[idx] = acc[idx] + 1;
            }
        }
        acc
    });

    let most_common: Vec<_> = count
        .iter()
        .map(|count| if *count >= threshold { 1 } else { 0 })
        .collect();

    let least_common = most_common
        .iter()
        .map(|count| if *count == 1 { 0 } else { 1 });

    let gamma_rate = most_common.iter().fold(0, |acc, curr| (acc << 1) | curr);
    let epsilon_rate = least_common.fold(0, |acc, curr| (acc << 1) | curr);
    gamma_rate * epsilon_rate
}

fn parse(input: &str, most_common: bool) -> usize {
    let mut lines: Vec<_> = input.lines().collect();
    let mut line_count = lines.iter().count();

    let mut index = 0;
    while line_count > 1 {
        let ones = lines.iter().fold(0, |acc, line| {
            if line.chars().nth(index).unwrap().to_string() == "1" {
                acc + 1
            } else {
                acc
            }
        });

        let zeros = line_count - ones;
        let wanted = if ones >= zeros {
            if most_common {
                "1"
            } else {
                "0"
            }
        } else {
            if most_common {
                "0"
            } else {
                "1"
            }
        };

        lines = lines
            .into_iter()
            .filter(|line| line.chars().nth(index).unwrap().to_string() == wanted)
            .collect();

        // println!(
        //     "iteration {:>2?}: lines:{:>4?}, zeros:{:>3?}, ones:{:>3?}, wanted:{:?}",
        //     &index + 1,
        //     &line_count,
        //     &zeros,
        //     &ones,
        //     &wanted
        // );
        line_count = lines.iter().count();
        index += 1;
    }
    let parsed = usize::from_str_radix(lines[0], 2).unwrap();
    // println!("found: {:?} or {:?}", lines[0], &parsed);
    parsed
}

fn parse2(input: &str) -> usize {
    let ox = parse(input, true);
    let co2 = parse(input, false);
    ox * co2
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1() {
        assert_eq!(parse1(TEST_INPUT), 198);
    }

    #[test]
    fn part2() {
        assert_eq!(parse2(TEST_INPUT), 230);
    }
}
