pub mod components;
pub mod resources;

use bevy::{log, prelude::*};
use components::*;
use resources::{tile_map::TileMap, BoardOptions, BoardPosition, TileSize, tile::Tile};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);

        #[cfg(feature = "debug")]
        app.register_type::<Coordinates>()
        .register_type::<BombNeighbor>()
        .register_type::<Bomb>()
        .register_type::<Uncover>();

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&mut Window>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>
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
        let board_material = materials.add(Color::WHITE.into());
        let tile_material = materials.add(Color::GRAY.into());
        let font = asset_server.load("fonts/minecraft.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");

        commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position),
                ..default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(board_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..default()
                }).insert(Name::new("Background"));

                Self::spawn_tiles(parent, &tile_map, tile_size, options.tile_padding, tile_material, bomb_image, font)
            });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        material: Handle<ColorMaterial>,
        bomb_image: Handle<Image>,
        font: Handle<Font>
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let mut commands = parent.spawn_empty();
                commands.insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..default()
                    },
                    transform: Transform::from_xyz((x as f32 * size) + (size / 2.), (y as f32 * size) + (size / 2.), 1.),
                    ..default()
                }).insert(Name::new(format!("Tile ({}, {})", x, y))).insert(Coordinates {
                    x: x as u16, y: y as u16
                });

                match tile {
                    // If the tile is a bomb we add the matching component and a sprite child
                    Tile::Bomb => {
                        commands.insert(Bomb).with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..default()
                            });
                        });
                    }
                    // If the tile is a bomb neighbor we add the matching component and a text child
                    Tile::BombNeighbor(v) => {
                        commands.insert(BombNeighbor { count: *v }).with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(*v, font.clone(), size - padding));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            }
        );

        Text2dBundle {
            text: Text { sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    color, font, font_size: size
                },
            }], alignment: TextAlignment::Center, ..default() },
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        }
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
