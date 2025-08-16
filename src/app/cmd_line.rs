use std::fs;
use crate::app::{App, Mode};

pub fn execute(app: &mut App) {
    match app.cmd_buffer.as_str() {
        "q" => quit(app),
        "w" => write(app),
        "wq" => {
            write(app);
            quit(app);
        }
        _ => {}
    }
    app.switch_mode(Mode::Normal);
}

fn quit(app: &mut App) {
    app.alive = false;
}

fn write(app: &mut App) {
    fs::write(&app.editor.file, app.editor.buffer.join("\n")+"\n")
        .expect("Coud not save file");
}
