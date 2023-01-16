use syntect::{
    easy::HighlightLines,
    parsing::{SyntaxReference, SyntaxSet},
    highlighting::{Style, ThemeSet}
};

use crate::traits::{Span, StyledText, SyntaxHighlighter};

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
    fn highlight_lines<'a>(&'a self, lines: &Vec<&'a str>) -> Option<StyledText> {
        let syntax = self.syntax.as_ref()?;
        let mut h = HighlightLines::new(&syntax, &self.ts.themes["base16-ocean.dark"]);

        let styled = lines.iter()
            .map(|line| {
                h.highlight_line(line, &self.ps).unwrap()
                    .iter()
                    .map(|a| Span { text: a.1, col: style_to_col(a.0)})
                    .collect::<Vec<_>>()
            })
            .collect();
        Some(styled)
    }
    fn set_syntax_from_ext(&mut self, ext: &str) {
        self.syntax = Some(self.ps.find_syntax_by_extension(ext).unwrap().clone());
    }
}

fn style_to_col(style: Style) -> (u8, u8, u8) {
    (style.foreground.r, style.foreground.g, style.foreground.b)
}