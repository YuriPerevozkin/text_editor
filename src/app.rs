mod cmd_handler;
mod cmd_line;
mod editor;

use crate::ui;

use ratatui::DefaultTerminal;

use crossterm::event::{self, Event, KeyEventKind};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

pub struct App {
    pub mode: Mode,
    pub alive: bool,
    pub terminal: DefaultTerminal,
    pub editor: editor::Editor,
    pub cmd_buffer: String,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Command => write!(f, "COMMAND"),
        }
    }
}

impl App {
    pub fn new(terminal: DefaultTerminal, file: String) -> Self {
        let editor = editor::Editor::new(file);

        Self {
            mode: Mode::Normal,
            alive: true,
            terminal,
            editor,
            cmd_buffer: "".to_string(),
        }
    }

    pub fn run(&mut self) {
        while self.alive {
            ui::draw_app(self);

            match event::read().unwrap() {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    cmd_handler::handle_command(self, key_event);
                }
                _ => {}
            }
        }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        if self.mode == Mode::Command {
            self.cmd_buffer.clear()
        }
        self.mode = mode
    }
}
