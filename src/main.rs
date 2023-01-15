use std::{
    env,
    io::stdout
};

mod globals;
mod editor;
mod input_handler;
mod io;
mod terminal;
mod traits;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1);

    let editor_io = io::FileIO::new(path.cloned());
    let mut editor = editor::Editor::new(Box::new(editor_io));
    
    let mut target = stdout();
    let mut terminal_screen = terminal::TerminalScreen::new(&mut target);
    terminal_screen.init();
    let (w, h) = terminal::get_size();
    
    editor.resize(w, h);

    while editor.is_running {
        editor.render(&mut terminal_screen);
        if let Some(event) = terminal::wait_for_event() {
            input_handler::handle_event(&mut editor, event);
        };
    }
}
