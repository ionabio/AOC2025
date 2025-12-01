use std::{fs, path::PathBuf};

use bevy_egui::egui;
use rfd::FileDialog;

/// Represents the currently loaded input file and its contents.
pub struct LoadedInput {
    pub path: PathBuf,
    pub contents: String,
}

/// Open a file dialog and return the chosen file and its contents, if any.
pub fn load_input_via_dialog() -> Option<LoadedInput> {
    let path = FileDialog::new().pick_file()?;
    let contents = fs::read_to_string(&path).ok()?;
    Some(LoadedInput { path, contents })
}

/// Stateful playback controller shared across visualizations.
#[derive(Debug)]
pub struct PlaybackState {
    pub playing: bool,
    pub speed: f32,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            playing: false,
            speed: 1.0,
        }
    }
}

/// Actions emitted from the playback controls panel.
#[derive(Default, Debug)]
pub struct PlaybackAction {
    pub step_once: bool,
    pub reset: bool,
}

/// Render playback controls and mutate the state based on user interaction.
pub fn playback_controls_ui(ui: &mut egui::Ui, state: &mut PlaybackState) -> PlaybackAction {
    let mut action = PlaybackAction::default();

    ui.horizontal(|ui| {
        if ui
            .button(if state.playing { "Pause" } else { "Play" })
            .clicked()
        {
            state.playing = !state.playing;
        }
        if ui.button("Step").clicked() {
            action.step_once = true;
        }
        if ui.button("Reset").clicked() {
            action.reset = true;
        }
    });

    ui.add(
        egui::Slider::new(&mut state.speed, 0.1..=5.0)
            .logarithmic(true)
            .text("Speed (steps/sec)"),
    );

    action
}

/// Render a key-value stats grid.
pub fn stats_grid(ui: &mut egui::Ui, stats: &[(&str, String)]) {
    egui::Grid::new("viz_stats_grid")
        .striped(true)
        .show(ui, |ui| {
            for (label, value) in stats {
                ui.label(*label);
                ui.label(value);
                ui.end_row();
            }
        });
}
