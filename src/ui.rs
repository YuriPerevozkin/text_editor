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
    layout::{
        Alignment, Flex, Position
    }, prelude::*, style::Stylize, symbols::border, text::Line, widgets::{
        block::Title, Block, Paragraph, Widget
    },
};


struct EditorWidget<'a> {
    title: Title<'a>,
    content: Vec<String>,
}

struct CommandLineWidget {
    content: String
}

pub fn draw_app(app: &mut App) {
    let editor_widget = EditorWidget {
        title: Title::from(Line::from(app.editor.file.to_string().bold().blue())),
        content: app.editor.buffer.clone(),
    };
    set_cursor_style(app);

    app.terminal.draw(|f| {
        f.render_widget(editor_widget, f.area());
        f.set_cursor_position(Position::new(
                <usize as TryInto<u16>>::try_into(app.editor.cursor.pos
                    +len_of_int(app.editor.buffer.len())+2).unwrap(),
                    <usize as TryInto<u16>>::try_into(app.editor.cursor.line+1).unwrap(),
        ));
        if app.mode == Mode::Command {
            let command_line_widget = CommandLineWidget {
                content: app.cmd_buffer.clone()
            };
            f.render_widget(command_line_widget, f.area());
        }
    }).unwrap();
}

fn set_cursor_style(app: &mut App) {
    let cursor_style = match app.mode {
        Mode::Normal => SetCursorStyle::SteadyBlock, _ => SetCursorStyle::SteadyBar,
    };

    execute!(
        stdout(),
        cursor_style,
    ).unwrap();
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

impl Widget for CommandLineWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        let [area] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::End)
            .areas(area);

        Paragraph::new(format!(":{}",self.content))
            .block(block)
            .render(area, buf)
    }
}

fn len_of_int(n: usize) -> usize {
    n.to_string().len()
}
