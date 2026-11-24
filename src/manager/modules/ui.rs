use crossterm::{
    cursor::{self, SetCursorStyle},
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::{self, Write};

use crate::manager::InputMode;

// ==========================================
// 1. ТЕМА ОФОРМЛЕНИЯ (В стиле TokyoNight)
// ==========================================
pub struct Theme {
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_highlight: Color, // Подсветка текущей строки
    pub fg: Color,
    pub fg_gutter: Color,    // Цвет номеров строк и рамок
    pub accent_normal: Color,
    pub accent_insert: Color,
}

impl Theme {
    pub fn tokyo_night() -> Self {
        Self {
            bg: Color::Rgb { r: 26, g: 27, b: 38 },           // #1a1b26 (Основной фон)
            bg_dark: Color::Rgb { r: 22, g: 22, b: 30 },      // #16161e (Фон статус-бара/панелей)
            bg_highlight: Color::Rgb { r: 41, g: 46, b: 66 }, // #292e42 (Активная строка)
            fg: Color::Rgb { r: 192, g: 202, b: 245 },        // #c0caf5 (Основной текст)
            fg_gutter: Color::Rgb { r: 86, g: 95, b: 137 },   // #565f89 (Номера строк, рамки)
            accent_normal: Color::Rgb { r: 122, g: 162, b: 247 }, // #7aa2f7 (Синий для Normal)
            accent_insert: Color::Rgb { r: 158, g: 206, b: 106 }, // #9ece6a (Зеленый для Insert)
        }
    }
}

// ==========================================
// 2. ГЕОМЕТРИЯ (Добавили центрирование для попапов)
// ==========================================
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn split_bottom(&self, bottom_height: u16) -> (Rect, Rect) {
        let top_h = self.height.saturating_sub(bottom_height);
        (
            Rect::new(self.x, self.y, self.width, top_h),
            Rect::new(self.x, self.y + top_h, self.width, bottom_height),
        )
    }

    pub fn split_left(&self, left_width: u16) -> (Rect, Rect) {
        let actual_left_w = left_width.min(self.width);
        let right_w = self.width.saturating_sub(actual_left_w);
        (
            Rect::new(self.x, self.y, actual_left_w, self.height),
            Rect::new(self.x + actual_left_w, self.y, right_w, self.height),
        )
    }

    /// Создает плавающее окно по центру текущей области (как в Helix/Telescope)
    pub fn centered(&self, width: u16, height: u16) -> Rect {
        let w = width.min(self.width);
        let h = height.min(self.height);
        let x = self.x + (self.width.saturating_sub(w)) / 2;
        let y = self.y + (self.height.saturating_sub(h)) / 2;
        Rect::new(x, y, w, h)
    }
}

// ==========================================
// 3. СОВРЕМЕННЫЕ ВИДЖЕТЫ
// ==========================================

/// Рамка со скругленными углами ╭─╮ и заливкой фона
pub struct Block<'a> {
    pub title: &'a str,
    pub theme: &'a Theme,
    pub bg_color: Color,
}

impl<'a> Block<'a> {
    pub fn render<W: Write>(&self, w: &mut W, area: Rect) -> io::Result<Rect> {
        if area.width < 2 || area.height < 2 {
            return Ok(area);
        }
        let inner_w = (area.width as usize).saturating_sub(2);
        let title_clipped: String = self.title.chars().take(inner_w).collect();
        let top_line = "─".repeat(inner_w.saturating_sub(title_clipped.chars().count()));

        queue!(w, SetBackgroundColor(self.bg_color), SetForegroundColor(self.theme.fg_gutter))?;

        // Скругленные углы!
        queue!(
            w,
            cursor::MoveTo(area.x, area.y),
            Print(format!("╭{}{}╮", title_clipped, top_line))
        )?;

        for y in (area.y + 1)..(area.y + area.height - 1) {
            queue!(
                w,
                cursor::MoveTo(area.x, y),
                Print("│"),
                cursor::MoveTo(area.x + area.width - 1, y),
                Print("│")
            )?;
        }

        queue!(
            w,
            cursor::MoveTo(area.x, area.y + area.height - 1),
            Print(format!("╰{}╯", "─".repeat(inner_w)))
        )?;

        Ok(Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        ))
    }
}

/// Продвинутый виджет редактора (с номерами строк и подсветкой активной строки)
pub struct EditorView<'a> {
    pub lines: &'a [String],
    pub active_row: u16,
    pub theme: &'a Theme,
}

impl<'a> EditorView<'a> {
    /// Возвращает (X_начала_текста, Y_начала_текста), чтобы мы знали куда ставить курсор
    pub fn render<W: Write>(&self, w: &mut W, area: Rect) -> io::Result<(u16, u16)> {
        let gutter_width = 5; // " 123 " - 5 символов под номера строк
        let text_width = (area.width as usize).saturating_sub(gutter_width);

        for row in 0..(area.height as usize) {
            let screen_y = area.y + (row as u16);
            let is_active = (row as u16) == self.active_row;

            // Если строка активная — меняем фон на чуть более светлый (Cursorline)
            let row_bg = if is_active { self.theme.bg_highlight } else { self.theme.bg };
            queue!(w, cursor::MoveTo(area.x, screen_y), SetBackgroundColor(row_bg))?;

            // 1. Рисуем Gutter (Номера строк или тильду ~)
            if row < self.lines.len() {
                // Активный номер строки рисуем ярким цветом, остальные — тусклым
                let num_color = if is_active { self.theme.fg } else { self.theme.fg_gutter };
                queue!(
                    w,
                    SetForegroundColor(num_color),
                    Print(format!("{:>3}  ", row + 1)) // Выравнивание по правому краю
                )?;
            } else {
                // Пустые строки за пределами текста (как в Vim)
                queue!(w, SetForegroundColor(self.theme.fg_gutter), Print("  ~  "))?;
            }

            // 2. Рисуем сам текст
            let line_text = self.lines.get(row).map(|s| s.as_str()).unwrap_or("");
            let clipped: String = line_text.chars().take(text_width).collect();
            let padding = " ".repeat(text_width.saturating_sub(clipped.chars().count()));

            queue!(
                w,
                SetForegroundColor(self.theme.fg),
                Print(clipped),
                Print(padding) // Заливаем фон до конца строки
            )?;
        }

        // Возвращаем координаты, где начинается именно ТЕКСТ (с учетом отступа под номера)
        Ok((area.x + gutter_width as u16, area.y))
    }
}

// ==========================================
// 4. СБОРКА ИНТЕРФЕЙСА
// ==========================================

pub struct UI {
    pub theme: Theme,
    pub show_command_palette: bool, // Включи true, чтобы увидеть всплывающее окно!
}

impl UI {
    pub fn new() -> Self {
        Self {
            theme: Theme::tokyo_night(),
            show_command_palette: false, // Попробуй поменять на true!
        }
    }

    pub fn render<W: Write>(
        &self,
        w: &mut W,
        strokes_list: &[String],
        char_index: u16,
        present_stroke: u16,
        mode: &InputMode,
        cursor_visible: bool,
    ) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        if width < 20 || height < 5 {
            return Ok(());
        }

        queue!(w, cursor::Hide)?;

        let full_screen = Rect::new(0, 0, width, height);
        
        // В Helix нет рамок вокруг всего экрана! Это дает больше воздуха.
        // Просто делим экран на рабочую зону и статус-бар (1 строка снизу).
        let (main_area, status_area) = full_screen.split_bottom(1);

        // --- РИСУЕМ РЕДАКТОР НА ВЕСЬ ЭКРАН ---
        let editor = EditorView {
            lines: strokes_list,
            active_row: present_stroke,
            theme: &self.theme,
        };
        let (text_start_x, text_start_y) = editor.render(w, main_area)?;

        // --- РИСУЕМ СТАТУС-БАР (В стиле Nvim/Helix) ---
        self.render_status_bar(w, status_area, mode, char_index, present_stroke, strokes_list.len())?;

        // --- ПЛАВАЮЩЕЕ ОКНО (Если активно) ---
        // Например, нажмешь пробел в Normal моде и откроется такое меню поверх текста
        if self.show_command_palette {
            let popup_area = main_area.centered(40, 8);
            let inner_popup = Block {
                title: " 🔍 Поиск файла ",
                theme: &self.theme,
                bg_color: self.theme.bg_dark,
            }.render(w, popup_area)?;

            // Заполняем внутренность попапа
            for row in 0..inner_popup.height {
                let text = match row {
                    0 => " > src/main.rs",
                    1 => "   src/ui.rs",
                    2 => "   Cargo.toml",
                    _ => "",
                };
                let pad = " ".repeat((inner_popup.width as usize).saturating_sub(text.chars().count()));
                queue!(
                    w,
                    cursor::MoveTo(inner_popup.x, inner_popup.y + row),
                    SetBackgroundColor(self.theme.bg_dark),
                    SetForegroundColor(self.theme.fg),
                    Print(text),
                    Print(pad)
                )?;
            }
        }

        // --- УМНЫЙ КУРСОР ---
        if cursor_visible && !self.show_command_palette {
            // Меняем форму курсора в зависимости от режима!
            let cursor_style = match mode {
                InputMode::Normal => SetCursorStyle::SteadyBlock, // Кубик █
                InputMode::Insert => SetCursorStyle::SteadyBar,   // Черточка |
            };

            let cursor_x = text_start_x + char_index;
            let cursor_y = text_start_y + present_stroke;
            
            queue!(
                w,
                cursor_style,
                cursor::MoveTo(cursor_x, cursor_y),
                cursor::Show
            )?;
        }

        queue!(w, ResetColor)?;
        w.flush()
    }

    fn render_status_bar<W: Write>(
        &self,
        w: &mut W,
        area: Rect,
        mode: &InputMode,
        col: u16,
        row: u16,
        total_lines: usize,
    ) -> io::Result<()> {
        let (mode_str, mode_color) = match mode {
            InputMode::Normal => (" NOR ", self.theme.accent_normal),
            InputMode::Insert => (" INS ", self.theme.accent_insert),
        };

        let file_info = " 🦀 main.rs ";
        // Процент прокрутки файла, как в Vim
        let percent = if total_lines == 0 { 100 } else { ((row as usize + 1) * 100) / total_lines.max(1) };
        let pos_str = format!(" {}%  Ln {}, Col {} ", percent, row + 1, col + 1);

        let used_width = mode_str.len() + file_info.chars().count() + pos_str.len();
        let spaces = " ".repeat((area.width as usize).saturating_sub(used_width));

        queue!(
            w,
            cursor::MoveTo(area.x, area.y - 1), // -1 because I use it for UI, so it's starder from 1, not zero
            // 1. Блок режима (NOR / INS)
            SetBackgroundColor(mode_color),
            SetForegroundColor(self.theme.bg_dark),
            Print(mode_str),
            // 2. Имя файла
            SetBackgroundColor(self.theme.bg_highlight),
            SetForegroundColor(self.theme.fg),
            Print(file_info),
            // 3. Пустое пространство статус-бара
            SetBackgroundColor(self.theme.bg_dark),
            Print(spaces),
            // 4. Координаты и проценты справа
            SetForegroundColor(self.theme.fg_gutter),
            Print(pos_str)
        )
    }
}