use aoc2025::{read_input, Timer};

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn contains(&self, other: &i64) -> bool {
        self.start <= *other && self.end >= *other
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            ranges.push(Range::new(
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            ));
        } else {
            numbers.push(line.parse::<i64>().unwrap());
        }
    }
    (ranges, numbers)
}


fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut merged: Vec<Range> = Vec::new();
    if !ranges.is_empty() {
        // Sort by start
        let mut sorted = ranges.clone();
        sorted.sort_by_key(|r| r.start);

        let mut current = sorted[0].clone();
        for r in sorted.iter().skip(1) {
            if r.start <= current.end + 1 {
                // Merge ranges (they overlap or touch)
                current.end = current.end.max(r.end);
            } else {
                merged.push(current);
                current = r.clone();
            }
        }
        merged.push(current);
    }
    merged
}

fn part1(input: &str, verbose: bool) -> i64 {
    let _timer = Timer::new("Part 1");
    let (ranges, numbers) = parse_input(input);
    //let merged = merge_ranges(&ranges);
    let mut total = 0;
    for number in &numbers {
        for range in &ranges {
            if range.contains(&number) {
                total += 1;
                if verbose {
                    println!(
                        "Number {} is in range {}-{}",
                        number, range.start, range.end
                    );
                }
                break;
            }
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    let _timer = Timer::new("Part 2");
    let (ranges, _numbers) = parse_input(input);
    let merged = merge_ranges(&ranges);

    let mut total: i64 = 0;
    for r in &merged {
        total += r.end - r.start + 1;
    }
    total   
}

fn main() {
    let input = read_input(5);
    println!("Part 1: {}", part1(&input, false));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT, true), 3);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }
}
