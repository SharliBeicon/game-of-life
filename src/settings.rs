use crate::components::*;
use bevy::prelude::*;

pub fn settings_menu_setup(mut commands: Commands, grid_size: Res<GridSize>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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
            OnSettingsMenuScreen,
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb_u8(55, 183, 195)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section("Grid Size", button_text_style.clone())
                                    .with_style(Style {
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(50.0)),
                                        ..default()
                                    }),
                            );
                            for grid_size_setting in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: BackgroundColor(*NORMAL_BUTTON),
                                        ..default()
                                    },
                                    GridSize(grid_size_setting),
                                ));
                                if *grid_size == GridSize(grid_size_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: BackgroundColor(*NORMAL_BUTTON),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}
