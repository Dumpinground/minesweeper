pub mod components;
pub mod resources;

use bevy::prelude::*;
use components::coordinates::Coordinates;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)
        .register_type::<Coordinates>();
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board() {
        
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
