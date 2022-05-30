use ggez::graphics::{Mesh, MeshBuilder, Color, self, DrawParam};
use ggez::{Context, GameResult};

use crate::utils::Vector2D;

#[derive(Clone)]
/// Represents an edge of the gui screen
pub struct Edge {
    mesh: Mesh
}

impl Edge {
    pub fn new(x: Vector2D, y: Vector2D, ctx: &mut Context) -> GameResult<Edge> {
        let line = MeshBuilder::new()
            .line(&[x.as_vec(), y.as_vec()], 2.0, Color::WHITE)?
            .build(ctx)?;
        Ok(Self{mesh: line})
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        let draw_params = DrawParam::new();
        graphics::draw(ctx, &self.mesh, draw_params)
    }

}