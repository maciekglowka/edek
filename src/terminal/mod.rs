use crossterm::{
    cursor,
    event::{read, Event},
    ExecutableCommand,
    QueueableCommand,
    style,
    terminal::{
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        size,
    }
};
use std::io::Write;

use crate::traits::{Screen, ScreenRenderer};

pub struct TerminalScreen<'a, W: Write> {
    target: &'a mut W
}

impl<'a, W: Write> TerminalScreen<'a, W> {
    pub fn new(target: &'a mut W) -> TerminalScreen<'a, W> {
        TerminalScreen { target }
    }
    pub fn init(&mut self) {
        self.target.execute(EnterAlternateScreen)
            .expect("The terminal could not be started!");
        enable_raw_mode().expect("Could not start the raw mode!");
    }
}

impl<W: Write> Drop for TerminalScreen<'_, W> {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not leave the raw mode!");
        self.target.execute(LeaveAlternateScreen).expect("Could not leave the alternate screen!");
    }
}

impl<W: Write> ScreenRenderer for TerminalScreen<'_, W> {
    fn render(&mut self, screen: Screen) {
        self.target.queue(cursor::MoveTo(0, 0));
        for line in screen.content.iter() {
            self.target.queue(Clear(ClearType::CurrentLine));
            self.target.queue(style::Print(line));
            self.target.queue(cursor::MoveToNextLine(1));
        }
        self.target.queue(cursor::MoveTo((screen.cursor_x) as u16, (screen.cursor_y) as u16));
        self.target.flush();
    }
}

pub fn get_size() -> (usize, usize) {
    let (w, h) = size().expect("Unknown terminal size!");
    (w as usize, h as usize)
}

pub fn wait_for_event() -> Option<Event> {
    match read() {
        Ok(e) => Some(e),
        _ => None
    }
}
