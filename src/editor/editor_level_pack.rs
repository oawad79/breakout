use std::{fs, io::Write};

use crate::game::{level_pack::{LevelPack, MAX_LEVELS}, world::level::{Level, LEVEL_NAME_LEN}};

use super::timewarp::Timewarp;

struct EditorLevel {
    level: Level,
    timewarp: Timewarp,
}

impl EditorLevel {
    pub fn new() -> Self {
        let level = Level::new();
        Self { timewarp: Timewarp::new(&level), level }
    }
    pub fn undo(&mut self) {
        self.timewarp.undo(&mut self.level)
    }
    pub fn redo(&mut self) {
        self.timewarp.redo(&mut self.level)
    }
    pub fn save_previous_state(&mut self) {
        self.timewarp.save_previous_state(&self.level)
    }
    pub fn push_current_state(&mut self) {
        self.timewarp.push_current_state()
    }
}

impl From<Level> for EditorLevel {
    fn from(value: Level) -> Self {
        Self { timewarp: Timewarp::new(&value), level: value }
    }
}

pub struct EditorLevelPack {
    levels: Vec<EditorLevel>,
    current: usize,
    name: String,
    author: String,
}

impl EditorLevelPack {
    pub fn new() -> Self {
        Self {
            levels: vec![EditorLevel::new()],
            current: 0,
            name: String::new(),
            author: String::new(),
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }
    pub fn level(&self) -> &Level {
        &self.levels[self.current].level
    }
    pub fn level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.current].level
    }
    pub fn timewarp(&self) -> &Timewarp {
        &self.levels[self.current].timewarp
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn author(&self) -> &String {
        &self.author
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn author_mut(&mut self) -> &mut String {
        &mut self.author
    }

    // Not sure about this....? but it also sort of makes sense
    pub fn timewarp_undo(&mut self) {
        self.levels[self.current].undo();
    }
    pub fn timewarp_redo(&mut self) {
        self.levels[self.current].redo();
    }
    pub fn timewarp_save_previous_state(&mut self) {
        self.levels[self.current].save_previous_state();
    }
    pub fn timewarp_push_current_state(&mut self) {
        self.levels[self.current].push_current_state();
    }

    pub fn can_add(&self) -> bool {
        self.levels.len() < MAX_LEVELS
    }
    pub fn can_next(&self) -> bool {
        self.current < self.levels.len()-1
    }
    pub fn can_prev(&self) -> bool {
        self.current != 0
    }
    pub fn can_shift_next(&self) -> bool {
        self.current < self.levels.len()-1
    }
    pub fn can_shift_prev(&self) -> bool {
        self.current != 0
    }

    pub fn add_level(&mut self) {
        if !self.can_add() {
            return;
        }
        self.current += 1;
        self.levels.insert(self.current, EditorLevel::new());
    }

    pub fn next(&mut self) {
        if self.can_next() {
            self.current += 1
        }
    }
    pub fn prev(&mut self) {
        if self.can_prev() {
            self.current -= 1
        }
    }

    pub fn shift_next(&mut self) {
        if self.can_shift_next() {
            self.levels.swap(self.current, self.current + 1);
            self.current += 1;
        }
    }
    pub fn shift_prev(&mut self) {
        if self.can_shift_prev() {
            self.levels.swap(self.current, self.current - 1);
            self.current -= 1;
        }
    }

    pub fn delete_level(&mut self) {
        if self.levels.len() == 1 {
            self.levels.clear();
            self.levels.push(EditorLevel::new());
            return;
        }
        self.levels.remove(self.current);
        self.current = self.current.clamp(0, self.levels.len()-1);
    }

    pub fn save(&self) {
        let bytes = self.encode_to_file();
        if cfg!(target_family = "wasm") {
            save_wasm(bytes, self.name());
        } else {
            save_desktop(bytes, self.name());
        }
    }

    pub fn encode_to_file(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        // Turns a string into a bunch of bytes 'LEVEL_NAME_LEN' long.
        // If the name is shorter than LEVEL_NAME_LEN, 0xFF will fill the rest of the space. (0xFF doesn't correspond to any allowed characters) 
        let push_string_bytes = |data: &mut Vec<u8>, string: &String| {
            let bytes = string.as_bytes();
            for i in 0..LEVEL_NAME_LEN {
                data.push(*bytes.get(i).unwrap_or(&0xFF));
            }
        };

        // The file begins with the pack name and author
        push_string_bytes(&mut data, self.name());
        push_string_bytes(&mut data, self.author());

        // After that it has the contents of each level
        for level in self.levels.iter().map(|el| &el.level) {
            // First we add the name...
            push_string_bytes(&mut data, level.name());
            // And then the tiles!
            // Each set of two tiles are encoded as a single byte.
            // Since there are only 15 tiles two tiles are guaranteed to fit in a byte.
            for t in level.tiles().chunks_exact(2) {
                let (a, b) = (t[0] as u8, t[1] as u8);
                data.push((a << 4) + b);
            }
        }

        data
    }
}

impl From<LevelPack> for EditorLevelPack {
    fn from(value: LevelPack) -> Self {
        let mut levels: Vec<EditorLevel> = value.levels()
            .clone()
            .iter()
            .map(|l| l.clone().into())
            .collect();

        if levels.is_empty() {
            levels.push(EditorLevel::new());
        }

        EditorLevelPack {
            levels,
            current: 0,
            name: value.name().clone(),
            author: value.author().clone()
        }
    }
}

fn save_desktop(bytes: Vec<u8>, name: &String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.brk", name))
        .unwrap();
    file.write_all(&bytes).unwrap();
}

fn save_wasm(_bytes: Vec<u8>, _name: &String) {

}