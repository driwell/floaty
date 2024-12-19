use bevy::prelude::*;

pub struct Shape;

impl Plugin for Shape {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(50.0, 50.0));
    let color = Color::srgb(255., 0., 0.);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
