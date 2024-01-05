use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod systems;
mod utils;

use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_sets(Update, (MoveSystems, ProcessSystems, Display))
        /////////
        // Setup
        .add_systems(Startup, setup)
        // Player Move
        .add_systems(Update, move_good_level_wall)
        // Set commands
        .add_systems(
            Update,
            funnel_move.in_set(MoveSystems).after(move_good_level_wall),
        )
        // Process commands
        .add_systems(
            Update,
            (process_move, undo)
                .in_set(ProcessSystems)
                .after(MoveSystems),
        )
        // Display
        .add_systems(
            Update,
            update_display_position
                .in_set(Display)
                .after(ProcessSystems),
        )
        /////////
        .insert_resource(MoveCommands { commands: vec![] })
        .insert_resource(MoveHistory { moves: vec![] })
        .run();
}
