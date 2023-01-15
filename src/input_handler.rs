use crossterm::event::{
    Event,
    Event::Key,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};
use crate::editor::{
    commands,
    CursorMove,
    Editor
};

pub fn handle_event(
    editor: &mut Editor,
    event: Event
) {
    match event {
        Key(e) => handle_key_event(editor, e),
        _ => ()
    }
}

fn handle_key_event(
    editor: &mut Editor,
    event: KeyEvent
) {
    match event {
        KeyEvent {code: KeyCode::Char(c), kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::WriteCharCmd::new(c), editor),
        KeyEvent {code: KeyCode::Enter, kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::NewLineCmd::new(), editor),
        KeyEvent {code: KeyCode::Left, kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::MoveCursorCmd::new(CursorMove::Left(1)), editor),
        KeyEvent {code: KeyCode::Right, kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::MoveCursorCmd::new(CursorMove::Right(1)), editor),
        KeyEvent {code: KeyCode::Up, kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::MoveCursorCmd::new(CursorMove::Up(1)), editor),
        KeyEvent {code: KeyCode::Down, kind: KeyEventKind::Press, ..} => 
            commands::execute(commands::MoveCursorCmd::new(CursorMove::Down(1)), editor),
        KeyEvent {code: KeyCode::Esc, kind: KeyEventKind::Press, ..} =>
            commands::execute(commands::ExitCmd, editor),
        _ => (),
    }
}