use std::collections::VecDeque;
use ttf_parser::{Face, OutlineBuilder};
use crate::glyph_drawer::DrawCommand::{Close, CurveTo, LineTo, MoveTo, QuadTo};
use crate::pen_connection::PlotterError;
use crate::pen_plotter::PenPlotter;

enum DrawCommand {
	MoveTo(f32, f32),
	LineTo(f32, f32),
	QuadTo(f32, f32, f32, f32),
	CurveTo(f32, f32, f32, f32, f32, f32),
	Close,
}

impl OutlineBuilder for DrawQueue {
	fn move_to(&mut self, x: f32, y: f32) { self.queue.push(MoveTo(x, y)); }
	fn line_to(&mut self, x: f32, y: f32) { self.queue.push(LineTo(x, y)); }
	fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) { self.queue.push(QuadTo(x1, y1, x, y)); }
	fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
		self.queue.push(CurveTo(x1, y1, x2, y2, x, y));
	}
	fn close(&mut self) { self.queue.push(Close); }
}

fn glyph_svg(face: &Face, c: char) -> Option<DrawQueue> {
	let id = face.glyph_index(c)?;
	let mut path = DrawQueue { queue: vec![], width: 200 };
	let r = face.outline_glyph(id, &mut path)?;
	path.width = r.x_max as i32;
	Some(path)
}

pub struct DrawQueue {
	queue: Vec<DrawCommand>,
	width: i32,
}

pub struct GlyphDrawer {
	face: Face<'static>,
	queue: VecDeque<DrawQueue>,
}

impl GlyphDrawer {
	pub fn new() -> Self {
		let data = std::fs::read("font/jetbrains.ttf").unwrap().leak();
		let face = Face::parse(data, 0).unwrap();

		Self { face, queue: vec![].into() }
	}

	pub fn queue(&mut self, char: char) {
		let queue = glyph_svg(&self.face, char).unwrap();
		self.queue.push_back(queue);
	}

	pub fn plot_next(&mut self, pen_plotter: &mut PenPlotter) -> Result<(), PlotterError> {
		if let Some(queue) = self.queue.pop_front() {
			for command in queue.queue {
				match command {
					MoveTo(x, y) => pen_plotter.move_to(x as i32, y as i32)?,
					LineTo(x, y) => pen_plotter.line_to(x as i32, y as i32)?,
					QuadTo(x1, y1, x, y) => pen_plotter.quad_to(x1 as i32, y1 as i32, x as i32, y as i32)?,
					CurveTo(x1, y1, x2, y2, x, y) => pen_plotter.curve_to(x1 as i32, y1 as i32, x2 as i32, y2 as i32, x as i32, y as i32)?,
					Close => pen_plotter.close()?,
				}
			}
			pen_plotter.new_char(queue.width)?;
		}
		Ok(())
	}
}