use bevy::{
    math::primitives::{Circle, Rectangle},
    prelude::*,
};
use std::f32::consts::{FRAC_PI_2, TAU};

use super::constants::{DIAL_POSITIONS, DIAL_RADIUS};
use super::types::{
    BackgroundSprite, DialElement, DialSimulation, PointerRoot, TickElement, WindowSizeTracker,
};

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    // Load and display the background image
    let texture = asset_server.load("day01_gemini.png");
    
    // Get window dimensions to scale the background
    if let Some(window) = windows.iter().next() {
        let window_width = window.width();
        let window_height = window.height();
        
        commands.spawn((
            BackgroundSprite,
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(window_width, window_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        ));
    }
}

const IMAGE_ASPECT_RATIO: f32 = 2816.0 / 1536.0;

pub fn maintain_aspect_ratio(
    mut resize_events: bevy::prelude::MessageReader<bevy::window::WindowResized>,
    mut windows: Query<&mut Window>,
) {
    for event in resize_events.read() {
        if let Ok(mut window) = windows.get_mut(event.window) {
            let current_width = event.width;
            let current_height = event.height;
            let current_ratio = current_width / current_height;
            
            // Only adjust if the ratio is significantly different
            if (current_ratio - IMAGE_ASPECT_RATIO).abs() > 0.01 {
                // Calculate new dimensions to maintain aspect ratio
                let new_height = current_width / IMAGE_ASPECT_RATIO;
                window.resolution.set(current_width, new_height);
            }
        }
    }
}

const BASE_HEIGHT: f32 = 768.0;

pub fn update_background_on_resize(
    mut resize_events: bevy::prelude::MessageReader<bevy::window::WindowResized>,
    mut background: Query<&mut Sprite, With<BackgroundSprite>>,
) {
    for event in resize_events.read() {
        if let Some(mut sprite) = background.iter_mut().next() {
            sprite.custom_size = Some(Vec2::new(event.width, event.height));
        }
    }
}

pub fn scale_dial_elements(
    mut resize_events: bevy::prelude::MessageReader<bevy::window::WindowResized>,
    mut dial_elements: Query<(Entity, &mut Transform), (With<DialElement>, Without<PointerRoot>)>,
    mut pointer_root: Query<&mut Transform, With<PointerRoot>>,
    ticks: Query<&TickElement>,
    mut size_tracker: ResMut<WindowSizeTracker>,
) {
    for event in resize_events.read() {
        let current_height = event.height;
        
        // Only update if the window size has actually changed significantly
        if (current_height - size_tracker.last_height).abs() < 1.0 {
            continue;
        }
        
        size_tracker.last_height = current_height;
        let scale_factor = current_height / BASE_HEIGHT;
        
        // Scale regular dial elements (rings and ticks)
        for (entity, mut transform) in dial_elements.iter_mut() {
            // Check if this is a tick element
            if let Ok(tick) = ticks.get(entity) {
                // Update tick position and scale - always from base values
                let radius = (DIAL_RADIUS + 6.0) * scale_factor;
                let position = Vec3::new(
                    tick.angle.sin() * radius,
                    tick.angle.cos() * radius,
                    0.5
                );
                *transform = Transform::from_translation(position)
                    .with_rotation(Quat::from_rotation_z(-tick.angle))
                    .with_scale(Vec3::splat(scale_factor));
            } else {
                // For dial rings, just scale from base (1.0)
                transform.scale = Vec3::splat(scale_factor);
            }
        }
        
        // Scale pointer root (which has children with visuals)
        if let Some(mut transform) = pointer_root.iter_mut().next() {
            transform.scale = Vec3::splat(scale_factor);
        }
    }
}

pub fn setup_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    //Dial
    let outer_ring = meshes.add(Circle::new(DIAL_RADIUS + 28.0));
    let inner_fill = meshes.add(Circle::new(DIAL_RADIUS - 18.0));
    let outer_material = materials.add(ColorMaterial::from_color(Color::srgb(0.08, 0.08, 0.12)));
    let inner_material = materials.add(ColorMaterial::from_color(Color::srgb(0.02, 0.02, 0.05)));

    commands.spawn((
        DialElement,
        Mesh2d(outer_ring),
        MeshMaterial2d(outer_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
    ));
    commands.spawn((
        DialElement,
        Mesh2d(inner_fill),
        MeshMaterial2d(inner_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.25)),
    ));

    //Ticks
    let major_tick_mesh = meshes.add(Rectangle::new(6.0, 28.0));
    let minor_tick_mesh = meshes.add(Rectangle::new(3.0, 16.0));
    let major_material = materials.add(ColorMaterial::from_color(Color::srgb(0.95, 0.45, 0.45)));
    let minor_material = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));

    for idx in 0..DIAL_POSITIONS {
        let angle = idx as f32 / DIAL_POSITIONS as f32 * TAU;
        let is_major = idx % 10 == 0;
        let (mesh, material) = if is_major {
            (major_tick_mesh.clone(), major_material.clone())
        } else {
            (minor_tick_mesh.clone(), minor_material.clone())
        };

        commands.spawn((
            DialElement,
            TickElement { angle },
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::default(),
        ));
    }

    //Pointer
    let pointer_mesh = meshes.add(Rectangle::new(10.0, DIAL_RADIUS - 40.0));
    let pointer_material = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.95)));

    let pointer_root = commands
        .spawn((
            DialElement,
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

pub fn update_pointer_visual(
    sim: Res<DialSimulation>,
    mut pointer: Single<&mut Transform, With<PointerRoot>>,
) {
    if !sim.is_changed() {
        return;
    }
    let angle = FRAC_PI_2 - sim.angle();
    (*pointer).rotation = Quat::from_rotation_z(angle);
}

