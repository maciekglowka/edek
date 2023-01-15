use std::cmp;

use crate::traits::Screen;
use super::{
    common::CursorMove,
    text::Text
};

#[derive(Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize
}

#[derive(Default)]
pub struct EditorWindow {
    pub cursor: Cursor,
    pub text: Text,
    pub w: usize,
    pub h: usize,
    pub scroll_x: usize,
    pub scroll_y: usize
}

impl EditorWindow {
    pub fn new() -> EditorWindow {
        EditorWindow { ..Default::default() }
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
    }
    pub fn move_cursor(&mut self, dir: CursorMove) {
        match dir {
            CursorMove::Down(v) => self.cursor.y += v,
            CursorMove::Home => self.cursor.x = 0,
            CursorMove::Left(v) => self.cursor.x = self.cursor.x.saturating_sub(v),
            CursorMove::Right(v) => self.cursor.x += v,
            CursorMove::Up(v) => self.cursor.y = self.cursor.y.saturating_sub(v)
        };
        self.cursor.y = cmp::min(self.cursor.y, self.text.lines.len() - 1);
        self.cursor.x = cmp::min(self.cursor.x, self.text.lines[self.cursor.y].len());
        self.check_scroll();
    }
    fn check_scroll(&mut self) {
        self.scroll_y = cmp::min(self.scroll_y, self.cursor.y);
        self.scroll_y = cmp::max(self.scroll_y, self.cursor.y.saturating_sub(self.h - 1));

        self.scroll_x = cmp::min(self.scroll_x, self.cursor.x);
        self.scroll_x = cmp::max(self.scroll_x, self.cursor.x.saturating_sub(self.w));
    }
    pub fn get_row_bounds(&self) -> (usize, usize) {
        (self.scroll_y, cmp::min(self.scroll_y + self.h, self.text.lines.len()))
    }
    pub fn visible_lines(&self) -> Vec<(usize, &str)> {
        let (min_row, max_row) = self.get_row_bounds();
        let max_col = self.scroll_x + self.w;
        let mut row = min_row;
        self.text.lines[min_row..max_row].into_iter()
            .map(|line| {
                let text = match line.len() {
                    l if l >= self.scroll_x => &line[self.scroll_x..cmp::min(max_col + 1, l)],
                    _ => ""
                };
                row += 1;
                (row, text)
            })
            .collect()
    }
    pub fn insert_char(&mut self, c: char) {
        self.text.insert_char(c, self.cursor.y, self.cursor.x);
        self.cursor.x += 1;
    }
    pub fn insert_line(&mut self) {
        if self.text.insert_line(self.cursor.y, self.cursor.x).is_ok() {
            self.move_cursor(CursorMove::Down(1));
            self.move_cursor(CursorMove::Home);
        };
    }
}