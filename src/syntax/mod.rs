use syntect::{
    easy::HighlightLines,
    parsing::{SyntaxReference, SyntaxSet},
    highlighting::{Theme, ThemeSet},
    util::{LinesWithEndings, as_24_bit_terminal_escaped}
};
use std::cell::RefCell;

use crate::traits::SyntaxHighlighter;

pub struct SyntectHighlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
    syntax: Option<SyntaxReference>
}

impl SyntectHighlighter {
    pub fn new() -> SyntectHighlighter {
        let ts = ThemeSet::load_defaults();
        let ps = SyntaxSet::load_defaults_newlines();
        SyntectHighlighter { ps, ts, syntax: None }
    }
}

impl SyntaxHighlighter for SyntectHighlighter {
    fn highlight_line(&self, s: &str) -> String {
        match &self.syntax {
            Some(syntax) => {
                let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);
                let ranges = h.highlight_line(s, &self.ps).unwrap();
                as_24_bit_terminal_escaped(&ranges[..], false)
            },
            None => s.to_string()
        }
        
    }
    fn set_syntax(&mut self, ext: &str) {
        self.syntax = Some(self.ps.find_syntax_by_extension(ext).unwrap().clone());
    }
}