use crate::text_renderer::char_valid;

use super::world::level::{Level, Tile, LEVEL_HEIGHT, LEVEL_NAME_LEN, LEVEL_WIDTH};

pub const MAX_LEVELS: usize = 99;

#[derive(Clone)]
pub struct LevelPack {
    name: String,
    author: String,
    levels: Vec<Level>,
}

impl LevelPack {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn author(&self) -> &String {
        &self.author
    }
    pub fn levels(&self) -> &Vec<Level> {
        &self.levels
    }

    pub fn load_from_file(data: Vec<u8>) -> Option<Self> {
        // If the pack doesn't have enough bytes for the name and author, it's not valid
        if data.len() <= LEVEL_NAME_LEN * 2 {
            return None;
        }
        
        let load_string = |bytes: &[u8]| -> String {
            let mut string = String::new();
            for i in 0..LEVEL_NAME_LEN {
                let character = match bytes.get(i) {
                    Some(byte) if char_valid(*byte as char) => *byte as char,
                    _ => break,
                };
                string.push(character);
            }
            string
        };

        // The file begins with the name and author of the level pack
        let name = load_string(&data[0..LEVEL_NAME_LEN]);
        let author = load_string(&data[LEVEL_NAME_LEN..LEVEL_NAME_LEN*2]);

        // Then it holds all of the levels
        let mut levels = Vec::new(); 
        for level_bytes in data[LEVEL_NAME_LEN*2..].chunks_exact(LEVEL_NAME_LEN + (LEVEL_WIDTH * LEVEL_HEIGHT)/2) {
            let mut level = Level::new();
            *level.name_mut() = load_string(&level_bytes[0..LEVEL_NAME_LEN]);
            
            for (i, tiles_byte) in level_bytes[LEVEL_NAME_LEN..].iter().enumerate() {
                let (a, b) = (tiles_byte >> 4, tiles_byte & 0b1111);
                
                for (index, nibble) in [(i*2, a), (i*2+1, b)] {
                    level.tiles_mut().get_mut(index).map(|t| *t = Tile::try_from(nibble).unwrap_or(Tile::Air));
                }
            }

            levels.push(level);
        }
        
        // If the pack has no levels, or too many levels, it's not valid!!
        if levels.is_empty() || levels.len() > MAX_LEVELS {
            return None;
        }

        Some(LevelPack { name, author, levels })
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_specific::*;
#[cfg(target_arch = "wasm32")]
mod wasm_specific {
    use std::sync::Mutex;
    use sapp_jsutils::JsObject;
    use crate::LevelPack;

    static TRY_FLAG: Mutex<bool> = Mutex::new(false);
    
    // Function JS calls
    #[no_mangle]
    pub extern "C" fn set_try_flag() {
        if let Ok(mut b) = TRY_FLAG.lock() {
            *b = true;
        }
        macroquad::logging::info!("set_try_flag!!");
    }
    // Signature of the JS function rust calls
    #[no_mangle]
    extern "C" {
        fn js_send_level_bytes() -> JsObject;
    }

    pub fn try_load_level() -> Option<LevelPack> {
        // If the try flag couldn't be acquired (for some reason?) or if it's false, return None
        // Otherwise set it to false and carry on the function
        match TRY_FLAG.lock() {
            Ok(mut b) if *b => { *b = false },
            _ => return None,
        };

        macroquad::logging::info!("attempted!!");

        let data = unsafe { js_send_level_bytes() };
        if data.is_nil() {
            return None;
        }
    
        let mut buf = vec![];
        data.to_byte_buffer(&mut buf);
    
        LevelPack::load_from_file(buf)
    }
}
