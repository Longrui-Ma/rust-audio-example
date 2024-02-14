//! Test script (binary crate).
//! 
//! in `lib.rs` (library crate):
//! ```
//! pub mod import{
//!     pub(crate) use jack::{AudioIn, AudioOut, Client, Control, ProcessHandler, ProcessScope};
//! }
//! #[doc(alias = "sinetable")]
//! #[doc(alias = "table")]
//! pub mod sine_table;
//! #[doc(alias = "sinewave")]
//! pub mod sine;
//! ```
use jack::{AudioOut, Client};
use mydsp_jack::sine::SineWave; // lib crate: mydsp

fn main() -> Result<(), jack::Error> {
    let (client, _status) = Client::new("sine_wave", jack::ClientOptions::empty())?;
    let spec = AudioOut::default();
    let output_ports = vec![client.register_port("output1", spec)?, client.register_port("output2", spec)?];
    let sample_rate = client.sample_rate() as f32;

    let size: usize = 4096;
    let frequency:f32 = 1000.0;
    let gain:f32 = 1.0;
    let sine_wave = SineWave::new(output_ports, sample_rate, size, None, frequency, gain);

    let active_client = client.activate_async((), sine_wave)?;
    std::thread::sleep(std::time::Duration::from_secs(20));
    active_client.deactivate()?;
    Ok(())
}
