mod pen_connection;
mod pen_plotter;
mod glyph_drawer;

use pen_connection::{PenConnection, Result};
use crate::glyph_drawer::GlyphDrawer;
use crate::pen_plotter::PenPlotter;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> Result<()> {
    let mut plotter = PenConnection::connect()?;
    println!("connected: {}", plotter.version()?);
    let mut pen_plotter = PenPlotter::new(plotter);

    let mut glyph_drawer = GlyphDrawer::new();

    glyph_drawer.queue('M');
    glyph_drawer.queue('B');

    glyph_drawer.plot_next(&mut pen_plotter)?;

    Ok(())
}
