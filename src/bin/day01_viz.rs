mod day01viz;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_egui::EguiPrimaryContextPass;

use day01viz::{
    load_default_input, maintain_aspect_ratio, run_simulation, scale_dial_elements,
    setup_background, setup_visuals, update_background_on_resize, update_pointer_visual,
    ui_system, DialSimulation, PlaybackResource, UiState, WindowSizeTracker,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(PlaybackResource::default())
        .insert_resource(DialSimulation::default())
        .insert_resource(UiState::default())
        .insert_resource(WindowSizeTracker { last_height: 768.0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AoC 2025 - Day 01 Visualization".into(),
                // Resolution with 2816:1536 aspect ratio (simplified to ~1.83:1)
                resolution: WindowResolution::new(1408, 768),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_background, setup_visuals, load_default_input))
        .add_systems(
            Update,
            (
                maintain_aspect_ratio,
                update_background_on_resize,
                scale_dial_elements,
                run_simulation,
                update_pointer_visual,
            )
                .chain(),
        )
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}
