mod backtrack;
use crate::HexMaze;
use backtrack::generate_backtracking;
#[cfg(feature = "bevy_reflect")]
use bevy::prelude::*;
use hexx::Hex;

#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GeneratorType {
    #[default]
    RecursiveBacktracking,
}
impl GeneratorType {
    pub fn generate(&self, maze: &mut HexMaze, start_pos: Option<Hex>, seed: Option<u64>) {
        match self {
            Self::RecursiveBacktracking => generate_backtracking(maze, start_pos, seed),
        }
    }
}
