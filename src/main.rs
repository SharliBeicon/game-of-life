use bevy::prelude::*;
use game_of_life::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280., 960.).into(),
                title: "Game of Life".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(components::GridSize(50))
        .init_state::<components::GameState>()
        .add_systems(Startup, setup)
        .add_plugins((menu::menu_plugin, simulation::simulation_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
