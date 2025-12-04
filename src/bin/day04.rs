use aoc2025::{read_input, Timer};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = if height > 0 { lines[0].len() } else { 0 };
        let mut cells = Vec::with_capacity(width * height);

        for line in lines {
            for c in line.chars() {
                cells.push(match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!("Invalid character: {}", c),
                });
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < self.height && col < self.width {
            Some(&self.cells[row * self.width + col])
        } else {
            None
        }
    }

    /// Get all 8 neighbors (diagonal and adjacent) for a given position
    /// Returns up to 8 neighbors depending on position (fewer at edges/corners)
    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for dr in -1..=1_i32 {
            for dc in -1..=1_i32 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0 && c >= 0 && (r as usize) < self.height && (c as usize) < self.width {
                    result.push((r as usize, c as usize));
                }
            }
        }
        result
    }

    /// Count filled neighbors in all 8 directions (diagonal + adjacent)
    fn count_filled_neighbors(&self, row: usize, col: usize) -> usize {
        self.neighbors(row, col)
            .iter()
            .filter(|(r, c)| matches!(self.get(*r, *c), Some(Cell::Roll)))
            .count()
    }

    /// Iterator over all positions and their cells: (row, col, &Cell)
    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> + '_ {
        self.cells.iter().enumerate().map(|(idx, cell)| {
            let row = idx / self.width;
            let col = idx % self.width;
            (row, col, cell)
        })
    }
    
    /// Set a cell at the given position to Empty (used for removal)
    fn set_empty(&mut self, row: usize, col: usize) {
        if row < self.height && col < self.width {
            self.cells[row * self.width + col] = Cell::Empty;
        }
    }
    
    /// Find all accessible rolls (those with < 4 filled neighbors)
    fn find_accessible(&self) -> Vec<(usize, usize)> {
        self.iter()
            .filter(|(row, col, cell)| {
                **cell == Cell::Roll && self.count_filled_neighbors(*row, *col) < 4
            })
            .map(|(row, col, _)| (row, col))
            .collect()
    }
}

// Implement IntoIterator for &Grid to allow: for (row, col, cell) in &grid { ... }
impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, &'a Cell);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

fn part1(input: &str) -> i32 {
    let _timer = Timer::new("Part 1");
    let grid = Grid::new(input);

    // Count rolls that have fewer than 4 filled neighbors
    // (accessible by forklifts)
    grid.iter()
        .filter(|(row, col, cell)| {
            **cell == Cell::Roll && grid.count_filled_neighbors(*row, *col) < 4
        })
        .count() as i32
}

fn part2(input: &str) -> i32 {
    let _timer = Timer::new("Part 2");
    let mut grid = Grid::new(input);
    let mut total_removed = 0;
    
    // Keep removing accessible rolls until none are left
    loop {
        let accessible = grid.find_accessible();
        
        if accessible.is_empty() {
            break;
        }
        
        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid.set_empty(*row, *col);
        }
        
        total_removed += accessible.len();
    }
    
    total_removed as i32
}

fn main() {
    let input = read_input(4);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }
}
