use std::io::stdout;

use crate::app::{
    App,
    Mode
};
use crossterm::{
    execute,
    cursor::SetCursorStyle,
};
use ratatui::{
    prelude::*, 
    layout::{Flex, Position},
    symbols::border,
    widgets::{Block, Paragraph},
};


const LINE_NUMBER_OFFSET: usize = 5;
const CURSOR_OFFSET: usize = LINE_NUMBER_OFFSET + 2;


pub fn draw_app(app: &mut App) {
    set_cursor_style(app);

    app.terminal.draw(|f| {
        render_editor(f.area(), f.buffer_mut(), &app.editor.buffer);

        f.set_cursor_position(Position::new(
            (app.editor.cursor.pos + CURSOR_OFFSET) as u16,
            (app.editor.cursor.line + 1) as u16,
        ));

        if app.mode == Mode::Command {
            render_command_line(f.area(), f.buffer_mut(), &app.cmd_buffer);
        }
    }).unwrap();
}


fn render_editor(area: Rect, buf: &mut Buffer, file_content: &Vec<String>) {
    let block = Block::bordered()
        .border_set(border::ROUNDED);

    let mut content = String::new();

    for (i, item) in file_content.iter().enumerate() {
        let space_number = LINE_NUMBER_OFFSET - int_len(i + 1);
        content = format!("{}{}{} {}\n", content, " ".repeat(space_number), i + 1, item)
    }

    Paragraph::new(content)
        .block(block)
        .render(area, buf);
}

fn render_command_line(area: Rect, buf: &mut Buffer, content: &String) {
    let block = Block::new();

    let [area] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::End)
        .areas(area);

    Paragraph::new(format!(":{}", content))
        .block(block)
        .render(area, buf)
}

fn set_cursor_style(app: &mut App) {
    let cursor_style = match app.mode {
        Mode::Normal => SetCursorStyle::SteadyBlock,
        _ => SetCursorStyle::SteadyBar,
    };

    execute!(stdout(), cursor_style).unwrap();
}

fn int_len(n: usize) -> usize {
    ((n as f32).log10() + 1.0) as usize
}
