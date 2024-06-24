use crate::components::*;
use bevy::prelude::*;
use rand::prelude::*;

pub const SPRITE_SIZE: f32 = 16.0;

pub fn simulation_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), simulation_setup)
        .add_systems(Update, simulation.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

fn simulation_setup(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    asset_server: Res<AssetServer>,
) {
    for i in 0..grid_size.0 {
        for j in 0..grid_size.0 {
            let (texture, state) = cell_setup(&asset_server);
            commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            (i as f32) * SPRITE_SIZE,
                            (j as f32) * SPRITE_SIZE,
                            0.0,
                        ),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                    sprite: Sprite { ..default() },
                    texture,
                    ..default()
                })
                .insert(state);
        }
    }

    commands.insert_resource(SpriteImages {
        empty_cell: asset_server.load("Gray.png"),
        alive_cell: asset_server.load("Green.png"),
        dead_cell: asset_server.load("Brown.png"),
    })
}

fn cell_setup(asset_server: &Res<AssetServer>) -> (Handle<Image>, Cell) {
    let mut rng = rand::thread_rng();
    let n: f64 = rng.gen();

    if n >= 0.66 {
        return (
            asset_server.load("Green.png"),
            Cell {
                state: CellState::Alive,
            },
        );
    }

    if n <= 0.33 {
        return (
            asset_server.load("Brown.png"),
            Cell {
                state: CellState::Dead,
            },
        );
    }

    (
        asset_server.load("Gray.png"),
        Cell {
            state: CellState::Empty,
        },
    )
}

fn simulation(
    mut cells: Query<(&mut Cell, &mut Handle<Image>)>,
    grid_size: Res<GridSize>,
    sprite_images: Res<SpriteImages>,
) {
    let mut alive_cells: Vec<bool> = Vec::new();
    for (cell, _) in cells.iter() {
        alive_cells.push(match cell.state {
            CellState::Alive => true,
            CellState::Dead | CellState::Empty => false,
        })
    }
    for (idx, (mut cell, mut sprite)) in cells.iter_mut().enumerate() {
        let mut neighbour_cnt = 0;
        let i = idx as i32 % grid_size.0 as i32;
        let j = idx as i32 / grid_size.0 as i32;

        for ix in (i - 1)..(i + 2) {
            for jy in (j - 1)..(j + 2) {
                if (ix != i || jy != j)
                    && ix >= 0
                    && ix < grid_size.0 as i32
                    && jy >= 0
                    && jy < grid_size.0 as i32
                {
                    let lin_ind = ix + jy * grid_size.0 as i32;
                    if alive_cells[lin_ind as usize] {
                        neighbour_cnt += 1;
                    }
                }
            }
        }

        if neighbour_cnt < 2 || neighbour_cnt > 3 {
            match cell.state {
                CellState::Alive => {
                    cell.state = CellState::Dead;
                    *sprite = sprite_images.dead_cell.clone();
                }
                CellState::Dead | CellState::Empty => {}
            }
        }

        if neighbour_cnt == 3 {
            cell.state = CellState::Alive;
            *sprite = sprite_images.alive_cell.clone();
        }
    }
}
