use bevy::prelude::*;

use std::collections::HashMap;

pub struct Sprites {
    library: HashMap<String, Handle<TextureAtlas>>,
}

impl Default for Sprites {
    fn default() -> Self {
        Sprites {
            library: HashMap::new(),
        }
    }
}

impl Sprites {
    pub fn add(&mut self, key: &str, atlas_handle: Handle<TextureAtlas>) {
        self.library.insert(key.to_string(), atlas_handle);
    }

    pub fn get(&self, key: &str) -> Option<&Handle<TextureAtlas>> {
        self.library.get(key)
    }
}
