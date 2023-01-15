use crate::globals;
use crate::traits::Screen;

use super::window::EditorWindow;

pub struct EditorLayout {
    pub menu_height: usize,
    pub sidebar_width: usize,
    pub w: usize,
    pub h: usize
}

impl EditorLayout {
    pub fn new() -> EditorLayout {
        EditorLayout {
            menu_height: globals::MENU_HEIGHT,
            sidebar_width: globals::SIDEBAR_WIDTH,
            w: 0,
            h: 0
        }
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
    }
    pub fn get_window_size(&self) -> (usize, usize) {
        (self.w - self.sidebar_width, self.h - self.menu_height)
    }
    pub fn to_render_screen(&self, window: &EditorWindow) -> Screen {
        let text_lines: Vec<_> = window.visible_lines()
            .iter()
            .map(|(idx, line)| {
                let mut out = format!("{:>width$} ", idx, width=self.sidebar_width-1 );
                out.push_str(line);
                out
            })
            .collect();
        let mut lines = vec!(
            format!("EDEK"),
            format!("Line: {}, Column: {} {}", window.cursor.y + 1, window.cursor.x + 1, window.scroll_y)
        );
        lines.extend(text_lines);
        Screen {
            content: lines,
            cursor_x: window.cursor.x + self.sidebar_width - window.scroll_x,
            cursor_y: window.cursor.y + self.menu_height - window.scroll_y
        }
    }
}