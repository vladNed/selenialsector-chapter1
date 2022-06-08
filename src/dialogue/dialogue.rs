use std::{cell::RefCell, rc::Rc};

use ggez::event::KeyCode;
use ggez::graphics::Font;
use ggez::{Context, GameResult};

use crate::utils::Point2D;
use crate::utils::TextBox;

type Decisions = Vec<Rc<RefCell<Dialogue>>>;
pub type CheckedDecision = (bool, Option<Rc<RefCell<Dialogue>>>);

#[derive(Debug)]
pub struct Dialogue {
    pub has_text_updated: bool,
    textbox: TextBox,
    pub start_position: Point2D,
    required_input: KeyCode,
    decisions: Option<Decisions>,
}
impl Dialogue {
    pub fn new(
        dialogue_text: String,
        decisions: Option<Decisions>,
        req_input: Option<KeyCode>,
        width: f32,
        dialogue_font: Option<Font>,
        font_size: f32,
        start_position: Option<Point2D>,
    ) -> Dialogue {
        let required_input = match req_input {
            Some(key) => key,
            None => KeyCode::Escape,
        };

        let font = match dialogue_font {
            Some(f) => f,
            None => Font::default(),
        };

        let start_vector = match start_position {
            Some(p) => p,
            None => Point2D::new(0.0, 0.0),
        };

        Dialogue {
            textbox: TextBox::new(width, font, font_size, dialogue_text, start_vector.clone()),
            required_input,
            decisions,
            start_position: start_vector,
            has_text_updated: false
        }
    }

    /// Increment position by x and y
    ///
    /// The new position will be the current position + the new point coordinates
    /// given as parameters.
    pub fn set_position(&mut self, new_pos: Point2D) {
        self.textbox.display_cursor = new_pos;
        self.start_position = new_pos;
        self.textbox.lines = vec![(self.start_position, "".to_string())];
    }

    /// Display method that displays to the game GUI the content value of the Dialogue
    /// using the display position
    pub fn display(&self, ctx: &mut Context) -> GameResult {
        self.textbox.display(ctx)
    }

    pub fn update(&mut self) {
        match self.textbox.update() {
            Some(_) => {
                self.has_text_updated = false
            },
            None => {
                self.has_text_updated = true
            }
        }
    }

    /// Based on the user input advances the dialogue to the next dialogue
    /// part of the conversation.
    pub fn advance(&self, user_input: KeyCode) -> CheckedDecision {
        let decisions = match &self.decisions {
            Some(d) => d,
            None => return (true, None),
        };

        let mut matches = false;
        let mut action_index = usize::MAX;
        for (pos, act) in decisions.iter().enumerate() {
            if act.borrow().required_input == user_input {
                matches = true;
                action_index = pos;
                break;
            }
        }

        if matches {
            return (true, Some(decisions[action_index].clone()));
        }

        return (false, None);
    }

    pub fn get_last_display_pos(&self) -> Point2D {
        match self.textbox.lines.last() {
            Some(line) => line.0,
            None => self.start_position
        }
    }
}
