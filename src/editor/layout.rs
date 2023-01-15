use std::cmp;

use crate::globals;
use crate::traits::Screen;

use super::Editor;

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
    pub fn to_render_screen(&self, editor: &Editor) -> Screen {
        let text_lines: Vec<_> = editor.window.visible_lines()
            .iter()
            .map(|(idx, line)| {
                let mut out = format!("{:>width$} ", idx, width=self.sidebar_width-1 );
                // let text = match &editor.highlighter {
                //     None => line.to_string(),
                //     Some(h) => h.highlight_line(&line)
                // };
                out.push_str(slice_str(
                    &line,
                    editor.window.scroll_x,
                    editor.window.scroll_x + editor.window.w
                ));
                out
            })
            .collect();
        let mut lines = vec!(
            format!("{:?}", editor.io.get_path()),
            format!("Line: {}, Column: {}", editor.window.cursor.y + 1, editor.window.cursor.x + 1)
        );
        lines.extend(text_lines);
        Screen {
            content: lines,
            cursor_x: editor.window.cursor.x + self.sidebar_width - editor.window.scroll_x,
            cursor_y: editor.window.cursor.y + self.menu_height - editor.window.scroll_y
        }
    }
}

fn slice_str(s: &str, start: usize, end: usize) -> &str {
    let l = s.chars().count();
    if l == 0 { return "" }
    let a = s.char_indices().nth(cmp::min(start, l - 1)).unwrap().0;
    match s.char_indices().nth(cmp::min(end, l)) {
        Some(b) => &s[a..b.0],
        None => &s[a..]
    }
}