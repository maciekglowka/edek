use std::error::Error;

pub trait EditorIO {
    fn load(&mut self) -> Result<String, Box<dyn Error>>;
    fn save(&mut self, content: &str) -> Result<(), Box<dyn Error>>;
    fn get_path(&self) -> Option<&String>;
}

pub trait ScreenRenderer {
    fn render(&mut self, screen: Screen);
}

pub trait SyntaxHighlighter {
    fn highlight_lines<'a>(&'a self, lines: &Vec<&'a str>) -> Option<StyledText>;
    fn set_syntax_from_ext(&mut self, ext: &str);
}

pub type StyledText<'a> = Vec<Vec<Span<'a>>>;

pub struct Span<'a> {
    pub text: &'a str,
    pub col: (u8, u8, u8)
}

pub struct Screen<'a> {
    pub content: StyledText<'a>,
    pub cursor_x: usize,
    pub cursor_y: usize
}