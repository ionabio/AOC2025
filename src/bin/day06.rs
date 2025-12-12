use aoc2025::{read_input, Timer};

enum Symbol {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<Symbol>) {
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for line in input.lines() {
        let mut nums = Vec::new();
        for token in line.split_whitespace() {
            // Try to parse as i64, else treat as symbol
            if let Ok(n) = token.parse::<i64>() {
                nums.push(n);
            } else {
                symbols.push(match token {
                    "+" => Symbol::Add,
                    "*" => Symbol::Multiply,
                    _ => panic!("Invalid symbol: {}", token),
                });
            }
        }
        if !nums.is_empty() {
            numbers.push(nums);
        }
    }

    (numbers, symbols)
}

fn parse_input_part2(input: &str) -> (Vec<Vec<i64>>, Vec<Symbol>) {
    let mut all_problems: Vec<Vec<i64>> = Vec::new();
    
    //parse the last line to symbols
    let symbols: Vec<Symbol> = input.lines().last().unwrap().split_whitespace().map(|s| match s {
        "+" => Symbol::Add,
        "*" => Symbol::Multiply,
        _ => panic!("Invalid symbol: {}", s),
    }).collect();

    //remove the last line from the input
    let lines: Vec<&str> = input.lines().collect();
    let number_lines: Vec<&str> = lines[..lines.len()-1].iter().copied().collect();

    let max_line_length = number_lines.iter().map(|line| line.len()).max().unwrap();
    
    let mut current_problem: Vec<i64> = Vec::new();
    
    // Read columns from right to left
    for i in (0..max_line_length).rev() {
        // Check if this column is all spaces
        let mut all_spaces = true;
        let mut column_string = String::new();
        
        for line in &number_lines {
            let ch = line.chars().nth(i);
            if let Some(c) = ch {
                if c != ' ' {
                    all_spaces = false;
                    column_string.push(c);
                }
            }
            // If ch is None (line is too short), treat as space
        }
        
        if all_spaces {
            // This column is a separator
            // Save the current problem if it's not empty
            if !current_problem.is_empty() {
                all_problems.push(current_problem.clone());
                current_problem.clear();
            }
        } else {
            // This column contains part of a number
            // The column_string contains digits from top to bottom, which forms one number
            if !column_string.is_empty() {
                // Add the entire column as one number (reading top-to-bottom)
                current_problem.push(column_string.parse::<i64>().unwrap());
            }
        }
    }
    
    // Don't forget the last problem
    if !current_problem.is_empty() {
        all_problems.push(current_problem);
    }
    
    // Reverse because we built from right to left but need left to right for symbols
    all_problems.reverse();

    (all_problems, symbols)
}

fn transpose_numbers_part1(numbers: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut transposed = Vec::new();
    for i in 0..numbers[0].len() {
        let mut row = Vec::new();
        for number in numbers.iter() {
            row.push(number[i]);
        }
        transposed.push(row);
    }
    transposed
}

fn part1(input: &str) -> i64 {
    let _timer = Timer::new("Part 1");
    let (numbers, symbols) = parse_input(input);
    let transposed = transpose_numbers_part1(&numbers);
    let mut total = 0;
    for (i, row) in transposed.iter().enumerate() {
        let symbol = &symbols[i];
        total += match symbol {
            Symbol::Add => row.iter().sum::<i64>(),
            Symbol::Multiply => row.iter().product::<i64>(),
        };
    }
    total
}

fn part2(input: &str) -> i64 {
    let _timer = Timer::new("Part 2");
    let (numbers, symbols) = parse_input_part2(input);
    let mut total = 0;
    for (i, row) in numbers.iter().enumerate() {
        let symbol = &symbols[i];
        total += match symbol { 
            Symbol::Add => row.iter().sum::<i64>(),
            Symbol::Multiply => row.iter().product::<i64>(),
        };
    }
    total
}

fn main() {
    let input = read_input(6);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   + ";
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }
    #[test]
    fn test_part2() {
        let result = part2(EXAMPLE_INPUT);
        println!("Part 2 result: {}", result);
        assert_eq!(result, 3263827);
    }
    
    #[test]
    fn test_parse_part2() {
        let (problems, symbols) = parse_input_part2(EXAMPLE_INPUT);
        println!("Problems: {:?}", problems);
        println!("Symbols: {:?}", symbols.len());
        
        // Expected problems from right to left:
        // Problem 4 (rightmost): 4 + 431 + 623 = 1058
        // Problem 3: 175 * 581 * 32 = 3253600  
        // Problem 2: 8 + 248 + 369 = 625
        // Problem 1 (leftmost): 356 * 24 * 1 = 8544
        
        // Since we read right-to-left and then reverse, problems should be:
        // [0]: 356, 24, 1 (leftmost)
        // [1]: 8, 248, 369
        // [2]: 175, 581, 32
        // [3]: 4, 431, 623 (rightmost)
        
        assert_eq!(problems.len(), 4);
        assert_eq!(symbols.len(), 4);
    }
}
