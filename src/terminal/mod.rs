use crossterm::{
    cursor,
    event::{read, Event},
    ExecutableCommand,
    QueueableCommand,
    style::{Color, PrintStyledContent, StyledContent, Stylize},
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
use std::{
    borrow::Cow,
    io::Write
};

use crate::traits::{Screen, ScreenRenderer, Span};

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
        self.target.queue(cursor::Hide);
        self.target.queue(cursor::MoveTo(0, 0));
        for line in screen.content.iter() {
            for span in line {
                self.target.queue(PrintStyledContent(span_to_styled(span)));
            }
            self.target.queue(cursor::MoveToNextLine(1));
        }
        self.target.queue(cursor::MoveTo((screen.cursor_x) as u16, (screen.cursor_y) as u16));
        self.target.queue(cursor::Show);
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

fn span_to_styled<'a>(s: &'a Span<Cow<'a, str>>) -> StyledContent<&'a str> {
    s.text.with(Color::Rgb { r: s.col.0, g: s.col.1, b: s.col.2 })
}