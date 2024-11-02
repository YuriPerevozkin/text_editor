use std::cell::RefCell;
use std::io::stdout;

use crossterm::{
    execute,
    cursor::SetCursorStyle,
};
use ratatui::{
    prelude::*,
    text::Line,
    style::Stylize,
    DefaultTerminal,
    symbols::border,
    layout::{
        Alignment,
        Position
    },
    widgets::{
        Block,
        Widget,
        Paragraph,
        block::Title,
    },
};

use crate::app::{
    App,
    Mode
};


struct EditorWidget<'a> {
    title: Title<'a>,
    content: Vec<String>,
}

struct PromptWidget {
}

pub struct Ui<'a> {
    terminal: DefaultTerminal,
    app: &'a RefCell<App>,
}


impl<'a> Ui<'a> {
    pub fn new(terminal: DefaultTerminal, app: &'a RefCell<App>) -> Ui<'a> {
        Ui {
            terminal,
            app
        }
    }

    pub fn draw_app(&mut self) {
        let editor_widget = EditorWidget {
            title: Title::from(Line::from(self.app.borrow().editor.file.to_string().bold().blue())),
            content: self.app.borrow().editor.buffer.clone(),
        };
        let prompt_widget = PromptWidget {};

        self.set_cursor_style();

        self.terminal.draw(|f| {
            f.render_widget(editor_widget, f.area());
            //f.render_widget(prompt_widget, chunks[1]);
            f.set_cursor_position(Position::new(
                <usize as TryInto<u16>>::try_into(self.app.borrow().editor.cursor.pos
                    +len_of_int(self.app.borrow().editor.buffer.len())+2).unwrap(),
                <usize as TryInto<u16>>::try_into(self.app.borrow().editor.cursor.line+1).unwrap(),
            ));
        }).unwrap();
    }

    fn set_cursor_style(&self) {
        let cursor_style = match self.app.borrow().mode {
            Mode::Normal => SetCursorStyle::SteadyBlock,
            _ => SetCursorStyle::SteadyBar,
        };

        execute!(
            stdout(),
            cursor_style,
        ).unwrap();
    }
}

impl Widget for EditorWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(self.title.alignment(Alignment::Left))
            .border_set(border::ROUNDED);

        let mut content = String::new();

        for (i, item) in self.content.iter().enumerate() {
            let space_number = len_of_int(self.content.len()) - len_of_int(i+1);
            content.push_str(format!("{}{} {}\n", " ".repeat(space_number), i+1, item).as_str())
        }

        Paragraph::new(content)
            .block(block)
            .render(area, buf);
    }
}

impl Widget for PromptWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        Paragraph::new(":")
            .block(block)
            .render(area, buf)
    }
}

fn len_of_int(n: usize) -> usize {
    n.to_string().len()
}
