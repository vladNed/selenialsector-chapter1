use ggez::event::KeyCode;
use ggez::graphics::{Text, Font, self, Color};
use ggez::{Context, GameResult};

use crate::utils::Point2D;

static FONT_PATH: &str = "/fonts/RobotoMono-VariableFont_wght.ttf";


// pub enum TerminalOp {
//     Add { val: String },
//     Clear,
//     Enter,
//     None,
//     Backspace,
// }

pub struct TerminalInput {
    text: String,
    font: Font,
    pub current_command: Option<KeyCode>
}
impl TerminalInput {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            text: String::new(),
            font: Font::new(ctx, FONT_PATH)?,
            current_command: None
        })
    }

    pub fn _add(&mut self, value: String) {
        self.text.push_str(&value.to_lowercase())
    }

    pub fn _reset(&mut self) {
        self.text.clear()
    }

    pub fn _backspace(&mut self) {
        if self.text.len() > 0 {
            self.text.pop().unwrap();
        }
    }

    fn build(&self, ctx: &mut Context) -> GameResult {
        let pos = Point2D::new(130.0, 530.0);
        let mut user_input_body = self.text.to_owned();
        user_input_body.push_str("_");
        let text = Text::new((user_input_body.to_owned(), self.font, 18.0));

        graphics::draw(ctx, &text, (pos.as_vec(), ))
    }

    fn draw_terminal_info(&self, ctx: &mut Context) -> GameResult {
        let pos = Point2D::new(20.0, 530.0);

        let term_info_body = String::from("<admin-001> $ ");
        let term_info_text = Text::new((term_info_body.to_owned(), self.font, 18.0));

        graphics::draw(ctx, &term_info_text, (pos.as_vec(), Color::MAGENTA))
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        self.build(ctx)?;
        self.draw_terminal_info(ctx)?;
        Ok(())
    }

    pub fn update(&mut self) {
        ()
    }

}