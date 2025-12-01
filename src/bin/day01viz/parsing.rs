use std::path::PathBuf;

use aoc2025::read_input;

use super::types::{DialSimulation, Step, UiState};

pub fn parse_steps(input: &str) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() < 2 {
            return Err(format!("Line {}: missing magnitude", idx + 1));
        }
        let (dir, magnitude_str) = line.split_at(1);
        let direction = match dir {
            "L" => -1,
            "R" => 1,
            other => return Err(format!("Line {}: invalid direction '{}'", idx + 1, other)),
        };
        let magnitude: i32 = magnitude_str
            .parse()
            .map_err(|_| format!("Line {}: invalid magnitude '{}'", idx + 1, magnitude_str))?;
        steps.push(Step {
            label: format!("{}{}", dir, magnitude),
            direction: direction * magnitude,
            magnitude,
        });
    }
    if steps.is_empty() {
        Err("No steps found in input".to_string())
    } else {
        Ok(steps)
    }
}

pub fn try_load_default(sim: &mut DialSimulation) -> Result<(), String> {
    let input = std::panic::catch_unwind(|| read_input(1))
        .map_err(|_| "Failed to read Input/day01.txt".to_string())?;
    let steps = parse_steps(&input)?;
    sim.reset_with_steps(steps, Some(PathBuf::from("Input/day01.txt")));
    Ok(())
}

pub fn apply_loaded_input(
    sim: &mut DialSimulation,
    ui_state: &mut UiState,
    contents: String,
    source: Option<PathBuf>,
) -> bool {
    match parse_steps(&contents) {
        Ok(steps) => {
            sim.reset_with_steps(steps, source);
            ui_state.last_error = None;
            true
        }
        Err(err) => {
            ui_state.last_error = Some(err);
            false
        }
    }
}

