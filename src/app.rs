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
    Insert,
    Normal,
    Command,
}

pub struct App {
    pub mode: Mode,
    pub alive: bool,
    pub editor: editor::Editor,
    pub command_line: String,
}

impl App {
    pub fn new(file: String) -> App {
        // TODO: add some error handling
        let editor = editor::Editor::new(file).unwrap();

        App {
            mode: Mode::Normal,
            alive: true,
            editor,
            command_line: "".to_string(),
        }
    }

    pub fn handle_events(&mut self) {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Insert => {
                match key_event.code {
                    KeyCode::Enter => self.editor.split_line(),
                    KeyCode::Backspace => self.editor.delete_char(),
                    KeyCode::Char(char) => self.editor.insert_char(char),

                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        return
                    },
                    _ => {}
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

                    KeyCode::Char(':') => {
                        self.mode = Mode::Command;
                        return
                    }
                    _ => {}
                }
            }
            
            Mode::Command => {
                match key_event.code {
                    KeyCode::Char(char) => self.command_line.push(char),
                    KeyCode::Backspace => _ = self.command_line.pop(),
                    KeyCode::Enter => {
                        match self.command_line.as_str() {
                            "q" => self.alive = false,
                            "w" => self.editor.save_file().unwrap(),
                            _ => {
                                self.mode = Mode::Normal;
                            }
                        }
                        self.command_line.clear();
                        self.mode = Mode::Normal;
                    },
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.command_line.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Command => write!(f, "Command")
        }
    }
}
