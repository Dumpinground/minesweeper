pub mod components;
pub mod resources;

use bevy::{log, prelude::*, utils::HashMap};
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
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        // TODO refactor this (This will move into a resource in a following chapter)
        let mut covered_tiled =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let mut safe_start = None;
        let board_entity = commands
            .spawn(GlobalTransform::default())
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(board_size),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Name::new("Background"));
                // Self::spawn
            });

        commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position),
                ..default()
            })
            .insert(Name::new("Board"));
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: Res,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
    }

    fn adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
