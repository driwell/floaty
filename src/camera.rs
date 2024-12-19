use bevy::prelude::*;

pub struct Camera;

impl Plugin for Camera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
