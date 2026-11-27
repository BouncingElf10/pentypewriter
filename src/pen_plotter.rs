use crate::pen_connection::{PenConnection, PlotterError};

pub struct PenPlotter {
	connection: PenConnection,
	offset: (i32, i32),
	cursor: (i32, i32),
}

pub type Result<T> = std::result::Result<T, PlotterError>;

impl PenPlotter {
	pub fn new(connection: PenConnection) -> Self {
		Self { connection, offset: (0, 0), cursor: (0, 0) }
	}
	
	pub fn move_to(&mut self, x: i32, y: i32) -> Result<()> {
		self.pen_up()?;
		self.cursor = (x, y);
		self.step((x - self.cursor.0) + self.offset.0, (y - self.cursor.1) + self.offset.1, 200)?;
		Ok(())
	}
	
	pub fn line_to(&mut self, x: i32, y: i32) -> Result<()> {
		self.pen_down()?;
		self.cursor = (x, y);
		self.step((x - self.cursor.0) + self.offset.0, (y - self.cursor.1) + self.offset.1, 200)?;
		Ok(())
	}
	
	pub fn close(&mut self) -> Result<()> {
		self.move_to(0,0)?;
		Ok(())
	}
	
	pub fn new_char(&mut self, offset: i32) -> Result<()> {
		self.offset = (self.offset.0 + offset, self.offset.1);
		Ok(())
	}
	
	pub fn step(&mut self, step_x: i32, step_y: i32, milliseconds: i32) -> Result<()> {
		self.connection.send(format!("SM,{},{},{}", milliseconds, step_x, step_y).as_str())?;
		Ok(())
	}
	
	pub fn enable(&mut self) -> Result<()> {
		self.connection.send("EM,1,1")?;
		Ok(())
	}

	pub fn disable(&mut self) -> Result<()> {
		self.connection.send("EM,0,0")?;
		Ok(())
	}
	
	pub fn pen_down(&mut self) -> Result<()> {
		self.connection.send("SP,1")?;
		Ok(())
	}

	pub fn pen_up(&mut self) -> Result<()> {
		self.connection.send("SP,0")?;
		Ok(())
	}
}