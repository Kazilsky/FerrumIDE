use crossterm::event::KeyCode;
use crossterm::event;

#[macro_export]
macro_rules! test_macros {
    ($test:expr) => {
        $test * $test
    };
}

// pub fn key_to_action(key: KeyCode, buffer) {
//     if let Some(key) = event::read()?.as_key_press_event() {
//         match buffer.input_mode {
//             InputMode::Normal => match key.code {
//                 KeyCode::Char('e') => {
//                     self.input_mode = InputMode::Editing;
//                 }
//                 KeyCode::Char('q') => {
//                     return Ok(());
//                 }
//                 _ => {}
//             },
//             InputMode::Editing => match key.code {
//                 KeyCode::Enter => buffer.line_break(),
//                 KeyCode::Char(to_insert) => buffer.enter_char(to_insert),
//                 KeyCode::Backspace => buffer.delete_char(),
//                 KeyCode::Left => buffer.move_cursor_left(),
//                 KeyCode::Right => buffer.move_cursor_right(),
//                 KeyCode::Esc => self.input_mode = InputMode::Normal,
//                 _ => {}
//             },
//         }
//     }
// }