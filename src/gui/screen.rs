use std::time::Duration;

use ggez::graphics::{Font, Text, Color};
use ggez::{graphics, Context, GameResult, timer};

use super::edge::Edge;
use crate::utils::Vector2D;

// TODO: Implement this
pub struct Screen {
    edges: Box<Vec<Edge>>,
    font: Font,

    cursor: Vector2D,
    some_text: String,
    done: bool,
    text_cursor: usize,
}

impl Screen {
    pub fn new(edges: Box<Vec<Edge>>, font: Font) -> Self {
        Self {
            edges,
            cursor: Vector2D::new(40.0, 40.0),
            some_text: "This is some text".to_string(),
            done: false,
            text_cursor: 0,
            font,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.text_cursor >= self.some_text.len() - 1{
            self.done = true;
            return Ok(());
        }

        self.text_cursor += 1;
        self.cursor += Vector2D::new(10.0, 0.0);
        println!("C-update: {:?}", self.cursor);
        Ok(())
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        if self.done {
            return Ok(());
        }

        let current_letter = self.some_text.chars().collect::<Vec<char>>()[self.text_cursor];
        let letter = Text::new((current_letter, self.font, 18.0));
        println!("L: {}, C: {:?}", current_letter, self.cursor);
        graphics::draw(ctx, &letter, (self.cursor.as_vec(), Color::WHITE))
    }
}
