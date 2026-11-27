use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use crate::pen_connection::PlotterError;

pub struct Tui {
	terminal: DefaultTerminal,
	chars: Arc<Mutex<(VecDeque<char>, VecDeque<char>)>>,
}

impl Tui {
	pub fn new(chars: Arc<Mutex<(VecDeque<char>, VecDeque<char>)>>) -> Self {
		let terminal = ratatui::init();
		ratatui::restore();
		Self { terminal, chars }
	}

	pub fn run(&mut self, reciever: mpsc::Receiver<PlotterError>) -> std::io::Result<()> {
		loop {
			if let Ok(err) = reciever.try_recv() {
				eprintln!("error: {err}");
				panic!()
			}

			self.terminal.draw(|frame| {
				let [main_area, help_area] =
					Layout::vertical([Constraint::Min(3), Constraint::Length(1)])
						.areas(frame.area());

				let spans: Vec<Span> = {
					let chars = self.chars.lock().unwrap();
					let matched = chars.0.iter().zip(chars.1.iter())
						.take_while(|(a, b)| a == b)
						.count();
					chars.0.iter().enumerate().map(|(i, c)| {
						let span = Span::raw(c.to_string());
						if i < matched {
							span.green()
						} else if i == matched {
							span.yellow()
						} else {
							span.white()
						}
					}).collect()
				};
				let input_box = Paragraph::new(Line::from(spans))
					.block(Block::bordered().title(" Typed ".bold()));

				let help = Paragraph::new("    Backspace: delete   Esc / Ctrl+C: quit").dark_gray();

				frame.render_widget(input_box, main_area);
				frame.render_widget(help, help_area);
			})?;

			if event::poll(Duration::from_millis(16))? {
				if let Event::Key(key) = event::read()? {
					if key.kind != KeyEventKind::Press {
						continue;
					}
					match key.code {
						KeyCode::Esc => return Ok(()),
						KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
							return Ok(())
						}
						KeyCode::Char(c) => {
							self.chars.lock().unwrap().0.push_back(c);
						},
						KeyCode::Backspace => {
							let mut chars = self.chars.lock().unwrap();
							if chars.0.len() > chars.1.len() + 1 {
								chars.0.pop_back();
							}
						}
						_ => {}
					}
				}
			}
		}
	}
}