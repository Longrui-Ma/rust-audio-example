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
//! pub mod echo;
//! pub mod pwm;
//! ```
use jack::{AudioIn, AudioOut, Client};
use mydsp_jack::sine::SineWave; // lib crate: mydsp
use mydsp_jack::echo::Echo;
use mydsp_jack::pwm::Pwm;
use mydsp_jack::config::Config;

fn main() -> Result<(), jack::Error> {
    let (client, _status) = Client::new("echo_sine", jack::ClientOptions::empty())?;

    let in_spec = AudioIn::default();
    let out_spec = AudioOut::default();
    let input_ports = vec![client.register_port("input1", in_spec)?, client.register_port("input2", in_spec)?, client.register_port("input3", in_spec)?];
    let output_ports = vec![client.register_port("output1", out_spec)?, client.register_port("output2", out_spec)?, client.register_port("output3", out_spec)?];
    // let input_ports = vec![client.register_port("input1", in_spec)?, client.register_port("input2", in_spec)?];
    // let output_ports = vec![client.register_port("output1", out_spec)?, client.register_port("output2", out_spec)?];
    // let input_ports = vec![client.register_port("input1", in_spec)?];
    // let output_ports = vec![client.register_port("output1", out_spec)?];
    
    let sample_rate = client.sample_rate();
    // create app instance (trait object)
    let nframes_delay = sample_rate / 2;
    let feedback = 0.6;
    let port_pairs = input_ports.len();
    println!("port_pairs: {}", port_pairs);
    let size: usize = 4096;
    let frequency:f32 = 440.0;
    let gain:f32 = 0.5;
    let app = 
        Box::new(Echo::new(nframes_delay, feedback, port_pairs,
            // Box::new(Echo::new(nframes_delay*2, 0.3, port_pairs,
                Box::new(Pwm::new(0.1, 4*sample_rate,
                            Box::new(SineWave::new(sample_rate, size, None, frequency, gain))
            // ))
            ))
        )
    );
    // create config instance
    let config = Config::new(input_ports, output_ports, app);

    let active_client = client.activate_async((), config)?;
    std::thread::sleep(std::time::Duration::from_secs(200));
    active_client.deactivate()?;
    Ok(())
}
