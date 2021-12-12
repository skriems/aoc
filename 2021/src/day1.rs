const INPUT: &'static str = include_str!("../inputs/day1.txt");

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let count = parse_part1(INPUT);
    println!("day1 part1: {:?}", count);
}

fn part2() {
    let count = parse_part2(INPUT);
    println!("day1 part2: {:?}", count);
}

fn parse_part1(input: &str) -> usize {
    // array_windows approach by yosh: two lines! ¯\_(ツ)_/¯
    let lines: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let count = lines.array_windows().filter(|[n1, n2]| n2 > n1).count();

    // My original imperative approach. But hey - no unwrap, so it's more robust :P
    //
    // let mut count: usize = 0;
    // let mut last: Option<usize> = None;
    // for num in input.lines().map(|line| line.parse::<usize>()).into_iter() {
    //     if let Ok(number) = num {
    //         if last.is_none() {
    //             last = Some(number);
    //             continue;
    //         }
    //         if let Some(l) = last {
    //             count = if number > l { count + 1 } else { count };
    //         }
    //         last = Some(number);
    //     }
    // }
    count
}

fn parse_part2(input: &str) -> usize {
    // 199  A
    // 200  A B
    // 208  A B C
    // 210    B C D
    // 200  E   C D
    // 207  E F   D
    // 240  E F G
    // 269    F G H
    // 260      G H
    // 263        H
    let lines: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let count = lines
        .array_windows()
        .map(|[n1, n2, n3]| n1 + n2 + n3)
        .collect::<Vec<usize>>()
        .array_windows()
        .filter(|[n1, n2]| n2 > n1)
        .count();
    count
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1() {
        assert_eq!(parse_part1(TEST_INPUT), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(parse_part2(TEST_INPUT), 5);
    }
}
