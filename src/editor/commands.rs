use super::{
    common::CursorMove,
    Editor,
    text::Text
};

pub fn execute(
    mut command: impl Command + 'static,
    editor: &mut Editor
) {
    command.execute(editor);
}

pub trait Command {
    // result is true if the command modifies the text
    fn execute(&mut self, editor: &mut Editor) -> bool;
    fn rollback(&mut self, editor: &mut Editor) {}
}

#[derive(Default)]
pub struct WriteCharCmd {
    c: char,
    state: Text
}
impl WriteCharCmd {
    pub fn new(c: char) -> WriteCharCmd {
        WriteCharCmd { c, ..Default::default() }
    }
}
impl Command for WriteCharCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        self.state = editor.window.text.clone();
        editor.window.insert_char(self.c);
        true
    }
    fn rollback(&mut self, editor: &mut Editor) {
        editor.window.text = self.state.clone();
    }
}

#[derive(Default)]
pub struct RemoveCharCmd {
    state: Text
}
impl RemoveCharCmd {
    pub fn new() -> RemoveCharCmd {
        RemoveCharCmd { ..Default::default() }
    }
}
impl Command for RemoveCharCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        self.state = editor.window.text.clone();
        editor.window.remove_char();
        true
    }
    fn rollback(&mut self, editor: &mut Editor) {
        editor.window.text = self.state.clone();
    }
}


pub struct MoveCursorCmd {
    dir: CursorMove
}
impl MoveCursorCmd {
    pub fn new(dir: CursorMove) -> MoveCursorCmd {
        MoveCursorCmd { dir }
    }
}
impl Command for MoveCursorCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        editor.window.move_cursor(self.dir);
        false
    }
}

pub struct ExitCmd;
impl Command for ExitCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        editor.is_running = false;
        false
    }
}

#[derive(Default)]
pub struct NewLineCmd {
    state: Text
}
impl NewLineCmd {
    pub fn new() -> NewLineCmd {
        NewLineCmd { ..Default::default() }
    }
}
impl Command for NewLineCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        self.state = editor.window.text.clone();
        editor.window.insert_line();
        true
    }
    fn rollback(&mut self, editor: &mut Editor) {
        editor.window.text = self.state.clone();
    }
}

pub struct SaveFileCmd;
impl Command for SaveFileCmd {
    fn execute(&mut self, editor: &mut Editor) -> bool {
        editor.io.save(&editor.window.text.to_str());
        false
    }
}