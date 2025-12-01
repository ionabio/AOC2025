use bevy::prelude::*;

use crate::day01::parsing::try_load_default;
use crate::day01::types::{DialSimulation, PlaybackResource, UiState};

pub fn run_simulation(
    time: Res<Time>,
    mut playback: ResMut<PlaybackResource>,
    mut sim: ResMut<DialSimulation>,
) {
    if playback.state.playing {
        playback.accumulator += time.delta_secs() * playback.state.speed.max(0.1);
        let steps_to_advance = playback.accumulator.floor() as u32;
        if steps_to_advance > 0 {
            playback.accumulator -= steps_to_advance as f32;
            sim.advance_ticks(steps_to_advance);
        }
        if sim.finished {
            playback.state.playing = false;
        }
    }
}

pub fn load_default_input(mut sim: ResMut<DialSimulation>, mut ui: ResMut<UiState>) {
    if let Err(err) = try_load_default(&mut sim) {
        ui.last_error = Some(err);
    } else {
        ui.last_error = None;
    }
}

