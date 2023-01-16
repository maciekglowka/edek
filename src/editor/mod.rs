use crate::traits::{EditorIO, ScreenRenderer, SyntaxHighlighter};

pub mod commands;
mod common;
mod layout;
mod text;
mod window;

pub use common::CursorMove;
pub use text::Text;

use layout::EditorLayout;
use window::EditorWindow;

pub struct Editor {
    pub is_running: bool,
    pub window: EditorWindow,
    pub layout: EditorLayout,
    io: Box<dyn EditorIO>,
    highlighter: Option<Box<dyn SyntaxHighlighter>>
}

impl Editor {
    pub fn new(mut io: Box<dyn EditorIO>) -> Editor {
        let content = match io.load() {
            Ok(s) => Text::from_str(&s),
            Err(_) => Text::new()
        };
        let mut editor = Editor { 
            is_running: true,
            io,
            highlighter: None,
            layout: EditorLayout::new(),
            window: EditorWindow::new() 
        };
        editor.window.text = content;
        editor
    }
    pub fn set_highlighter(&mut self, highlighter: Box<dyn SyntaxHighlighter>) {
        self.highlighter = Some(highlighter);
    }
    pub fn set_syntax(&mut self) {
        self.highlighter.as_mut().unwrap().set_syntax_from_ext("rs");
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.layout.resize(w, h);
        let (win_w, win_h) = self.layout.get_window_size();
        self.window.resize(win_w, win_h);
    }
    pub fn render<T: ScreenRenderer>(&self, renderer: &mut T) {
        renderer.render(self.layout.to_render_screen(self));
    }
}