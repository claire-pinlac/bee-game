use bevy::prelude::*;
//crate::menu::MenuPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(menu_setup);
    }
}

fn menu_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(
                1.0, 0.94, 0.57,
            )),
        },
        ..default()
    },));
}

