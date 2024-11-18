pub struct Player {
    pub name: String,
    pub marker: char,
}

impl Player {
    pub fn new(name: String, marker: char) -> Self {
        Self { name, marker }
    }
}