use text_editor::app::{
    App,
    Mode,
};

use crossterm::event::KeyCode;


#[test]
fn handle_key_event_right() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Char('l').into());
    assert_eq!(app.editor.cursor.pos, 1);
}

#[test]
fn handle_key_event_left() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.pos = 2;

    app.handle_key_event(KeyCode::Char('h').into());
    assert_eq!(app.editor.cursor.pos, 1);
}

#[test]
fn handle_key_event_down() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Char('j').into());
    assert_eq!(app.editor.cursor.line, 1);
}

#[test]
fn handle_key_event_up() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.line = 1;

    app.handle_key_event(KeyCode::Char('k').into());
    assert_eq!(app.editor.cursor.line, 0);
}

#[test]
fn handle_key_event_home() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.pos = 2;

    app.handle_key_event(KeyCode::Char('0').into());
    assert_eq!(app.editor.cursor.pos, 0);
}

#[test]
fn handle_key_event_end() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Char('$').into());
    assert_eq!(app.editor.cursor.pos, 4);
}

#[test]
fn handle_key_event_insert_mode() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Char('i').into());
    assert_eq!(app.mode, Mode::Insert);
}
