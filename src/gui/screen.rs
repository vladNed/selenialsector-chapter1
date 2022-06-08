use std::cell::RefCell;
use std::rc::Rc;

use ggez::event::{quit, KeyCode};
use ggez::graphics::Font;
use ggez::{Context, GameResult};

use super::edge::{Edge, Vector};

use crate::dialogue::Dialogue;
use crate::utils::Point2D;

static INNER_SCREEN_MARGIN: Point2D = Point2D { x: 10.0, y: 10.0 };
static SCREEN_FONT_SIZE: f32 = 18.0;

type DialogueHistory = Vec<Rc<RefCell<Dialogue>>>;

pub struct Screen {
    _edges: Box<Vec<Edge>>,
    dialogue_history: DialogueHistory,
    current_dialogue: Rc<RefCell<Dialogue>>,
}

impl Screen {
    pub fn new(edges: Box<Vec<Edge>>, font: Font) -> Self {
        let screen_anchor = match edges.first() {
            Some(edge) => *edge.get_start() + INNER_SCREEN_MARGIN,
            None => panic!("Screen edges are not set."),
        };
        let action2 = Rc::new(RefCell::new(Dialogue::new(
            "Nice you chose the only one we have".to_string(),
            None,
            Some(KeyCode::M),
            1200.0,
            Some(font),
            SCREEN_FONT_SIZE,
            Some(screen_anchor),
        )));

        let action1 = Rc::new(RefCell::new(Dialogue::new(
            "What do you choose ? Milk or Tea ?".to_string(),
            Some(vec![action2]),
            None,
            1200.0,
            Some(font),
            SCREEN_FONT_SIZE,
            Some(screen_anchor),
        )));

        Self {
            _edges: edges,
            current_dialogue: action1,
            dialogue_history: vec![],
        }
    }

    pub fn update(&mut self, ctx: &mut Context, user_input: Option<KeyCode>) -> GameResult {
        // Check if the whole text was parsed and displayed
        let new_dialogue_ref = Rc::clone(&self.current_dialogue);
        let mut current_dialogue = new_dialogue_ref.borrow_mut();
        if !current_dialogue.has_text_updated {
            current_dialogue.update();
            return Ok(());
        }

        let user_input_event = match user_input {
            Some(i) => i,
            None => KeyCode::Escape,
        };

        // Check user input to advance the dialogue
        let next_dialogue_result = current_dialogue.advance(user_input_event);
        if next_dialogue_result.0 {
            match next_dialogue_result.1 {
                Some(new_dialogue) => {
                    let mut last_display_cursor = current_dialogue.get_last_display_pos();
                    last_display_cursor += Point2D::new(0.0, 18.0);

                    self.dialogue_history
                        .push(Rc::clone(&self.current_dialogue));
                    self.current_dialogue = Rc::clone(&new_dialogue);
                    self.current_dialogue
                        .borrow_mut()
                        .set_position(last_display_cursor);
                }
                None => quit(ctx)
            }
        }

        Ok(())
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        for dialogue in self.dialogue_history.iter() {
            dialogue.borrow().display(ctx)?
        }
        self.current_dialogue.borrow().display(ctx)?;
        Ok(())
    }
}
