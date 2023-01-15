use crate::globals;
use crate::traits::{EditorIO, ScreenRenderer};

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
    io: Box<dyn EditorIO>
}

impl Editor {
    pub fn new(mut io: Box<dyn EditorIO>) -> Editor {
        let content = match io.load() {
            Ok(s) => Text::from_str(&s),
            Err(e) => panic!("{:?}", e)
        };
        let mut editor = Editor { 
            is_running: true,
            io,
            layout: EditorLayout::new(),
            window: EditorWindow::new() 
        };
        editor.window.text = content;
        editor
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.layout.resize(w, h);
        let (win_w, win_h) = self.layout.get_window_size();
        self.window.resize(win_w, win_h);
    }
    pub fn render<T: ScreenRenderer>(&self, renderer: &mut T) {
        renderer.render(self.layout.to_render_screen(&self.window));
    }
}