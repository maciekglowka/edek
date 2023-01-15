use std::error::Error;

pub trait EditorIO {
    fn load(&mut self) -> Result<String, Box<dyn Error>>;
    fn save(&mut self, content: &str) -> Result<(), Box<dyn Error>>;
    fn get_path(&self) -> Option<&String>;
}

// #[derive(Debug)]
// pub enum IOStatus {
//     Blank,
//     Attached(String)
// }

pub trait ScreenRenderer {
    fn render(&mut self, screen: Screen);
}

pub struct Screen {
    pub content: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize
}

pub trait SyntaxHighlighter {
    fn highlight_line(&self, s: &str) -> String;
    fn set_syntax(&mut self, ext: &str);
}