use std::{
    borrow::Cow,
    cmp
};

use crate::globals;
use crate::traits::{Screen, Span};

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
    pub fn to_render_screen<'a>(&'a self, editor: &'a Editor) -> Screen {
        let highlighted = editor.highlighter.as_ref().unwrap().highlight_lines(&editor.window.visible_lines()).unwrap();
        let start = editor.window.get_row_bounds().0;
        let text_lines: Vec<_> = highlighted
            .into_iter()
            .enumerate()
            .map(|(idx, line)| {
                let mut out = vec!(Span { text: Cow::Owned(format!("{:>width$} ", start + idx + 1, width=self.sidebar_width-1 )), ..Default::default() });
                out.extend(
                    fit_styled(line, editor.window.scroll_x, editor.window.scroll_x + editor.window.w)
                );
                out
            })
            .collect();
        let mut lines = vec!(
            vec!(Span { text: Cow::Owned(format!("{:?}", editor.io.get_path())), ..Default::default() }),
            vec!(Span { text: Cow::Owned(format!("Line: {}, Column: {}", editor.window.cursor.y + 1, editor.window.cursor.x + 1)), ..Default::default() })
        );
        lines.extend(text_lines);
        Screen {
            content: lines,
            cursor_x: editor.window.cursor.x + self.sidebar_width - editor.window.scroll_x,
            cursor_y: editor.window.cursor.y + self.menu_height - editor.window.scroll_y
        }
    }
}

fn fit_styled(line: Vec<Span<&str>>, start: usize, end: usize) -> Vec<Span<Cow<str>>> {
    // refactor needed
    let mut i = 0;
    let mut out = Vec::new();

    for span in line {
        let l = span.text.chars().count();
        match l + i {
            a if a <= start => (),
            a if a > end => {
                let s = span.text.char_indices().nth(start.saturating_sub(i)).unwrap().0;
                let e = span.text.char_indices().nth((end-start).saturating_sub(i)).unwrap().0;
                out.push(Span { text: Cow::Borrowed(&span.text[s..e]), col: span.col });
                break;
            },
            _ => {
                let s = span.text.char_indices().nth(start.saturating_sub(i)).expect(&format!("{}", i)).0;
                out.push(Span { text: Cow::Borrowed(&span.text[s..]), col: span.col });
            }
        }
        i += l;
    }
    let remainder = (end-start).saturating_sub(out.iter().map(|a| a.text.len()).sum());
    if remainder > 0 {out.push(Span { text: Cow::Owned(" ".repeat(remainder)), ..Default::default() })}
    out
}