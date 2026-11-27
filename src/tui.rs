use std::collections::VecDeque;
use std::str::Chars;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;

pub struct Tui {
	terminal: DefaultTerminal,
	chars: VecDeque<char>
}

impl Tui {
	pub fn new() -> Self {
		let terminal = ratatui::init();
		ratatui::restore();
		Self { terminal, chars: VecDeque::new() }
	}

	pub fn run(&mut self) -> std::io::Result<()> {
		loop {
			self.terminal.draw(|frame| {
				let [main_area, help_area] =
					Layout::vertical([Constraint::Min(3), Constraint::Length(1)])
						.areas(frame.area());

				let text: String = self.chars.iter().collect();
				let input_box = Paragraph::new(text)
					.block(Block::bordered().title(" Typed ".bold()))
					.white();

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
							self.chars.push_back(c);
						},
						KeyCode::Backspace => {
							self.chars.pop_back();
						}
						_ => {}
					}
				}
			}
		}
	}
}