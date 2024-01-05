use bevy::prelude::*;

use crate::components::*;

/// RESOURCES UTILS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveCause {
    UserMove,
    FunnelMove(PositionTuple),
}

pub struct MoveCommand {
    pub entities: Vec<(Entity, PositionTuple)>,
    pub cause: MoveCause,
    pub delta: (i16, i16),
}

impl MoveCommand {
    pub fn keep_still_valid(&mut self) {
        match self.cause {
            MoveCause::UserMove => {}
            MoveCause::FunnelMove(funnel_pos) => {
                self.entities.retain(|(_, pos)| *pos == funnel_pos);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActualMove {
    pub entities: Vec<Entity>,
    pub cause: MoveCause,
    pub delta: (i16, i16),
}

impl ActualMove {
    pub fn undo_delta(&self) -> (i16, i16) {
        (-self.delta.0, -self.delta.1)
    }
}

/// RESOURCES

#[derive(Resource)]
pub struct MoveCommands {
    pub commands: Vec<MoveCommand>,
}

#[derive(Resource)]
pub struct MoveHistory {
    pub moves: Vec<Vec<ActualMove>>, // Vec<AcutalMove> is all the moves in a single frame
}

impl MoveHistory {
    pub fn last_actual_move(&self) -> Option<&ActualMove> {
        self.moves
            .last()
            .and_then(|last_game_loop_move| last_game_loop_move.last())
    }

    pub fn last_move_cause(&self) -> Option<MoveCause> {
        self.last_actual_move()
            .map(|last_actual_move| last_actual_move.cause)
    }
}
