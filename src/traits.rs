use std::{
    borrow::Cow,
    error::Error
};

pub trait EditorIO {
    fn load(&mut self) -> Result<String, Box<dyn Error>>;
    fn save(&mut self, content: &str) -> Result<(), Box<dyn Error>>;
    fn get_path(&self) -> Option<&String>;
}

pub trait ScreenRenderer {
    fn render(&mut self, screen: Screen);
}

pub trait SyntaxHighlighter {
    fn highlight_lines<'a>(&'a self, lines: &Vec<&'a str>) -> Option<StyledText<&'a str>>;
    fn set_syntax_from_ext(&mut self, ext: &str);
}

pub type StyledText<D> = Vec<Vec<Span<D>>>;

pub struct Span<D: std::fmt::Display + Default> {
    pub text: D,
    pub col: (u8, u8, u8)
}

impl<D: std::fmt::Display + Default> Default for Span<D> {
    fn default() -> Self {
        Span { col: (255, 255, 255), text: Default::default() }
    }
}

pub struct Screen<'a> {
    pub content: StyledText<Cow<'a, str>>,
    pub cursor_x: usize,
    pub cursor_y: usize
}