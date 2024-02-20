//! Test script (binary crate).
//! 
//! in `lib.rs` (library crate):
//! ```
//! pub mod import{
//!     pub(crate) use jack::{AudioIn, AudioOut, Client, Control, ProcessHandler, ProcessScope};
//! }
//! pub mod config;
//! #[doc(alias = "sinetable")]
//! #[doc(alias = "table")]
//! pub mod sine_table;
//! #[doc(alias = "phase")]
//! pub mod phasor;
//! #[doc(alias = "sinewave")]
//! pub mod sine;
//! ```
use jack::{AudioOut, Client};
use mydsp_jack::sine::SineWave; // lib crate: mydsp
use mydsp_jack::config::Config;

fn main() -> Result<(), jack::Error> {
    let (client, _status) = Client::new("sine_wave", jack::ClientOptions::empty())?;

    let input_ports = vec![];
    let spec = AudioOut::default();
    // let output_ports = vec![client.register_port("output1", spec)?, client.register_port("output2", spec)?];
    let output_ports = vec![client.register_port("output1", spec)?];
    
    let sample_rate = client.sample_rate() as f32;
    // create app instance (trait object)
    let size: usize = 4096;
    let frequency:f32 = 440.0;
    let gain:f32 = 1.0;
    let app = Box::new(SineWave::new(sample_rate, size, None, frequency, gain));
    // create config instance
    let config = Config::new(input_ports, output_ports, app);

    let active_client = client.activate_async((), config)?;
    std::thread::sleep(std::time::Duration::from_secs(20));
    active_client.deactivate()?;
    Ok(())
}
