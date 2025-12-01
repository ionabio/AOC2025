use std::{
    collections::VecDeque,
    f32::consts::{FRAC_PI_2, TAU},
    path::PathBuf,
};

use aoc2025::{
    read_input,
    viz::{
        load_input_via_dialog, playback_controls_ui, stats_grid, LoadedInput, PlaybackAction,
        PlaybackState,
    },
};
use bevy::{
    math::primitives::{Circle, Rectangle},
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

const DIAL_POSITIONS: i32 = 100;
const START_POSITION: i32 = 50;
const HISTORY_LIMIT: usize = 14;
const DIAL_RADIUS: f32 = 220.0;

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

#[derive(Resource, Default)]
struct PlaybackResource {
    state: PlaybackState,
    accumulator: f32,
}

impl PlaybackResource {
    fn reset(&mut self) {
        self.state.playing = false;
        self.accumulator = 0.0;
    }
}

#[derive(Resource)]
struct DialSimulation {
    steps: Vec<Step>,
    dial_size: i32,
    position: i32,
    zero_hits: i32,
    rotation_count: i32,
    current_step_idx: usize,
    ticks_remaining: i32,
    current_direction: i32,
    current_step_hits: i32,
    history: VecDeque<String>,
    finished: bool,
    source: Option<PathBuf>,
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

impl DialSimulation {
    fn reset_with_steps(&mut self, steps: Vec<Step>, source: Option<PathBuf>) {
        self.steps = steps;
        self.source = source;
        self.history.clear();
        self.reset_state();
        self.push_history(format!("Loaded {} steps", self.steps.len()));
    }

    fn restart(&mut self) {
        self.history.clear();
        self.reset_state();
        self.push_history("Simulation reset".to_string());
    }

    fn reset_state(&mut self) {
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

    fn prepare_next_step(&mut self) {
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

    fn advance_ticks(&mut self, ticks: u32) -> bool {
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

    fn record_step_summary(&mut self) {
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

    fn push_history(&mut self, entry: String) {
        if self.history.len() == HISTORY_LIMIT {
            self.history.pop_front();
        }
        self.history.push_back(entry);
    }

    fn total_steps(&self) -> usize {
        self.steps.len()
    }

    fn completed_steps(&self) -> usize {
        self.current_step_idx.min(self.steps.len())
    }

    fn current_step_label(&self) -> Option<&str> {
        self.steps
            .get(self.current_step_idx)
            .map(|step| step.label.as_str())
    }

    fn source_label(&self) -> String {
        self.source
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Default puzzle input".to_string())
    }

    fn angle(&self) -> f32 {
        let fraction = self.position as f32 / self.dial_size as f32;
        TAU * fraction
    }
}

#[derive(Resource, Default)]
struct UiState {
    last_error: Option<String>,
}

#[derive(Component)]
struct PointerRoot;

fn setup_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let outer_ring = meshes.add(Circle::new(DIAL_RADIUS + 28.0));
    let inner_fill = meshes.add(Circle::new(DIAL_RADIUS - 18.0));
    let outer_material = materials.add(ColorMaterial::from_color(Color::srgb(0.08, 0.08, 0.12)));
    let inner_material = materials.add(ColorMaterial::from_color(Color::srgb(0.02, 0.02, 0.05)));

    commands.spawn((
        Mesh2d(outer_ring),
        MeshMaterial2d(outer_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
    ));
    commands.spawn((
        Mesh2d(inner_fill),
        MeshMaterial2d(inner_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.25)),
    ));

    let major_tick_mesh = meshes.add(Rectangle::new(6.0, 28.0));
    let minor_tick_mesh = meshes.add(Rectangle::new(3.0, 16.0));
    let major_material = materials.add(ColorMaterial::from_color(Color::srgb(0.95, 0.45, 0.45)));
    let minor_material = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));

    for idx in 0..DIAL_POSITIONS {
        let angle = idx as f32 / DIAL_POSITIONS as f32 * TAU;
        let radius = DIAL_RADIUS + 6.0;
        let position = Vec3::new(angle.sin() * radius, angle.cos() * radius, 0.5);
        let (mesh, material) = if idx % 10 == 0 {
            (major_tick_mesh.clone(), major_material.clone())
        } else {
            (minor_tick_mesh.clone(), minor_material.clone())
        };

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(position).with_rotation(Quat::from_rotation_z(-angle)),
        ));
    }

    let pointer_mesh = meshes.add(Rectangle::new(10.0, DIAL_RADIUS - 40.0));
    let pointer_material = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.95)));

    let pointer_root = commands
        .spawn((
            PointerRoot,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    commands.entity(pointer_root).with_children(|parent| {
        parent.spawn((
            Mesh2d(pointer_mesh),
            MeshMaterial2d(pointer_material),
            Transform::from_translation(Vec3::new(0.0, (DIAL_RADIUS - 40.0) / 2.0, 2.0)),
        ));
    });
}

fn load_default_input(mut sim: ResMut<DialSimulation>, mut ui: ResMut<UiState>) {
    if let Err(err) = try_load_default(&mut sim) {
        ui.last_error = Some(err);
    } else {
        ui.last_error = None;
    }
}

fn try_load_default(sim: &mut DialSimulation) -> Result<(), String> {
    let input = std::panic::catch_unwind(|| read_input(1))
        .map_err(|_| "Failed to read Input/day01.txt".to_string())?;
    let steps = parse_steps(&input)?;
    sim.reset_with_steps(steps, Some(PathBuf::from("Input/day01.txt")));
    Ok(())
}

fn run_simulation(
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

fn update_pointer_visual(
    sim: Res<DialSimulation>,
    mut pointer: Single<&mut Transform, With<PointerRoot>>,
) {
    if !sim.is_changed() {
        return;
    }
    let angle = FRAC_PI_2 - sim.angle();
    (*pointer).rotation = Quat::from_rotation_z(angle);
}

fn ui_system(
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

fn apply_loaded_input(
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

#[derive(Clone)]
struct Step {
    label: String,
    direction: i32,
    magnitude: i32,
}

fn parse_steps(input: &str) -> Result<Vec<Step>, String> {
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
