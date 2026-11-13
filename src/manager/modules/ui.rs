use crossterm::{
    cursor, queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

pub struct UI {}

pub enum InputMode {
    Normal,
    Insert,
}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    fn status_line_name(&self, mode: &InputMode) -> &'static str {
        match mode {
            InputMode::Insert => "Insert",
            InputMode::Normal => "Normal",
        }
    }

    pub fn render<W: Write>(
        &self,
        w: &mut W,
        strokes_list: &[String],
        char_index: u16,
        present_stroke: u16,
        cursor_visible: bool,
    ) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        if width < 4 || height < 5 {
            return Ok(());
        }

        queue!(w, cursor::Hide, Clear(ClearType::All))?;

        let title = " Editor ";
        let top_border_len = (width as usize).saturating_sub(2 + title.len());
        queue!(
            w,
            cursor::MoveTo(0, 0),
            Print(format!("┌{}{}┐", title, "─".repeat(top_border_len)))
        )?;

        let inner_width = (width as usize).saturating_sub(0);
        let inner_height = (height as usize).saturating_sub(0);

        for row in 0..inner_height {
            let screen_y = (row + 1) as u16;
            let line_text = strokes_list.get(row).map(|s| s.as_str()).unwrap_or("");

            let clipped: String = line_text.chars().take(inner_width).collect();

            queue!(
                w,
                cursor::MoveTo(0, screen_y),
                Print("│"),
                Print(&clipped),
                cursor::MoveTo(width - 1, screen_y),
                Print("│")
            )?;
        }

        let bottom_border_len = (width as usize).saturating_sub(2);
        queue!(
            w,
            cursor::MoveTo(0, height - 1),
            Print(format!("└{}┘", "─".repeat(bottom_border_len)))
        )?;

        if cursor_visible {
            queue!(
                w,
                cursor::MoveTo(char_index + 1, present_stroke + 1),
                cursor::Show
            )?;
        }

        w.flush()
    }
}
