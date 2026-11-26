use crate::pen_connection::{PenConnection, PlotterError};

pub struct PenPlotter {
	connection: PenConnection,
}

pub type Result<T> = std::result::Result<T, PlotterError>;

impl PenPlotter {
	pub fn new(connection: PenConnection) -> Self {
		Self { connection }
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