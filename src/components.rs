use bevy::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEXT_COLOR: Color = Color::rgb_u8(235, 244, 246);
    pub static ref NORMAL_BUTTON: Color = Color::rgb_u8(8, 131, 149);
    pub static ref HOVERED_BUTTON: Color = Color::rgb_u8(7, 25, 82);
    pub static ref HOVERED_PRESSED_BUTTON: Color = Color::rgb_u8(7, 25, 82);
    pub static ref PRESSED_BUTTON: Color = Color::rgb_u8(24, 1, 121);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GridSize(pub u32);

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
}

pub enum CellState {
    Alive,
    Dead,
    Empty,
}

#[derive(Resource, Default)]
pub struct SpriteImages {
    pub empty_cell: Handle<Image>,
    pub alive_cell: Handle<Image>,
    pub dead_cell: Handle<Image>,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
