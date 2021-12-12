const INPUT: &'static str = include_str!("../inputs/day2.txt");

pub fn run() {
    let res = parse1(INPUT);
    println!("day2 part1: {:?}", res);
    let res = parse2(INPUT);
    println!("day2 part2: {:?}", res);
}

fn parse1(input: &str) -> usize {
    let (horizontal, depth) = input
        .lines()
        .map(|line| line.split(" ").into_iter().collect::<Vec<_>>())
        .fold((0, 0), |acc, curr| {
            let (mut horizontal, mut depth) = acc;
            match curr[0] {
                "forward" => {
                    horizontal += curr[1].parse::<usize>().unwrap();
                }
                "up" => {
                    depth -= curr[1].parse::<usize>().unwrap();
                }
                "down" => {
                    depth += curr[1].parse::<usize>().unwrap();
                }
                _ => (),
            }
            (horizontal, depth)
        });
    horizontal * depth
}

fn parse2(input: &str) -> usize {
    let (horizontal, depth, _aim) = input
        .lines()
        .map(|line| line.split(" ").into_iter().collect::<Vec<_>>())
        .fold((0, 0, 0), |acc, curr| {
            let (mut horizontal, mut depth, mut aim) = acc;
            match curr[0] {
                "forward" => {
                    let val = curr[1].parse::<usize>().unwrap();
                    horizontal += val;
                    depth += val * aim;
                }
                "up" => {
                    aim -= curr[1].parse::<usize>().unwrap();
                }
                "down" => {
                    aim += curr[1].parse::<usize>().unwrap();
                }
                _ => (),
            }
            (horizontal, depth, aim)
        });
    horizontal * depth
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1() {
        assert_eq!(parse1(TEST_INPUT), 150);
    }

    #[test]
    fn part2() {
        assert_eq!(parse2(TEST_INPUT), 900);
    }
}
