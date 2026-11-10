use std::println;

use crossterm::event::KeyCode;

pub struct Buffer {
    /// History of recorded messages
    pub strokes_array: Vec<String>,
    /// Number a present stroke
    pub present_stroke: usize,
}

pub enum Direction {
    Up,
    Down
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            strokes_array: vec![String::new()],
            present_stroke: 0,
        }
    }

    pub fn enter_char(&mut self, char_index: usize, new_char: char) {
        let byte_index = self.byte_index(char_index);

        // Safely insert the character using the converted byte index
        self.strokes_array[self.present_stroke].insert(byte_index, new_char);
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
        self.strokes_array[self.present_stroke] =
            before_char_to_delete.chain(after_char_to_delete).collect();
    }

    pub fn line_break(&mut self, char_index: usize, type_break: KeyCode) -> usize {
        match type_break {
            KeyCode::Enter => {
                let byte_symbol = self.byte_index(char_index);

                let new_line_slice = self.strokes_array[self.present_stroke]
                    .split_off(byte_symbol);
                self.strokes_array
                    .insert(self.present_stroke + 1, new_line_slice);

                self.present_stroke += 1;
            }
            KeyCode::Backspace => {
                let current_line = self.strokes_array.remove(self.present_stroke);
                self.present_stroke -= 1;
                let chars_len_current_line = self.strokes_array[self.present_stroke].chars().count();
                self.strokes_array[self.present_stroke].push_str(&current_line);
                
                return chars_len_current_line // I return len for cursor in manager
            }
            _ => {} // Very mutch KeyCode here be, yes, but it's not needable
        }    
        return 0 
    }

    pub fn swap_current_stroke(&mut self, direction: Direction) {
        match direction {
            Direction::Up   => if self.present_stroke > 0                  {self.present_stroke -= 1}
            Direction::Down => if self.present_stroke < self.max_strokes() {self.present_stroke += 1}
        }
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
    pub fn byte_index(&mut self, char_index: usize) -> usize {
        self.strokes_array[self.present_stroke]
            .char_indices()
            .map(|(b_idx, _)| b_idx)
            .nth(char_index)
            // If the index isn't found, check if they are appending to the very end
            .unwrap_or_else(|| self.strokes_array[self.present_stroke].len())
    }
}
