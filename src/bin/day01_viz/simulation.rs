use std::f32::consts::TAU;

use super::constants::{DIAL_POSITIONS, HISTORY_LIMIT, START_POSITION};
use super::types::DialSimulation;

impl DialSimulation {
    pub fn reset_with_steps(&mut self, steps: Vec<super::types::Step>, source: Option<std::path::PathBuf>) {
        self.steps = steps;
        self.source = source;
        self.history.clear();
        self.reset_state();
        self.push_history(format!("Loaded {} steps", self.steps.len()));
    }

    pub fn restart(&mut self) {
        self.history.clear();
        self.reset_state();
        self.push_history("Simulation reset".to_string());
    }

    pub fn reset_state(&mut self) {
        self.position = START_POSITION;
        self.zero_hits = 0;
        self.rotation_count = 0;
        self.current_step_idx = 0;
        self.ticks_remaining = 0;
        self.current_direction = 0;
        self.current_step_hits = 0;
        self.finished = self.steps.is_empty();
        if !self.finished {
            self.prepare_next_step();
        }
    }

    pub fn prepare_next_step(&mut self) {
        while self.current_step_idx < self.steps.len() {
            let step = &self.steps[self.current_step_idx];
            self.current_direction = step.direction.signum().clamp(-1, 1);
            self.ticks_remaining = step.magnitude;
            self.current_step_hits = 0;
            if self.ticks_remaining == 0 {
                self.record_step_summary();
                self.current_step_idx += 1;
                continue;
            }
            self.finished = false;
            return;
        }
        self.finished = true;
    }

    pub fn advance_ticks(&mut self, ticks: u32) -> bool {
        if self.steps.is_empty() || ticks == 0 {
            return false;
        }
        let mut progressed = false;
        for _ in 0..ticks {
            if self.finished {
                break;
            }
            if self.ticks_remaining == 0 {
                self.prepare_next_step();
                if self.finished {
                    break;
                }
            }
            self.position = (self.position + self.current_direction).rem_euclid(self.dial_size);
            self.rotation_count += 1;
            if self.position == 0 {
                self.zero_hits += 1;
                self.current_step_hits += 1;
                self.push_history(format!("Hit zero at rotation {}", self.rotation_count));
            }
            self.ticks_remaining -= 1;
            progressed = true;
            if self.ticks_remaining == 0 {
                self.record_step_summary();
                self.current_step_idx += 1;
                self.prepare_next_step();
            }
        }
        progressed
    }

    pub fn record_step_summary(&mut self) {
        if self.current_step_idx >= self.steps.len() {
            return;
        }
        let step = &self.steps[self.current_step_idx];
        let summary = format!(
            "{} -> pos {} (zeros +{})",
            step.label, self.position, self.current_step_hits
        );
        self.push_history(summary);
    }

    pub fn push_history(&mut self, entry: String) {
        if self.history.len() == HISTORY_LIMIT {
            self.history.pop_front();
        }
        self.history.push_back(entry);
    }

    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    pub fn completed_steps(&self) -> usize {
        self.current_step_idx.min(self.steps.len())
    }

    pub fn current_step_label(&self) -> Option<&str> {
        self.steps
            .get(self.current_step_idx)
            .map(|step| step.label.as_str())
    }

    pub fn source_label(&self) -> String {
        self.source
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Default puzzle input".to_string())
    }

    pub fn angle(&self) -> f32 {
        let fraction = self.position as f32 / self.dial_size as f32;
        TAU * fraction
    }
}

