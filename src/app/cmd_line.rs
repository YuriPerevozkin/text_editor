use crate::app::{App, Mode};

pub fn execute(app: &mut App) {
    match app.cmd_buffer.as_str() {
        "q" => app.alive = false,
        "w" => app.editor.save_file().unwrap(),
        _ => {}
    }
    app.switch_mode(Mode::Normal);
}
