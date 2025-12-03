use aoc2025::{read_input, Timer};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
    grid
}
fn part1(input: &str, verbose: bool) -> i64 {
    let _timer = Timer::new("Part 1");
    let grid = parse_input(input);
    let mut total = 0;
    for row in grid {
        // Find max in all positions except the last
        let (max_pos, &max_val) = row[..row.len()-1]
            .iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .unwrap();
        
        // Find max in all positions after max_pos
        let &second_max = row[max_pos+1..].iter().max().unwrap();
        
        let max_joltage = max_val as i64 * 10 + second_max as i64;
        
        if verbose {
            println!("Max joltage: {}", max_joltage);
        }
        total += max_joltage;
    }
    if verbose {
        println!("Total: {}", total);
    }
    total
}

fn part2(input: &str, verbose: bool) -> i64 {
    let _timer = Timer::new("Part 2");
    let grid = parse_input(input);
    let mut total = 0;
    
    for row in grid {
        let n = row.len();
        let k = 12; // Pick 12 batteries
        let mut result = Vec::new();
        let mut last_pos = -1i32; // Position of last picked digit
        
        for i in 0..k {
            // We need k-i-1 more digits after this one
            let remaining = k - i - 1;
            let start = (last_pos + 1) as usize;
            let end = n - remaining;
            
            // Find max in range [start..end] (pick first occurrence on tie)
            let max_val = *row[start..end].iter().max().unwrap();
            // we need the first occurrence in case of ties
            let max_pos = row[start..end]
                .iter()
                .position(|&v| v == max_val)
                .unwrap();
            
            let actual_pos = start + max_pos;
            result.push(max_val);
            last_pos = actual_pos as i32;
        }
        
        // Convert result to number
        let mut joltage = 0i64;
        for &digit in &result {
            joltage = joltage * 10 + digit as i64;
        }
        
        if verbose {
            println!("Max joltage: {}", joltage);
        }
        total += joltage;
    }
    
    if verbose {
        println!("Total: {}", total);
    }
    total
}

fn main() {
    let input = read_input(3);
    println!("Part 1: {}", part1(&input, false));
    println!("Part 2: {}", part2(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT, true), 357);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT, true), 3121910778619);
    }
}
