pub mod components;
pub mod resources;

use bevy::{log, prelude::*};
use components::coordinates::Coordinates;
use resources::{tile_map::TileMap, BoardOptions, BoardPosition, TileSize};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);

        #[cfg(feature = "debug")]
        app.register_type::<Coordinates>();

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        mut windows: Query<&mut Window>,
    ) {
        let window = windows.single();

        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        // Tilemap Generation
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        // Tilemap Debugging
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };
    }

    fn adaptative_tile_size(
        window: &Window,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
