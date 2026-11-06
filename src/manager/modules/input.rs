use crossterm::event::KeyCode;

macro_rules! test_macros {
    ($test:expr) => {
        $test * $test
    };
}

#[macro_export]

macro_rules! dispatch_input {
    ($self:ident, $key:expr, {
        $( Normal => $n_intent:ident => $n_action:block )*
        $( Insert => $e_intent:ident $( ( $param:ident ) )? => $e_action:block )*
    }) => {
        match $self.input_mode {
            InputMode::Normal => match parse_normal_key($key) {
                $( NormalIntent::$n_intent => $n_action, )*
            },
            InputMode::Insert => match parse_editing_key($key) {
                $(
                    EditIntent::$e_intent $( ( $param ) )? => $e_action,
                )*
            },
        }
    };
}

pub enum EditIntent {
    Char(char),
    Backspace,
    Enter,
    Left,
    Right,
    Escape,
    None,
}

pub enum NormalIntent {
    SwitchToEdit,
    Quit,
    None,
}

pub fn parse_editing_key(key: KeyCode) -> EditIntent {
    match key {
        KeyCode::Char(c) => EditIntent::Char(c),
        KeyCode::Backspace => EditIntent::Backspace,
        KeyCode::Enter => EditIntent::Enter,
        KeyCode::Left => EditIntent::Left,
        KeyCode::Right => EditIntent::Right,
        KeyCode::Esc => EditIntent::Escape,
        _ => EditIntent::None,
    }
}

pub fn parse_normal_key(key: KeyCode) -> NormalIntent {
    match key {
        KeyCode::Char('e') => NormalIntent::SwitchToEdit,
        KeyCode::Char('q') => NormalIntent::Quit,
        _ => NormalIntent::None,
    }
}
