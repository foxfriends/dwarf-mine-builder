use game_engine::prelude::*;

mod cube_info;
mod cube_position;
mod dwarf_info;
mod grid_position;
mod grid_size;
mod target_cube;
mod target_position;
mod task;

pub use self::{
    cube_info::CubeInfo,
    cube_position::CubePosition,
    dwarf_info::DwarfInfo,
    grid_position::GridPosition,
    grid_size::GridSize,
    target_cube::TargetCube,
    target_position::TargetPosition,
    task::Task,
};

crate fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<CubeInfo>()
        .register_component::<CubePosition>()
        .register_component::<DwarfInfo>()
        .register_component::<GridPosition>()
        .register_component::<GridSize>()
        .register_component::<TargetCube>()
        .register_component::<TargetPosition>()
        .register_component::<Task>()
}
