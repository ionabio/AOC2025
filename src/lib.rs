use std::fs;
use std::path::Path;
use std::time::Instant;

#[cfg(feature = "viz")]
pub mod viz;

/// A scoped timer that measures and prints execution time
///
/// The timer starts when created and prints the elapsed time when dropped.
///
/// # Example
/// ```
/// use aoc2025::Timer;
///
/// fn some_function() {
///     let _timer = Timer::new("some_function");
///     // ... your code here ...
/// } // Timer automatically prints elapsed time here
/// ```
pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    /// Create a new timer with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("⏱️  {} took: {:?}", self.name, elapsed);
    }
}

/// Macro to easily create a scoped timer
///
/// # Example
/// ```
/// use aoc2025::time_it;
///
/// fn some_function() {
///     time_it!("some_function");
///     // ... your code here ...
/// }
/// ```
#[macro_export]
macro_rules! time_it {
    ($name:expr) => {
        let _timer = $crate::Timer::new($name);
    };
}

/// Read the input file for a given day
///
/// # Arguments
/// * `day` - The day number (e.g., 1 for day01.txt)
///
/// # Returns
/// The contents of the input file as a String
///
/// # Panics
/// Panics if the file cannot be read
pub fn read_input(day: u8) -> String {
    let filename = format!("Input/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Read the input file from a custom path
///
/// # Arguments
/// * `path` - The path to the input file
///
/// # Returns
/// The contents of the input file as a String
///
/// # Panics
/// Panics if the file cannot be read
pub fn read_input_from_path<P: AsRef<Path>>(path: P) -> String {
    let path_ref = path.as_ref();
    fs::read_to_string(path_ref)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path_ref.display()))
}

/// Parse input into lines, filtering out empty lines
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Parse input into lines, keeping empty lines
pub fn parse_all_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "line1\nline2\n\nline3";
        let lines = parse_lines(input);
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_parse_all_lines() {
        let input = "line1\nline2\n\nline3";
        let lines = parse_all_lines(input);
        assert_eq!(lines, vec!["line1", "line2", "", "line3"]);
    }
}
