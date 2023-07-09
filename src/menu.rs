use std::time::Duration;

use bevy::{prelude::*, winit::WinitSettings};
use crate::GameState;
use super::bee_game::{AnimInfo, BeeFly};


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
            .add_system(button_system.in_set(OnUpdate(GameState::Menu)))
            .add_system(flying_bee.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup.in_schedule(OnExit(GameState::Menu)));
    }
}

#[derive(Component)]
struct MenuMarker;

#[derive(Component)]
enum ButtonIdent {
    Play,
    Exit,
}

fn menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let _ = asset_server.load::<Image, &str>("textures/bg2.png");
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::rgb(1.0, 0.95, 0.84),
                ),
            },
            ..default()
        },
        MenuMarker,
    ));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MenuMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(75.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonIdent::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/goodtimes.otf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 0.08, 0.20),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(75.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect {
                                left: Val::Percent(5.),
                                right: Val::Percent(5.),
                                top: Val::Percent(5.),
                                bottom: Val::Percent(5.),
                            },
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonIdent::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/goodtimes.otf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 0.08, 0.20),
                        },
                    ));
                });
        });

        let center = Vec2::new(0.0, 0.0);
        let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("textures/bee.png"),
            Vec2::new(32.0, 32.0),
            2,
            1,
            None,
            None,
        ));
    
        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                    ..Default::default()
                },
                transform: Transform::from_xyz(center.x, center.y, 500.0),
                texture_atlas:texture_atlas.clone(),
                ..Default::default()
            },
            BeeFly {
                aim: center,
                center,
                width: 900.0,
                height: 900.0,
                timer: Timer::new(Duration::from_millis(2000), TimerMode::Repeating),
            },
            AnimInfo {
                timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                num: 2,
            },
            
            MenuMarker,
        ));

        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                    ..Default::default()
                },
                transform: Transform::from_xyz(center.x-400.0, center.y-250.0, 500.0),
                texture_atlas:texture_atlas.clone(),
                ..Default::default()
            },
            BeeFly {
                aim: center,
                center,
                width: 900.0,
                height: 900.0,
                timer: Timer::new(Duration::from_millis(2000), TimerMode::Repeating),
            },
            AnimInfo {
                timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                num: 2,
            },
            
            MenuMarker,
        ));


}

const NORMAL_BUTTON: Color = Color::rgb(1.0, 0.92, 0.80);
const HOVERED_BUTTON: Color = Color::rgb(1.0, 0.94, 0.57);
const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.84, 0.48);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &ButtonIdent),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, mut color, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Bzzzz".to_string();
                *color = PRESSED_BUTTON.into();
                use ButtonIdent::*;
                match button {
                    Play => game_state.set(GameState::Game),
                    Exit => exit.send(bevy::app::AppExit),
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Bzzzz".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<MenuMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn flying_bee(){
    
}
