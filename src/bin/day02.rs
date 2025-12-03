use aoc2025::{read_input, Timer};
use std::collections::HashSet;

/// Parse input into list of (start, end) ranges
fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let parts: Vec<&str> = range_str.trim().split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<i64>().ok()?;
                let end = parts[1].parse::<i64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

/// Generate all invalid IDs with exactly 2 repetitions in given ranges
fn generate_invalid_ids_part1(ranges: &[(i64, i64)]) -> Vec<i64> {
    let mut invalid_ids = HashSet::new();
    
    for &(start, end) in ranges {
        let start_digits = if start == 0 { 1 } else { (start as f64).log10().floor() as usize + 1 };
        let end_digits = (end as f64).log10().floor() as usize + 1;
        
        // For each possible number of digits in range
        for num_digits in start_digits..=end_digits {
            // Part 1: exactly 2 repetitions, so pattern length is num_digits/2
            if num_digits % 2 == 0 {
                let pattern_len = num_digits / 2;
                
                // Multiplier for 2 repetitions: pattern * (10^pattern_len + 1)
                // e.g., pattern=12, result=1212: 12 * (100 + 1) = 12 * 101 = 1212
                let multiplier = 10_i64.pow(pattern_len as u32) + 1;
                
                // Range of valid patterns (no leading zeros)
                let pattern_min = 10_i64.pow((pattern_len - 1) as u32);
                let pattern_max = 10_i64.pow(pattern_len as u32) - 1;
                
                for pattern in pattern_min..=pattern_max {
                    let id = pattern * multiplier;
                    if id >= start && id <= end {
                        invalid_ids.insert(id);
                    }
                }
            }
        }
    }
    
    invalid_ids.into_iter().collect()
}

/// Generate all invalid IDs with at least 2 repetitions in given ranges
fn generate_invalid_ids_part2(ranges: &[(i64, i64)]) -> Vec<i64> {
    let mut invalid_ids = HashSet::new();
    
    for &(start, end) in ranges {
        // num of digits of start and end
        let start_digits = if start == 0 { 1 } else { (start as f64).log10().floor() as usize + 1 };
        let end_digits = (end as f64).log10().floor() as usize + 1;
        
        // For each possible number of digits in range
        for num_digits in start_digits..=end_digits {
            // For each pattern length that divides num_digits
            for pattern_len in 1..=num_digits/2 {
                // pattern length should fit in the whole number of digits
                if num_digits % pattern_len == 0 {
                    let reps = num_digits / pattern_len;
                    
                    // Calculate multiplier for repetition
                    // e.g., for 3 reps of 2-digit: pattern * (10000 + 100 + 1) = pattern * 10101
                    let mut multiplier = 0i64;
                    for i in 0..reps {
                        multiplier += 10_i64.pow((i * pattern_len) as u32);
                    }
                    
                    // Range of valid patterns (no leading zeros)
                    // e.g., for 2-digit: 10-99
                    let pattern_min = if pattern_len == 1 { 1 } else { 10_i64.pow((pattern_len - 1) as u32) };
                    let pattern_max = 10_i64.pow(pattern_len as u32) - 1;
                    
                    for pattern in pattern_min..=pattern_max {
                        // e.g., for 3 reps of 2-digit: pattern * (10000 + 100 + 1) = pattern * 10101
                        let id = pattern * multiplier;
                        if id >= start && id <= end {
                            invalid_ids.insert(id);
                        }
                    }
                }
            }
        }
    }
    
    invalid_ids.into_iter().collect()
}

fn part1(input: &str, _verbose: bool) -> i64 {
    let _timer = Timer::new("Part 1");
    let ranges = parse_ranges(input);
    let invalid_ids = generate_invalid_ids_part1(&ranges);
    invalid_ids.iter().sum()
}

fn part2(input: &str, _verbose: bool) -> i64 {
    let _timer = Timer::new("Part 2");
    let ranges = parse_ranges(input);
    let invalid_ids = generate_invalid_ids_part2(&ranges);
    invalid_ids.iter().sum()
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
        assert_eq!(part1(EXAMPLE_INPUT, true), 1227775554);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT, true), 4174379265);
    }
}
