pub struct Buffer {
    /// History of recorded messages
    pub strokes_array: Vec<String>,
    /// Number a present stroke
    pub present_stroke: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            strokes_array: Vec::new(),
            present_stroke: 0,
        }
    }

    pub fn enter_char(&mut self, char_index: usize, new_char: char) {
        let current_index = char_index;

        // INFO: I need add new_char in present stroke (it's Line type and I don't know how I can do that) (2 PW, already String)
        // :)
        self.strokes_array[self.present_stroke].insert(current_index, new_char);
    }

    pub fn delete_char(&mut self, current_index: usize) {
        // let is_not_cursor_leftmost = view.character_index != 0;
        // if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.
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
        self.strokes_array[self.present_stroke] = before_char_to_delete
            .chain(after_char_to_delete)
            .collect();
    } 

    // INFO: Затестить потому что вроде работает
    pub fn line_break(&mut self, split_param: usize) {
        let new_line_slice = self.strokes_array[self.present_stroke].split_off(split_param);
        self.strokes_array.insert(self.present_stroke + 1, new_line_slice);

        self.present_stroke += 1;
    }

    pub fn max_strokes(&self) -> usize {
        let response = self.strokes_array.len() - 1;
        // because it return one, not zero (like [0])
        response as usize
    }

    /// MODULE PART

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.strokes_array[self.present_stroke].len()
    }
}
