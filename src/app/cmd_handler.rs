use crate::app::cmd_line;
use crate::app::{App, Mode};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_command(app: &mut App, key_event: KeyEvent) {
    match app.mode {
        Mode::Insert => match key_event.code {
            KeyCode::Enter => app.editor.split_line(),
            KeyCode::Backspace => app.editor.delete_char(),
            KeyCode::Char(char) => app.editor.insert_char(char),
            KeyCode::Esc => app.switch_mode(Mode::Normal),
            _ => {}
        },

        Mode::Normal => {
            if let KeyCode::Char(char) = key_event.code {
                app.cmd_buffer.push(char)
            }
            match app.cmd_buffer.as_str() {
                // --- Navigation ---
                "h" => app.editor.move_cursor_left(1),
                "j" => app.editor.move_cursor_down(1),
                "k" => app.editor.move_cursor_up(1),
                "l" => app.editor.move_cursor_right(1),

                "0" => app.editor.move_cursor_to_start(),
                "$" => app.editor.move_cursor_to_end(),

                "G" => app.editor.move_cursor_to_end_of_file(),
                "g" => return,
                "gg" => app.editor.move_cursor_to_start_of_file(),

                // --- Enter insert mode ---
                "i" => app.switch_mode(Mode::Insert),
                "I" => {
                    app.editor.move_cursor_to_start();
                    app.switch_mode(Mode::Insert)
                }

                "a" => {
                    app.editor.move_cursor_right(1);
                    app.switch_mode(Mode::Insert);
                }
                "A" => {
                    app.editor.move_cursor_to_end();
                    app.switch_mode(Mode::Insert);
                }

                "o" => {
                    app.editor.move_cursor_to_end();
                    app.editor.split_line();
                    app.switch_mode(Mode::Insert);
                }
                "O" => {
                    app.editor.move_cursor_up(1);
                    app.editor.move_cursor_to_end();
                    app.editor.split_line();
                    app.switch_mode(Mode::Insert);
                }

                "s" => {
                    app.editor.move_cursor_right(1);
                    app.editor.delete_char();
                    app.switch_mode(Mode::Insert);
                }

                // --- Editing ---
                "d" => return,
                "dd" => app.editor.delete_line(),

                // --- Misc ---
                ":" => app.switch_mode(Mode::Command),
                _ => {}
            }
            app.cmd_buffer.clear();
        }

        Mode::Command => match key_event.code {
            KeyCode::Char(char) => app.cmd_buffer.push(char),
            KeyCode::Backspace => _ = app.cmd_buffer.pop(),
            KeyCode::Enter => cmd_line::execute(app),
            KeyCode::Esc => app.switch_mode(Mode::Normal),
            _ => {}
        },
    }
}
