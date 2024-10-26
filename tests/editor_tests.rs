use text_editor::app::editor::Editor;


#[test]
fn move_cursor_right() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.move_cursor_right(1);
    assert_eq!(editor.cursor.pos, 1);

    editor.cursor.pos = 4;
    editor.move_cursor_right(1);
    assert_eq!(editor.cursor.pos, 4);
}

#[test]
fn move_cursor_left() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.move_cursor_left(1);
    assert_eq!(editor.cursor.pos, 0);

    editor.cursor.pos = 4;
    editor.move_cursor_left(1);

    assert_eq!(editor.cursor.pos, 3);
}

#[test]
fn move_cursor_up() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.move_cursor_up(1);
    assert_eq!(editor.cursor.line, 0);

    editor.cursor.line = 5;
    editor.move_cursor_up(1);
    assert_eq!(editor.cursor.line, 4);
}

#[test]
fn move_cursor_down() {
    let mut editor = Editor::new("tests/test".to_string());
    
    editor.move_cursor_down(1);
    assert_eq!(editor.cursor.line, 1);

    editor.cursor.line = 9;
    editor.move_cursor_down(1);
    assert_eq!(editor.cursor.line, 9);
}

#[test]
fn move_cursor_to_start() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.cursor.pos = 3;
    editor.move_cursor_to_start();
    assert_eq!(editor.cursor.pos, 0);

    editor.move_cursor_to_start();
    assert_eq!(editor.cursor.pos, 0);
}

#[test]
fn move_cursor_to_end() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.move_cursor_to_end();
    assert_eq!(editor.cursor.pos, 4);

    editor.move_cursor_to_end();
    assert_eq!(editor.cursor.pos, 4);
}

#[test]
fn delete_char() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.cursor.pos = 2;

    editor.delete_char();
    assert_eq!(editor.buffer[editor.cursor.line], "tst");

    editor.delete_char();
    assert_eq!(editor.buffer[editor.cursor.line], "st");
}

#[test]
fn insert_char() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.insert_char(" ");
    assert_eq!(editor.buffer[editor.cursor.line], " test");

    editor.insert_char("1");
    assert_eq!(editor.buffer[editor.cursor.line], " 1test");
}

#[test]
fn join_lines() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.delete_char();
    assert_eq!(editor.buffer[editor.cursor.line], "test");

    editor.cursor.line = 1;

    editor.delete_char();
    assert_eq!(editor.buffer[editor.cursor.line], "testfile");
}

#[test]
fn split_line() {
    let mut editor = Editor::new("tests/test".to_string());

    editor.cursor.pos = 2;
    editor.split_line();
    assert_eq!(editor.buffer[editor.cursor.line], "st")
}
