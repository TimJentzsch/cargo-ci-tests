use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}
