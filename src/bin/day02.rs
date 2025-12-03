use aoc2025::{read_input, Timer};

fn part1(input: &str, _verbose: bool) -> i32 {
    let _timer = Timer::new("Part 1");
    let _ranges: Vec<Vec<i32>> = input.split(',').map(|range| range.split('-').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    return 0;
}

fn part2(_input: &str, _verbose: bool) -> i32 {
    let _timer = Timer::new("Part 2");

    return 0;
}

fn main() {
    let input = read_input(2);
    println!("Part 1: {}", part1(&input, false));
    println!("Part 2: {}", part2(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str ="11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT, true), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT, true), 0);
    }
}
