pub mod editor;

use std::io;

use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};


pub enum Mode {
    Exit,
    Insert,
}


pub struct App {
    pub alive: bool,
    pub editor: editor::Editor,
    pub mode: Mode,
}

impl App {
    pub fn new(file: String) -> App {
        App {
            alive: true,
            editor: editor::Editor::new(file),
            mode: Mode::Insert,
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let Mode::Insert = self.mode {
            match key_event.code {
                KeyCode::Right => self.editor.move_cursor_right(1),
                KeyCode::Left => self.editor.move_cursor_left(1),
                KeyCode::Up => self.editor.move_cursor_up(1),
                KeyCode::Down => self.editor.move_cursor_down(1),
                KeyCode::Home => self.editor.move_cursor_to_start(),
                KeyCode::End => self.editor.move_cursor_to_end(),

                KeyCode::Enter => self.editor.split_line(),
                KeyCode::Backspace => self.editor.delete_char(),
                KeyCode::Char(' ') => self.editor.insert_char(" "),

                KeyCode::F(1) => self.editor.save_file(),
                KeyCode::Esc => self.mode = Mode::Exit,
                _ => self.editor.insert_char(&key_event.code.to_string())
            }
        }

        if let Mode::Exit = self.mode {
            match key_event.code {
                KeyCode::Char('y') => self.alive = false,
                KeyCode::Char('n') => self.mode = Mode::Insert,
                KeyCode::Char('s') => {
                    self.editor.save_file();
                    self.alive = false 
                },
                _ => {}
            }
        }
    }
}
