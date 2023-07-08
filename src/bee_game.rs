use std::time::Duration;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
//use bevy_kira_audio::AudioPlugin;

pub struct BeeGame;

impl Plugin for BeeGame {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(clouds_move)
            .add_system(pillar_spawner)
            .add_system(pillar_move)
            .add_system(jump_input)
            .add_system(let_it_jump)
            .add_system(bee_fly)
            .add_system(text_write)
            .add_system(text_update)
            .add_system(anim_handler)
            .add_startup_system(audio_setup);
        
    }
}

#[derive(Resource)]
struct PillarShared {
    x_vel: f32,
    y_pos: f32,
    y_vel: f32,
    x_pos_bounds: (f32, f32),
    y_pos_bounds: (f32, f32),
    spawn_timer: Timer,
    texture: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, wins: Query<&Window>,mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::rgb(0.35, 0.8, 1.0),
                ),
            },
            ..default()
        },
        BloomSettings {
            ..Default::default()
        },
    ));

    setup_pillars(&mut commands, &asset_server, &wins);

    setup_bee(&mut commands, &asset_server, &mut texture_atlases);

    setup_clouds(&mut commands, &asset_server, &wins);
}

#[derive(Component)]
struct Pillar {
    y_offset: f32,
}

fn setup_pillars(commands: &mut Commands, asset_server: &Res<AssetServer>, wins: &Query<&Window>) {
    let window = wins.single();

    let mut timer = Timer::new(Duration::from_millis(2500), TimerMode::Repeating);
    timer.set_elapsed(Duration::MAX);
    let pillar_shared = PillarShared {
        x_vel: 150.0,
        y_pos: 0.0,
        y_vel: 0.0,
        x_pos_bounds: (-window.width() / 2.0 - 100.0, window.width() / 2.0 + 100.0),
        y_pos_bounds: (-200.0, 200.0),
        spawn_timer: timer,
        texture: asset_server.load("textures/pipe.png"),
    };

    commands.insert_resource(pillar_shared);
}

#[derive(Component)]
struct BeeFly {
    aim: Vec2,
    center: Vec2,
    width: f32,
    height: f32,
    timer: Timer,
}

#[derive(Component)]
struct AnimInfo {
    timer: Timer,
    num: usize,
}

fn setup_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let center = Vec2::new(350.0, 0.0);
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
            transform: Transform::from_xyz(center.x, center.y, 100.0),
            texture_atlas,
            ..Default::default()
        },
        BeeFly {
            aim: center,
            center,
            width: 100.0,
            height: 220.0,
            timer: Timer::new(Duration::from_millis(4000), TimerMode::Repeating),
        },
        AnimInfo {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            num: 2,
        }
    ));
}

#[derive(Component)]
struct Cloud {
    vel: f32,
    respawn_bounds: (f32, f32),
}

fn setup_clouds(commands: &mut Commands, asset_server: &Res<AssetServer>, wins: &Query<&Window>) {
    let window = wins.single();
    for _ in 0..10 {
        let respawn_bounds = (-window.width() / 2.0 - 200.0, window.width() / 2.0 + 200.0);
        let x = rand::random::<f32>() * (respawn_bounds.1 - respawn_bounds.0) + (respawn_bounds.0);
        let y = window.height() * (rand::random::<f32>() - 0.5);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: 120.0,
                        y: 120.0 * 0.695,
                    }),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 50.0),
                texture: asset_server.load("textures/cloud1.png"),
                ..Default::default()
            },
            Cloud {
                vel: rand::random::<f32>() * 1.0 + 0.5,
                respawn_bounds,
            },
        ));
    }
}

fn clouds_move(mut query: Query<(&mut Transform, &Cloud)>) {
    for (mut t, c) in query.iter_mut() {
        t.translation.x -= c.vel;
        if t.translation.x < c.respawn_bounds.0 {
            t.translation.x = c.respawn_bounds.1;
        }
    }
}

fn pillar_move(mut commands:Commands, mut query: Query<(&mut Transform, Entity), With<Pillar>>, pillar_shared: Res<PillarShared>, time: Res<Time>) {
    for (mut t, e) in query.iter_mut() {
        t.translation.x += pillar_shared.x_vel * time.delta_seconds();
        if t.translation.x > pillar_shared.x_pos_bounds.1{
            commands.entity(e).despawn();
        }
    }

}

fn pillar_spawner(
    mut commands: Commands,
    mut pillar_shared: ResMut<PillarShared>,
    time: Res<Time>,
) {
    pillar_shared.spawn_timer.tick(time.delta());

    if pillar_shared.spawn_timer.just_finished() {
        spawn_piller(&mut commands, &pillar_shared);
    }
}

fn spawn_piller(commands: &mut Commands, pillar_shared: &ResMut<PillarShared>) {
    let y_offset = (rand::random::<f32>() - 0.5) * 200.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: None,
                ..Default::default()
            },
            transform: Transform::from_xyz(pillar_shared.x_pos_bounds.0, 0.0, 80.0)
                .with_scale(Vec3::ONE * 2.5),
            texture: pillar_shared.texture.clone(),
            ..Default::default()
        },
        Pillar { y_offset },
    ));
}

fn jump_input(keys: Res<Input<KeyCode>>, mut pillar_shared: ResMut<PillarShared>, time: Res<Time>, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    if keys.just_pressed(KeyCode::Space) {
        pillar_shared.y_vel = 4.0;
        audio.play(asset_server.load("sounds/beep.wav"));
    }

    pillar_shared.y_vel -= 10.0 * time.delta_seconds();
}

fn let_it_jump(
    mut pillar_shared: ResMut<PillarShared>,
    mut query: Query<(&mut Transform, &Pillar)>,
) {
    pillar_shared.y_pos += pillar_shared.y_vel;
    pillar_shared.y_pos = pillar_shared
        .y_pos
        .clamp(pillar_shared.y_pos_bounds.0, pillar_shared.y_pos_bounds.1);
    for (mut t, p) in query.iter_mut() {
        t.translation.y = pillar_shared.y_pos + p.y_offset;
    }
}

fn bee_fly(mut query: Query<(&mut Transform, &mut BeeFly)>, time: Res<Time>) {
    for (mut t, mut b) in query.iter_mut() {
        b.timer.tick(time.delta());
        if b.timer.just_finished() {
            b.aim = Vec2::new(
                (rand::random::<f32>() - 0.5) * b.width + b.center.x,
                (rand::random::<f32>() - 0.5) * b.height + b.center.y,
            );
        }

        let p = Vec2::new(t.translation.x, t.translation.y);

        let p = p.lerp(b.aim, 0.8 * time.delta_seconds());

        t.translation.x = p.x;
        t.translation.y = p.y;
    }
}

#[derive(Component)]
struct ColorText;

fn text_write(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "BEE",
            TextStyle {
                font: asset_server.load("fonts/goodtimes.otf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));
}

fn text_update(mut commands: Commands) {
    commands.spawn(());
}

fn anim_handler(mut query: Query<(&mut TextureAtlasSprite, &mut AnimInfo)>, time: Res<Time>) {
    for (mut tas, mut ai) in query.iter_mut() {
        ai.timer.tick(time.delta());

        if ai.timer.just_finished() {
            tas.index = (tas.index + 1) % ai.num;
        }
    }
}

fn audio_setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/farm.mp3"));
}
