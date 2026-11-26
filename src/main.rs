mod pen_plotter;

use pen_plotter::{PenConnection, Result};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> Result<()> {
    let mut plotter = PenConnection::connect()?;
    println!("connected: {}", plotter.version()?);

    plotter.send("EM,1,1")?; // enable both motors, 1/16 microstepping
    plotter.send("SP,0")?; // pen down

    // SM,<duration ms>,<axis1 steps>,<axis2 steps>
    // Equal steps on both axes moves in a straight line along one direction.
    plotter.send("SM,1000,4000,4000")?;

    plotter.send("SP,1")?; // pen up
    plotter.send("EM,0,0")?; // disable motors
    println!("done");
    Ok(())
}
