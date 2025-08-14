use crate::app::{App, Mode};
use crate::app::cmd_line;

use crossterm::event::{
    KeyCode,
    KeyEvent,
};

pub fn handle_command(app: &mut App, key_event: KeyEvent) {
    match app.mode {
        Mode::Insert => {
            match key_event.code {
                KeyCode::Enter => app.editor.split_line(),
                KeyCode::Backspace => app.editor.delete_char(),
                KeyCode::Char(char) => app.editor.insert_char(char),
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                _ => {}
            }
        }

        Mode::Normal => {
            match key_event.code {
                KeyCode::Char('h') => app.editor.move_cursor_left(1),
                KeyCode::Char('j') => app.editor.move_cursor_down(1),
                KeyCode::Char('k') => app.editor.move_cursor_up(1),
                KeyCode::Char('l') => app.editor.move_cursor_right(1),

                KeyCode::Char('0') => app.editor.move_cursor_to_start(),
                KeyCode::Char('$') => app.editor.move_cursor_to_end(),

                KeyCode::Char('i') => app.switch_mode(Mode::Insert),
                KeyCode::Char('I') => {
                    app.editor.move_cursor_to_start();
                    app.switch_mode(Mode::Insert)
                }

                KeyCode::Char('a') => {
                    app.editor.move_cursor_right(1);
                    app.switch_mode(Mode::Insert);
                }
                KeyCode::Char('A') => {
                    app.editor.move_cursor_to_end();
                    app.switch_mode(Mode::Insert);
                }

                KeyCode::Char('o') => {
                    app.editor.move_cursor_to_end();
                    app.editor.split_line();
                    app.switch_mode(Mode::Insert);
                }
                KeyCode::Char('O') => {
                    app.editor.move_cursor_up(1);
                    app.editor.move_cursor_to_end();
                    app.editor.split_line();
                    app.switch_mode(Mode::Insert);
                }

                KeyCode::Char(':') => app.switch_mode(Mode::Command),
                _ => {}
            }
        }

        Mode::Command => {
            match key_event.code {
                KeyCode::Char(char) => app.cmd_buffer.push(char),
                KeyCode::Backspace => _ = app.cmd_buffer.pop(),
                KeyCode::Enter => cmd_line::execute(app),
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                _ => {}
            }
        }
    }
}
