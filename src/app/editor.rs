use std::fs;


pub struct Cursor {
    pub line: usize,
    pub pos: usize,
}

pub struct Editor {
    pub file: String,
    pub buffer: Vec<String>,
    pub cursor: Cursor,
}

impl Editor {
    pub fn new(file: String) -> Editor {
        let file_content = fs::read_to_string(&file).unwrap();
        let buffer = file_content.lines().map(str::to_string).collect();

        Editor {
            file,
            buffer,
            cursor: Cursor {
                line: 0,
                pos: 0,
            }
        }
    }

    pub fn move_cursor_right(&mut self, n: usize) {
        if self.cursor.pos != self.buffer[self.cursor.line].len() {
            self.cursor.pos += n;
        }
    }

    pub fn move_cursor_left(&mut self, n: usize) {
        if self.cursor.pos != 0 {
            self.cursor.pos -= n;
        }
    }

    pub fn move_cursor_up(&mut self, n: usize) {
        if self.cursor.line != 0 {
            self.cursor.line -= n;
            if self.cursor.pos > self.buffer[self.cursor.line].len() {
                self.move_cursor_to_end()
            }
        }
    }

    pub fn move_cursor_down(&mut self, n: usize) {
        if self.cursor.line+1 != self.buffer.len() {
            self.cursor.line += n;
            if self.cursor.pos > self.buffer[self.cursor.line].len() {
                self.move_cursor_to_end()
            }
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor.pos = 0
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor.pos = self.buffer[self.cursor.line].len()
    }

    pub fn insert_char(&mut self, char: &str) {
        self.buffer[self.cursor.line].insert_str(self.cursor.pos, char);
        self.move_cursor_right(1)
    }

    pub fn delete_char(&mut self) {
        if self.cursor.pos > 0 {
            self.buffer[self.cursor.line].remove(self.cursor.pos-1);
            self.move_cursor_left(1)
        }
        else if self.cursor.line > 0 {
            self.join_lines()
        }
    }

    fn join_lines(&mut self) {
        let deleted_line = self.buffer.remove(self.cursor.line);
        self.move_cursor_up(1);
        self.move_cursor_to_end();
        self.buffer[self.cursor.line] += &deleted_line;
    }

    pub fn split_line(&mut self) {
        let splited = self.buffer[self.cursor.line].split_at(self.cursor.pos);
        let first_line = splited.0.to_string();
        let second_line = splited.1.to_string();

        self.buffer.insert(self.cursor.line+1, second_line);
        self.buffer[self.cursor.line] = first_line;
        self.move_cursor_down(1);
        self.move_cursor_to_start();
    }

    pub fn save_file(&self) {
        fs::write(self.file.as_str(), self.buffer.join("\n")+"\n").unwrap()
    }
}