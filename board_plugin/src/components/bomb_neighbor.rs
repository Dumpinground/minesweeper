use bevy::{prelude::Component, reflect::Reflect};

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

/// Bomb neighbor component
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct BombNeighbor {
    /// Number
    pub count: u8,
}