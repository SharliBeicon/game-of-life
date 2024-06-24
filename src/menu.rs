use bevy::{app::AppExit, prelude::*};

use crate::{components::*, settings::*, simulation::SPRITE_SIZE};

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        .add_systems(
            Update,
            setting_button::<GridSize>.run_if(in_state(MenuState::Settings)),
        )
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = BackgroundColor(*NORMAL_BUTTON);
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                BackgroundColor(*PRESSED_BUTTON)
            }
            (Interaction::Hovered, Some(_)) => BackgroundColor(*HOVERED_PRESSED_BUTTON),
            (Interaction::Hovered, None) => BackgroundColor(*HOVERED_BUTTON),
            (Interaction::None, None) => BackgroundColor(*NORMAL_BUTTON),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: *TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb_u8(55, 183, 195)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Of Life",
                            TextStyle {
                                font_size: 80.0,
                                color: *TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor(*NORMAL_BUTTON),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor(*NORMAL_BUTTON),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: BackgroundColor(*NORMAL_BUTTON),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    grid_size: Res<GridSize>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    center_camera(&mut camera_query, &grid_size);
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}

fn center_camera(
    camera_query: &mut Query<&mut Transform, With<Camera>>,
    grid_size: &Res<GridSize>,
) {
    let specified_size = Vec2::new(
        grid_size.0 as f32 * SPRITE_SIZE,
        grid_size.0 as f32 * SPRITE_SIZE,
    );
    let center_position = Vec3::new(specified_size.x / 2.0, specified_size.y / 2.0, 0.0);

    for mut transform in camera_query.iter_mut() {
        transform.translation = center_position;
    }
}
