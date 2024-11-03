use std::fs;
use std::io::Error;


#[derive(Default)]
pub struct Cursor {
    pub line: usize,
    pub pos: usize,
}

pub struct Editor {
    pub file: String,
    pub buffer: Vec<String>,
    pub cursor: Cursor,
    pub cursor_cache: usize,
}

impl Editor {
    pub fn new(file: String) -> Result<Editor, Error> {
        let buffer = fs::read_to_string(&file)?.lines().map(str::to_string).collect();

        Ok(Editor {
            file,
            buffer,
            cursor: Cursor::default(),
            cursor_cache: 0,
        })
    }

    pub fn move_cursor_right(&mut self, n: usize) {
        if self.cursor.pos != self.buffer[self.cursor.line].len() {
            self.cursor.pos += n;
            self.cache_cursor()
        }
    }

    pub fn move_cursor_left(&mut self, n: usize) {
        if self.cursor.pos != 0 {
            self.cursor.pos -= n;
            self.cache_cursor()
        }
    }

    pub fn move_cursor_up(&mut self, n: usize) {
        if self.cursor.line != 0 {
            self.cursor.line -= n;

            self.cursor.pos = self.cursor_cache;

            if self.cursor.pos > self.buffer[self.cursor.line].len() {
                self.move_cursor_to_end()
            }
        }
    }

    pub fn move_cursor_down(&mut self, n: usize) {
        if self.cursor.line != self.buffer.len()-1 {
            self.cursor.line += n;

            self.cursor.pos = self.cursor_cache;

            if self.cursor.pos > self.buffer[self.cursor.line].len() {
                self.move_cursor_to_end()
            }
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor.pos = 0;
        self.cache_cursor()
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor.pos = self.buffer[self.cursor.line].len();
    }

    pub fn insert_char(&mut self, char: char) {
        self.buffer[self.cursor.line].insert(self.cursor.pos, char);
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
        self.cache_cursor();
        self.buffer[self.cursor.line] += &deleted_line;
    }

    pub fn split_line(&mut self) {
        let splited = self.buffer[self.cursor.line].split_at(self.cursor.pos);
        let first_line = splited.0.to_string();
        let second_line = splited.1.to_string();

        self.buffer[self.cursor.line] = first_line;
        self.buffer.insert(self.cursor.line+1, second_line);
        self.move_cursor_down(1);
        self.move_cursor_to_start();
    }

    pub fn cache_cursor(&mut self) {
        self.cursor_cache = self.cursor.pos
    }

    pub fn save_file(&self) -> Result<(), Error> {
        fs::write(&self.file, self.buffer.join("\n")+"\n")
    }
}
