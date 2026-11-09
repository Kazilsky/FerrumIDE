pub struct Cursor {
    pub character_index: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { character_index: 0 }
    }

    // INFO: I use it when use buffer:enter_chat
    pub fn move_left(&mut self) {
        self.character_index = self.character_index.saturating_sub(1);
    }

    // INFO: I use it when use buffer:enter_chat
    pub fn move_right(&mut self) {
        self.character_index += 1;
    }

    pub fn set_x(&mut self, x: usize) {
        self.character_index = x;
    }
}
