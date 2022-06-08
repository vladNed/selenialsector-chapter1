use ggez::{
    graphics::{self, Color, Font, Text},
    Context, GameResult,
};

use super::Point2D;

type Line = (Point2D, String);

/// Iterable text data source.
///
/// Can be loaded with a text that will be split into chars and words.
/// Each item returned represents a `(next_char, word_size)`.
///
/// The `next_char` and `word_size` can be further used to create the
/// text scrolling effect. Works perfectly with a `TextBox`.
///
/// The queue will automatically be cleared once the last item in is
/// returned by the iterator.
#[derive(Clone, Debug)]
pub struct TextQueue {
    /// Raw data received when loading
    data: String,

    /// Reading position of the text queue
    cursor: usize,

    /// Data transformed as chars
    chars: Vec<char>,

    /// Shows if the text queue is empty
    is_empty: bool,

    /// Text data split into words
    words: Vec<String>,

    /// Index position in the list of words
    words_cursor: usize,
}

impl TextQueue {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            chars: Vec::new(),
            cursor: usize::MIN,
            is_empty: true,
            words: Vec::new(),
            words_cursor: usize::MIN,
        }
    }

    /// Load text data that will be split into chars and words.
    pub fn load(&mut self, data: String) {
        self.data = data;
        self.chars = self.data.chars().collect::<Vec<char>>();
        self.is_empty = false;
        self.words = self
            .data
            .split(" ")
            .map(|w| w.to_string())
            .collect::<Vec<String>>();
        self.words_cursor = usize::MIN;
    }

    /// Clears the entire text queue. Would be returned to default.
    pub fn clear(&mut self) {
        self.data.clear();
        self.chars.clear();
        self.is_empty = true;
        self.cursor = usize::MIN;
        self.words_cursor = usize::MIN;
    }
}
impl Iterator for TextQueue {
    type Item = (char, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            return None;
        }

        if self.cursor >= self.chars.len() && self.cursor > usize::MIN {
            self.clear();
            return None;
        }

        let next_char = self.chars[self.cursor];

        let word_size: usize = match next_char.is_whitespace() {
            true => {
                self.words_cursor += 1;
                usize::MIN
            },
            false => self.words[self.words_cursor].len()
        };
        self.cursor += 1;
        Some((next_char, word_size))
    }
}

#[derive(Clone, Debug)]
pub struct TextBox {
    /// Represents the 2D position on the display
    pub display_cursor: Point2D,

    /// Represents the lines of the text
    pub lines: Vec<Line>,

    /// Width of the textbox in pixels
    width: f32,

    /// Represents the line position in the lines array
    lines_cursor: usize,


    /// Line length index
    current_line_length: f32,

    /// Font for the textbox
    font: Font,
    font_size: f32,

    /// Text space representing a checkpoint to calculate text justify
    checkpoint: f32,

    /// Iterable data source for text
    pub text_queue: TextQueue
}

impl TextBox {
    pub fn new(
        width: f32,
        font: Font,
        font_size: f32,
        text: String,
        start_vector: Point2D
    ) -> Self {
        let default_line: Line = (start_vector, String::new());
        let mut text_queue = TextQueue::new();
        text_queue.load(text);

        Self {
            width,
            lines_cursor: usize::MIN,
            display_cursor: start_vector,
            current_line_length: 0.0,
            lines: vec![default_line],
            font,
            font_size,
            checkpoint: 0.0,
            text_queue
        }
    }

    pub fn new_line(&mut self) {
        self.display_cursor += Point2D::new(0.0, self.font_size);
        self.lines.push((self.display_cursor, String::new()));
        self.lines_cursor += 1;
        self.current_line_length = 0 as f32;
        self.checkpoint = 0.0;
    }

    pub fn _blank_line(&mut self) {
        self.new_line();
        self.new_line();
    }

    pub fn update(&mut self) -> Option<()>{
        let (next_char, word_size) = match self.text_queue.next() {
            Some(res) => res,
            None => return None
        };
        self.add_checkpoint(word_size);
        self.add_new_line_if_needed(word_size);

        self.lines[self.lines_cursor].1.push(next_char);
        self.current_line_length += 10.0;

        Some(())
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        for (cursor, line) in self.lines.iter() {
            let tx = Text::new((line.clone(), self.font, self.font_size));
            graphics::draw(ctx, &tx, (cursor.as_vec(), Color::WHITE))?;
        }
        Ok(())
    }

    fn add_new_line_if_needed(&mut self, word_size: usize) {
        if self.current_line_length > self.width {
            self.new_line();
            return
        }

        let sent_length = self.checkpoint + (10.0 * word_size as f32);
        if sent_length > self.width {
            self.new_line();
        }
    }

    fn add_checkpoint(&mut self, word_size: usize) {
        if word_size == 0 {
            self.checkpoint = self.current_line_length;
        }
    }
}
