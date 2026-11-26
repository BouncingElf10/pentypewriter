use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::time::Duration;

use serialport::{SerialPort, SerialPortType};

const EBB_VID: u16 = 0x04D8;
const EBB_PID: u16 = 0xFD92;
const TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub enum PlotterError {
    NotFound,
    Serial(serialport::Error),
    Io(io::Error),
    Device(String),
    NoReply(String),
}

impl fmt::Display for PlotterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "no EBB board found; is the USB cable plugged in?"),
            Self::Serial(e) => write!(f, "serial error: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Device(m) => write!(f, "board error: {m}"),
            Self::NoReply(c) => write!(f, "no reply to `{c}`"),
        }
    }
}

impl std::error::Error for PlotterError {}

impl From<serialport::Error> for PlotterError {
    fn from(e: serialport::Error) -> Self {
        Self::Serial(e)
    }
}

impl From<io::Error> for PlotterError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, PlotterError>;
pub fn find_ports() -> Result<Vec<String>> {
    Ok(serialport::available_ports()?
        .into_iter()
        .filter(|p| matches!(&p.port_type,
            SerialPortType::UsbPort(u) if u.vid == EBB_VID && u.pid == EBB_PID))
        .map(|p| p.port_name)
        .collect())
}

pub struct PenConnection<T: Read + Write = Box<dyn SerialPort>> {
    link: BufReader<T>,
}

impl PenConnection {
    pub fn connect() -> Result<Self> {
        let name = find_ports()?.into_iter().next().ok_or(PlotterError::NotFound)?;
        Self::open(&name)
    }

    pub fn open(port: &str) -> Result<Self> {
        let port = serialport::new(port, 9600).timeout(TIMEOUT).open()?;
        Ok(Self::from_stream(port))
    }
}

impl<T: Read + Write> PenConnection<T> {
    pub fn from_stream(stream: T) -> Self {
        Self { link: BufReader::new(stream) }
    }

    pub fn send(&mut self, cmd: &str) -> Result<Vec<String>> {
        let out = self.link.get_mut();
        out.write_all(cmd.as_bytes())?;
        out.write_all(b"\r")?;
        out.flush()?;

        let mut data = Vec::new();
        loop {
            let mut line = String::new();
            if self.link.read_line(&mut line)? == 0 {
                return Err(PlotterError::NoReply(cmd.to_string()));
            }
            let line = line.trim();
            if line == "OK" {
                return Ok(data);
            } else if let Some(msg) = line.strip_prefix('!') {
                return Err(PlotterError::Device(msg.trim().to_string()));
            } else if !line.is_empty() {
                data.push(line.to_string());
            }
        }
    }

    pub fn version(&mut self) -> Result<String> {
        let out = self.link.get_mut();
        out.write_all(b"V\r")?;
        out.flush()?;
        let mut line = String::new();
        if self.link.read_line(&mut line)? == 0 {
            return Err(PlotterError::NoReply("V".into()));
        }
        Ok(line.trim().to_string())
    }
}