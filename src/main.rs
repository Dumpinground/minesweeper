use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::{resources::BoardOptions, BoardPlugin};

fn main() {
    let mut app = App::new();
    // Window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper!".into(),
            resolution: WindowResolution::new(700., 800.),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    // Board Plugin Options
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..default()
    });

    app.add_plugin(BoardPlugin);
    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
