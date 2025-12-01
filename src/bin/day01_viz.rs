mod day01viz;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_egui::EguiPrimaryContextPass;

use day01viz::{
    load_default_input, run_simulation, setup_visuals, update_pointer_visual, ui_system,
    DialSimulation, PlaybackResource, UiState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(PlaybackResource::default())
        .insert_resource(DialSimulation::default())
        .insert_resource(UiState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AoC 2025 - Day 01 Visualization".into(),
                resolution: WindowResolution::new(1200, 800),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_visuals, load_default_input))
        .add_systems(Update, (run_simulation, update_pointer_visual))
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}
