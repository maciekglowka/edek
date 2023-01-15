use std::error::Error;

pub trait EditorIO {
    fn load(&mut self) -> Result<String, Box<dyn Error>>;
    fn save(&mut self, content: &str) -> Result<(), Box<dyn Error>>;
    fn status(&self) -> IOStatus;
}

pub enum IOStatus {
    Blank,
    Attached(String)
}

pub trait ScreenRenderer {
    fn render(&mut self, screen: Screen);
}

pub struct Screen {
    pub content: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize
}