use bevy::{
    math::primitives::{Circle, Rectangle},
    prelude::*,
};
use std::f32::consts::{FRAC_PI_2, TAU};

use super::constants::{DIAL_POSITIONS, DIAL_RADIUS};
use super::types::{DialSimulation, PointerRoot};

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
        Mesh2d(outer_ring),
        MeshMaterial2d(outer_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
    ));
    commands.spawn((
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

    //Pointer

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

