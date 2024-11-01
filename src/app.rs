pub mod editor;

use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};


#[derive(Debug, PartialEq)]
pub enum Mode {
    Exit,
    Insert,
    Normal,
}

pub struct App {
    pub mode: Mode,
    pub alive: bool,
    pub editor: editor::Editor,
}

impl App {
    pub fn new(file: String) -> App {
        let editor = editor::Editor::new(file).unwrap();

        App {
            mode: Mode::Normal,
            alive: true,
            editor,
        }
    }

    pub fn handle_events(&mut self) {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Insert => {
                match key_event.code {
                    KeyCode::Enter => self.editor.split_line(),
                    KeyCode::Backspace => self.editor.delete_char(),
                    KeyCode::Char(' ') => self.editor.insert_char(" "),

                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        return
                    },
                    _ => self.editor.insert_char(&key_event.code.to_string())
                }
            }

            Mode::Normal => {
                match key_event.code {
                    KeyCode::Char('h') => self.editor.move_cursor_left(1),
                    KeyCode::Char('j') => self.editor.move_cursor_down(1),
                    KeyCode::Char('k') => self.editor.move_cursor_up(1),
                    KeyCode::Char('l') => self.editor.move_cursor_right(1),

                    KeyCode::Char('0') => self.editor.move_cursor_to_start(),
                    KeyCode::Char('$') => {
                        self.editor.move_cursor_to_end();
                        self.editor.cache_cursor();
                    }

                    KeyCode::Char('i') => {
                        self.mode = Mode::Insert;
                        return
                    }
                    KeyCode::Char('o') => {
                        self.editor.move_cursor_to_end();
                        self.editor.split_line();
                        self.mode = Mode::Insert;
                        return
                    }

                    KeyCode::F(1) => self.editor.save_file().unwrap(),
                    KeyCode::Esc => self.mode = Mode::Exit,
                    _ => ()
                }
            }

            Mode::Exit => {
                match key_event.code {
                    KeyCode::Char('y') => self.alive = false,
                    KeyCode::Char('n') => self.mode = Mode::Normal,
                    KeyCode::Char('s') => {
                        self.editor.save_file().unwrap();
                        self.alive = false 
                    },
                    _ => {}
                }
            }
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Normal => write!(f, "N"),
            Mode::Insert => write!(f, "I"),
            Mode::Exit => write!(f, "Exit"),
        }
    }
}
