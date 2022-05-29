use ggez::graphics::{Text, Font, self};
use ggez::{Context, GameResult};

use crate::utils::Vector2D;

static FONT_PATH: &str = "/fonts/RobotoMono-VariableFont_wght.ttf";


pub enum TerminalOp {
    Add { val: String },
    Clear,
    Enter,
    None,
    Backspace,
}

pub struct TerminalInput {
    text: String
}
impl TerminalInput {
    pub fn new() -> Self {
        Self {text: String::new()}
    }

    pub fn add(&mut self, value: String) {
        self.text.push_str(&value.to_lowercase())
    }

    pub fn reset(&mut self) {
        self.text.clear()
    }

    pub fn backspace(&mut self) {
        if self.text.len() > 0 {
            self.text.pop().unwrap();
        }
    }

    fn build(&self, ctx: &mut Context) -> GameResult<Text> {
        let font = Font::new(ctx, FONT_PATH)?;
        let mut body = String::from("<admin-001> $ ");
        body.push_str(&self.text);
        body.push_str("|");
        let text = Text::new((body.to_owned(), font, 18.0));

        Ok(text)
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        let text = self.build(ctx)?;
        let dest_point = Vector2D::new(20.0, 530.0);

        graphics::draw(ctx, &text, (dest_point.as_vec(), ))
    }

    pub fn update(&mut self, op: TerminalOp) {
        match op {
            TerminalOp::Add { val } => self.add(val),
            TerminalOp::Clear => self.reset(),
            TerminalOp::Enter => todo!(),
            TerminalOp::Backspace => self.backspace(),
            _ => println!("Meh...")
        }
    }

}