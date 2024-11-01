use std::cell::RefCell;

use ratatui::{
    text::Line,
    style::Stylize,
    DefaultTerminal,
    symbols::border,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{
        block::{Position, Title},
        Block, Paragraph,
    },
};

use crate::app::{
    App,
    Mode,
};


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
        match self.app.borrow().mode {
            Mode::Exit => self.draw_exit_popup(),
            _ => self.draw_editor(),
        }
    }

    fn draw_editor(&mut self) {
        let title = Title::from(Line::from(self.app.borrow().editor.file.to_string().bold()));
        let mode = &self.app.borrow().mode;

        let instructions = Title::from(Line::from(vec![
            "[Save ".into(),
            "F1".blue().bold(),
            " Exit ".into(),
            "ESC".blue().bold(),
            "]".into(),
            format!("{}", mode).into(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Left))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .border_set(border::ROUNDED);

        let content = self.app.borrow().editor.buffer.join("\n");

        let paragraph = Paragraph::new(content)
            .block(block);

        self.terminal.draw(|f| {
            f.render_widget(paragraph, f.area());
            f.set_cursor_position(ratatui::layout::Position::new(
                <usize as TryInto<u16>>::try_into(self.app.borrow().editor.cursor.pos).unwrap()+1,
                <usize as TryInto<u16>>::try_into(self.app.borrow().editor.cursor.line).unwrap()+1,
            ));
        }).unwrap();
    }

    pub fn draw_exit_popup(&mut self) {
        let instructions = Title::from(Line::from(vec![
            "[".into(),
            "Y".blue().bold(),
            "es ".into(),
            "N".blue().bold(),
            "o ".into(),
            "S".blue().bold(),
            "ave&quit]".into(),
        ]));

        let exit_block = Block::bordered()
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .border_set(border::ROUNDED);


        let paragraph = Paragraph::new("Exit without saving?")
            .block(exit_block)
            .alignment(Alignment::Center);

        self.terminal.draw(|f| {
            let area = centered_rect(13, 5, f.area());
            f.render_widget(paragraph, area)
        });
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
