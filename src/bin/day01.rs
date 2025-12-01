use aoc2025::{read_input, Timer};

fn parse_input(input: &str) -> Vec<i32> {
    let mut steps = Vec::new();
    for line in input.lines() {
        let (s_dir, s_size) = line.split_at(1);
        let size: i32 = s_size.parse::<i32>().unwrap()
            * match s_dir {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction"),
            };
        steps.push(size);
    }
    steps
}

fn part1(input: &str, verbose: bool) -> i32 {
    let _timer = Timer::new("Part 1");
    let mut current_position = 50;
    let mut number_of_zeros = 0;
    let number_of_dials = 100;
    //make an enum of direction L, R

    let steps = parse_input(input);
    for step in steps {
        current_position += step;
        current_position = current_position.rem_euclid(number_of_dials);
        if current_position == 0 {
            number_of_zeros += 1;
        }
        if verbose {
            println!(
                "Dial is rotated to {}{} to point at {} ",
                if step < 0 { "L" } else { "R" },
                step.abs(),
                current_position
            );
        }
    }
    if verbose {
        println!("Number of zeros: {}", number_of_zeros);
    }
    number_of_zeros
}

#[allow(dead_code)]
fn part2(input: &str, verbose: bool) -> i32 {
    let _timer = Timer::new("Part 2");
    let mut current_position = 50;
    let mut number_of_zeros = 0;
    let number_of_dials = 100;
    let steps = parse_input(input);

    for step in steps {
        let mut count = 0;
        let distance = step.abs();
        let direction: i32 = if step > 0 { 1 } else { -1 };

        for _ in 0..distance {
            current_position = (current_position + direction).rem_euclid(number_of_dials);
            if current_position == 0 {
                count += 1;
            }
        }

        number_of_zeros += count;

        if verbose {
            println!(
                "Dial is rotated {}{} to point at {}",
                if step < 0 { "L" } else { "R" },
                step.abs(),
                current_position
            );
            if count > 0 {
                println!(
                    " during this rotation, it points at zero, {} time{}.",
                    count,
                    if count == 1 { "" } else { "s" }
                );
            }
        }
    }

    if verbose {
        println!("Total times pointing at zero: {}", number_of_zeros);
    }
    number_of_zeros
}

fn part2_fast(input: &str, verbose: bool) -> i32 {
    let _timer = Timer::new("Part 2 Fast");
    let mut current_position = 50;
    let mut number_of_zeros = 0;
    let number_of_dials = 100;
    let steps = parse_input(input);

    for step in steps {
        let distance = step.abs();  
        let count = if step > 0 {
            // Moving right: count = floor((current + distance) / dials)
            (current_position + distance) / number_of_dials
        } else {
            if current_position == 0 {
                // Special case: starting at 0, we hit it every full rotation
                distance / number_of_dials
            } else if distance < current_position {
                // We don't reach 0
                0
            } else {
                // We cross 0 at least once, then every full rotation
                1 + (distance - current_position) / number_of_dials
            }
        };
        
        // Update position
        current_position = (current_position + step).rem_euclid(number_of_dials);
        number_of_zeros += count;

        if verbose {
            println!(
                "Dial is rotated {}{} to point at {}",
                if step < 0 { "L" } else { "R" },
                step.abs(),
                current_position
            );
            if count > 0 {
                println!(
                    " during this rotation, it points at zero, {} time{}.",
                    count,
                    if count == 1 { "" } else { "s" }
                );
            }
        }
    }

    if verbose {
        println!("Total times pointing at zero: {}", number_of_zeros);
    }
    number_of_zeros
}

fn main() {
    let input = read_input(1);
    println!("Part 1: {}", part1(&input, false));
    println!("Part 2: {}", part2_fast(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT, true), 3); // TODO: Update with expected result
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT, true), 6); // TODO: Update with expected result
    }

    #[test]
    fn test_part2_fast() {
        assert_eq!(part2_fast(EXAMPLE_INPUT, true), 6);
    }

    #[test]
    fn test_part2_equivalence() {
        // Verify that part2_fast produces the same result as part2
        assert_eq!(part2_fast(EXAMPLE_INPUT, false), part2(EXAMPLE_INPUT, false));
    }
}
