use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Characters {
    pub characters: Vec<String>,
}

impl Characters {
    pub fn new(characters: Vec<String>) -> Self {
        Self {
            characters: characters,
        }
    }
    pub fn into_inner(self) -> Vec<String> {
        self.characters
    }
}
