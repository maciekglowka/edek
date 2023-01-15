#[derive(Clone, Default)]
pub struct Text {
    pub lines: Vec<String>
}

impl Text {
    pub fn new() -> Text {
        Text { lines: Vec::new() }
    }
    pub fn from_str(s: &str) -> Text {
        let lines: Vec<_> = s.split('\n').map(|l| l.to_string()).collect();
        Text { lines }
    }
    pub fn insert_char(&mut self, c: char, row: usize, col: usize) {
        if let Some(line) = self.lines.get_mut(row) {
            line.insert(col, c);
        }
    }
    // TODO add proper errors
    pub fn insert_line(&mut self, row: usize, col: usize) -> Result<(), ()> {
        let mut line = self.lines.get_mut(row).ok_or(())?;
        let rhs = line[col..].to_string();
        line.truncate(col);
        self.lines.insert(row + 1, rhs.into());
        Ok(())
    }
}