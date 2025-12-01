use bevy_egui::{egui, EguiContexts};

use aoc2025::viz::{load_input_via_dialog, playback_controls_ui, stats_grid, LoadedInput, PlaybackAction};

use super::parsing::{apply_loaded_input, try_load_default};
use super::types::{DialSimulation, PlaybackResource, UiState};

pub fn ui_system(
    mut contexts: EguiContexts,
    mut sim: ResMut<DialSimulation>,
    mut playback: ResMut<PlaybackResource>,
    mut ui_state: ResMut<UiState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Controls")
        .default_pos(egui::pos2(16.0, 16.0))
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load input...").clicked() {
                    if let Some(LoadedInput { path, contents }) = load_input_via_dialog() {
                        if apply_loaded_input(&mut sim, &mut ui_state, contents, Some(path)) {
                            playback.reset();
                        }
                    }
                }
                if ui.button("Reload day 01").clicked() {
                    if let Err(err) = try_load_default(&mut sim) {
                        ui_state.last_error = Some(err);
                    } else {
                        ui_state.last_error = None;
                        playback.reset();
                    }
                }
            });

            if let Some(err) = &ui_state.last_error {
                ui.colored_label(egui::Color32::RED, err);
            } else {
                ui.label(format!("Source: {}", sim.source_label()));
            }
            ui.separator();

            let action: PlaybackAction = playback_controls_ui(ui, &mut playback.state);
            if action.reset {
                sim.restart();
                playback.reset();
            }
            if action.step_once {
                sim.advance_ticks(1);
            }
        });

    egui::Window::new("Statistics")
        .default_pos(egui::pos2(860.0, 16.0))
        .show(ctx, |ui| {
            let stats = vec![
                ("Current position", sim.position.to_string()),
                ("Zero hits", sim.zero_hits.to_string()),
                ("Rotations processed", sim.rotation_count.to_string()),
                (
                    "Step progress",
                    format!("{} / {}", sim.completed_steps(), sim.total_steps()),
                ),
                (
                    "Current step",
                    sim.current_step_label().unwrap_or("Complete").to_string(),
                ),
                (
                    "Status",
                    if sim.finished {
                        "Complete".to_string()
                    } else if playback.state.playing {
                        "Playing".to_string()
                    } else {
                        "Paused".to_string()
                    },
                ),
            ];
            stats_grid(ui, &stats);

            ui.separator();
            ui.label("Recent events");
            for entry in sim.history.iter().rev() {
                ui.label(entry);
            }
        });
}

