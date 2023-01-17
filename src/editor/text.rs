#[derive(Clone, Default)]
pub struct Text {
    pub lines: Vec<String>
}

impl Text {
    pub fn new() -> Text {
        Text { lines: vec!(String::new()) }
    }
    pub fn from_str(s: &str) -> Text {
        let lines: Vec<_> = s.split('\n').map(|l| l.to_string()).collect();
        Text { lines }
    }
    pub fn to_str(&self) -> String {
        self.lines.join("\n")
    }
    pub fn insert_char(&mut self, c: char, row: usize, col: usize) {
        if let Some(line) = self.lines.get_mut(row) {
            if let Some(i) = line.char_indices().nth(col) {
                line.insert(i.0, c);
            } else {
                line.push(c);
            }
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
    pub fn remove_char(&mut self, row: usize, col: usize) {
        if let Some(line) = self.lines.get_mut(row) {
            if let Some(i) = line.char_indices().nth(col) {
                line.remove(i.0);
            }
        }
    }
    pub fn merge_lines(&mut self, row: usize) {
        // merge current line with a previous one
        if row == 0 { return; }
        let current = match self.lines.get(row) {
            None => return,
            Some(c) => c.clone()
        };
        // the prev should exist since we are not at row 0
        let previous = self.lines.get_mut(row-1).unwrap();
        previous.push_str(&current);
        self.lines.remove(row);
    }
}