use ratatui::text::Line;

pub struct Buffer {
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    pub strokes_array: Vec<Line<'static>>,
    /// Number a present stroke
    present_stroke: usize,
    /// String text in stroke
    pub string_from_stroke: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            character_index: 0,
            strokes_array: Vec::new(),
            present_stroke: 0,
            string_from_stroke: String::new(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.string_from_stroke.insert(index, new_char);
        self.move_cursor_right();
    } 

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.string_from_stroke.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.string_from_stroke.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.string_from_stroke = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    /// INFO: ДОДЕЛАТЬ!!
    pub fn line_break(&mut self) {
        
    }


    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.string_from_stroke.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    /// MODULE PART
    
    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.string_from_stroke
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.string_from_stroke.len())
    }
}