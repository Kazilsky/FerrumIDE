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
        $( Insert => $e_intent:ident $( ( $param:ident ) )?     => $e_action:block )*
        $( All    => $a_intent:ident $( ( $key_param:ident ) )? => $a_action:block )*
    }) => {
        match parse_all_key($key) {
            AllIntent::None => match $self.context.input_mode {
                InputMode::Normal => match parse_normal_key($key) {
                    $( NormalIntent::$n_intent => $n_action, )*
                    NormalIntent::None => {} // Ignore unused keys in Normal mode
                    _ => {}
                },
                InputMode::Insert => match parse_editing_key($key) {
                    $( EditIntent::$e_intent $( ( $param ) )? => $e_action, )*
                    EditIntent::None => {} // Ignore unused keys in Insert mode
                    _ => {}
                },
            },

            // 2. If it WAS Up/Down/Left/Right, run the All action:
            $( 
                AllIntent::$a_intent $( ( $key_param ) )? => $a_action, 
            )*
            _ => {}     
        }  
    };
}

pub enum EditIntent {
    Char(char),
    Backspace,
    Enter,
    Escape,
    None,
}

pub enum NormalIntent {
    SwitchToEdit,
    Quit,
    None,
}

pub enum AllIntent {
    Up,
    Down,
    Left,
    Right, 
    None
}

pub fn parse_editing_key(key: KeyCode) -> EditIntent {
    match key {
        KeyCode::Char(c) => EditIntent::Char(c),
        KeyCode::Backspace => EditIntent::Backspace,
        KeyCode::Enter => EditIntent::Enter,
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

pub fn parse_all_key(key: KeyCode) -> AllIntent {
    match key {
        KeyCode::Up     => AllIntent::Up,
        KeyCode::Down   => AllIntent::Down,
        KeyCode::Left   => AllIntent::Left,
        KeyCode::Right  => AllIntent::Right,
        _               => AllIntent::None,
    }
}
