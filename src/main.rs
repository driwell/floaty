use bevy::prelude::*;

mod camera;
mod shape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(shape::Shape)
        .add_plugins(camera::Camera)
        .run();
}
