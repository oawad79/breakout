use std::collections::VecDeque;

use crate::game::level::{Level, Tile, TileArray};

pub struct Timewarp {
    previous_state: TileArray,
    undo_states: VecDeque<TileArray>,
    redo_states: Vec<TileArray>,
}

impl Timewarp {
    pub fn new(previous_state: TileArray) -> Self {
        Self { previous_state, undo_states: VecDeque::with_capacity(50), redo_states: Vec::with_capacity(50) }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_states.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.redo_states.is_empty()
    }

    pub fn undo(&mut self, level: &mut Level) {
        if let Some(undo_state) = self.undo_states.pop_front() {
            self.redo_states.push(level.tiles().clone());
            *level.tiles_mut() = undo_state;
        }
    }
    pub fn redo(&mut self, level: &mut Level) {
        if let Some(redo_state) = self.redo_states.pop() {
            self.undo_states.push_front(level.tiles().clone());
            *level.tiles_mut() = redo_state;
        }
    }

    pub fn immediate_push(&mut self, level: &Level) {
        if self.previous_state == *level.tiles() {
            return;
        }
        self.previous_state = level.tiles().clone();
        self.undo_states.push_front(self.previous_state);
        self.redo_states.clear();
    }

    pub fn delayed_push_begin(&mut self, level: &Level) {
        if self.previous_state == *level.tiles() {
            return;
        }
        self.previous_state = level.tiles().clone();
    }
    pub fn delayed_push_end(&mut self, level: &Level) {
        if self.previous_state == *level.tiles() {
            return;
        }
        self.undo_states.push_front(self.previous_state);
        self.redo_states.clear();
    }
}