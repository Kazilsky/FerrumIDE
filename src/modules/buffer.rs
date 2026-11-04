pub struct Buffer {
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    pub strokes_array: Vec<String>,
    /// Number a present stroke
    present_stroke: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            character_index: 0,
            strokes_array: vec![String::new()],
            // INFO: Create with 1 stroke already (because witout in, it be with error)
            present_stroke: 0,
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

    pub fn move_cursor_up(&mut self) {
        if self.present_stroke > 0 {
            self.present_stroke -= 1;
            self.character_index = self.clamp_cursor(self.character_index);
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.present_stroke < self.max_strokes() {
            self.present_stroke += 1;
            // Точно так же корректируем курсор по длине строки
            self.character_index = self.clamp_cursor(self.character_index);
        }
    }

    pub fn move_cursor_end(&mut self) {
        if self.character_index != self.byte_index() {
            self.character_index = self.byte_index()
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        let current_index = self.character_index;

        // INFO: I need add new_char in present stroke (it's Line type and I don't know how I can do that) (2 PW, already String)
        // :)
        self.strokes_array[self.present_stroke].insert(current_index, new_char);
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
            let before_char_to_delete = self.strokes_array[self.present_stroke]
                .chars()
                .take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.strokes_array[self.present_stroke]
                .chars()
                .skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.strokes_array[self.present_stroke] =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        } else {
            self.move_cursor_up();
            self.move_cursor_end();
        }
    }

    // INFO: Затестить потому что вроде работает
    pub fn line_break(&mut self) {
        let new_line_slice = self.strokes_array[self.present_stroke].split_off(self.character_index);
        self.strokes_array.insert(self.present_stroke + 1, new_line_slice);

        self.present_stroke += 1;
        self.character_index = 0; 
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.strokes_array[self.present_stroke].chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    const fn max_strokes(&self) -> usize {
        let response = self.strokes_array.len() - 1;
        // because it return one, not zero (like [0])
        response as usize
    }

    /// MODULE PART

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.strokes_array[self.present_stroke].len()
    }
}
