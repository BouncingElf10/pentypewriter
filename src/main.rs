mod pen_connection;
mod pen_plotter;
mod glyph_drawer;
mod tui;

use std::collections::VecDeque;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use pen_connection::{PenConnection, Result};
use crate::glyph_drawer::GlyphDrawer;
use crate::pen_connection::PlotterError;
use crate::pen_plotter::PenPlotter;
use crate::tui::Tui;

fn main() -> io::Result<()> {
    let chars = Arc::new(Mutex::new((VecDeque::new(), VecDeque::new())));
    let plot_chars = Arc::clone(&chars);
    let (sender, receiver) = mpsc::channel::<PlotterError>();

    thread::spawn(move || {
        if let Err(e) = plot_loop(plot_chars) {
            sender.send(e).unwrap();
        }
    });

    Tui::new(chars).run(receiver)
}

fn plot_loop(chars: Arc<Mutex<(VecDeque<char>, VecDeque<char>)>>) -> Result<()> {
    let plotter = PenConnection::connect()?;
    let mut pen_plotter = PenPlotter::new(plotter);
    let mut glyph_drawer = GlyphDrawer::new();

    loop {
        let next = chars.lock().unwrap().0.pop_front();
        match next {
            Some(c) => {
                glyph_drawer.queue(c);
                glyph_drawer.plot_next(&mut pen_plotter)?;
                chars.lock().unwrap().1.push_back(c);
            }
            None => thread::sleep(Duration::from_millis(20)),
        }
    }
}
