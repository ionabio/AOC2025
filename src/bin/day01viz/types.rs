use std::{collections::VecDeque, path::PathBuf};

use aoc2025::viz::PlaybackState;

use super::constants::{DIAL_POSITIONS, HISTORY_LIMIT, START_POSITION};

#[derive(Clone)]
pub struct Step {
    pub label: String,
    pub direction: i32,
    pub magnitude: i32,
}

#[derive(bevy::prelude::Resource)]
pub struct DialSimulation {
    pub steps: Vec<Step>,
    pub dial_size: i32,
    pub position: i32,
    pub zero_hits: i32,
    pub rotation_count: i32,
    pub current_step_idx: usize,
    pub ticks_remaining: i32,
    pub current_direction: i32,
    pub current_step_hits: i32,
    pub history: VecDeque<String>,
    pub finished: bool,
    pub source: Option<PathBuf>,
}

impl Default for DialSimulation {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            dial_size: DIAL_POSITIONS,
            position: START_POSITION,
            zero_hits: 0,
            rotation_count: 0,
            current_step_idx: 0,
            ticks_remaining: 0,
            current_direction: 0,
            current_step_hits: 0,
            history: VecDeque::with_capacity(HISTORY_LIMIT),
            finished: true,
            source: None,
        }
    }
}

#[derive(bevy::prelude::Resource, Default)]
pub struct PlaybackResource {
    pub state: PlaybackState,
    pub accumulator: f32,
}

impl PlaybackResource {
    pub fn reset(&mut self) {
        self.state.playing = false;
        self.accumulator = 0.0;
    }
}

#[derive(bevy::prelude::Resource, Default)]
pub struct UiState {
    pub last_error: Option<String>,
}

#[derive(bevy::prelude::Component)]
pub struct PointerRoot;

#[derive(bevy::prelude::Component)]
pub struct BackgroundSprite;

#[derive(bevy::prelude::Component)]
pub struct DialElement;

#[derive(bevy::prelude::Component)]
pub struct TickElement {
    pub angle: f32,
}

#[derive(bevy::prelude::Resource)]
pub struct WindowSizeTracker {
    pub last_height: f32,
}

