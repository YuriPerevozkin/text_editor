use text_editor::app::App;

use crossterm::event::KeyCode;


#[test]
fn handle_key_event_right() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Right.into());
    assert_eq!(app.editor.cursor.pos, 1);
}

#[test]
fn handle_key_event_left() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.pos = 2;

    app.handle_key_event(KeyCode::Left.into());
    assert_eq!(app.editor.cursor.pos, 1);
}

#[test]
fn handle_key_event_down() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::Down.into());
    assert_eq!(app.editor.cursor.line, 1);
}

#[test]
fn handle_key_event_up() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.line = 1;

    app.handle_key_event(KeyCode::Up.into());
    assert_eq!(app.editor.cursor.line, 0);
}

#[test]
fn handle_key_event_home() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.pos = 2;

    app.handle_key_event(KeyCode::Home.into());
    assert_eq!(app.editor.cursor.pos, 0);
}

#[test]
fn handle_key_event_end() {
    let mut app = App::new("tests/test".to_string());

    app.handle_key_event(KeyCode::End.into());
    assert_eq!(app.editor.cursor.pos, 4);
}

#[test]
fn handle_key_event_backspace() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.line = 1;

    app.handle_key_event(KeyCode::Backspace.into());
    assert_eq!(app.editor.buffer[app.editor.cursor.line], "testfile");
}

#[test]
fn handle_key_event_enter() {
    let mut app = App::new("tests/test".to_string());

    app.editor.cursor.pos = 2;
    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(app.editor.buffer[app.editor.cursor.line], "st");
}

#[test]
fn handle_key_event_char() {
    let mut app = App::new("tests/test".to_string());

    app.editor.insert_char("a");
    assert_eq!(app.editor.buffer[app.editor.cursor.line], "atest");

    app.editor.insert_char(" ");
    assert_eq!(app.editor.buffer[app.editor.cursor.line], "a test");
}
