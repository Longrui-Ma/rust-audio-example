//! # Test script 
//! This is a simple script to test and demonstrate how to play with `mydsp-jack`.
//! ## How to use:
//! * In a (new) binary crate (e.g. use `cargo new [crate-name]` to create a new binary crate),
//! * rename this file (demo.rs) to main.rs in /src,
//! * and don't forget to add `mydsp_jack` in `Cargo.toml`.
//! * If you are using `VScode`, extension `rust-analyzer` is recommended.
//! ## QUICK DEMO
//! Change lines that have word `DEMO` in the comments to experiment with different effects.
use jack::{AudioIn, AudioOut, Client};
// The following `use` can be combined, but for clarity in this demonstration, each module is called separately.
use mydsp_jack::sine::SineWave;
use mydsp_jack::echo::Echo;
use mydsp_jack::pwm::Pwm;
use mydsp_jack::noise::WhiteNoise;
use mydsp_jack::smooth::Smooth;
use mydsp_jack::one_zero::OneZero;
use mydsp_jack::distortion::Distortion;
use mydsp_jack::am::Am;
use mydsp_jack::fm::Fm;
use mydsp_jack::flanger::Flanger;
use mydsp_jack::ks::KS;
use mydsp_jack::config::Config;

fn main() -> Result<(), jack::Error> {
    // open a jack client, and if server is off, this will also starts the server.
    let (client, _status) = Client::new("echo_sine", jack::ClientOptions::empty())?;
    // use default setup for input port(s) and output port(s)
    let in_spec = AudioIn::default();
    let out_spec = AudioOut::default();
    // DEMO: choose 2 lines among 6 (3 pairs input-output ports or 2 or 1), comment the rest.
    let input_ports = vec![client.register_port("input1", in_spec)?, client.register_port("input2", in_spec)?, client.register_port("input3", in_spec)?];
    let output_ports = vec![client.register_port("output1", out_spec)?, client.register_port("output2", out_spec)?, client.register_port("output3", out_spec)?];
    // let input_ports = vec![client.register_port("input1", in_spec)?, client.register_port("input2", in_spec)?];
    // let output_ports = vec![client.register_port("output1", out_spec)?, client.register_port("output2", out_spec)?];
    // let input_ports = vec![client.register_port("input1", in_spec)?];
    // let output_ports = vec![client.register_port("output1", out_spec)?];
    
    let sample_rate = client.sample_rate();
    // define parameters for app instances.
    let nframes_delay = sample_rate / 2;
    let feedback = 0.6;
    let port_pairs = input_ports.len();
    println!("port_pairs: {}", port_pairs);
    let size: usize = 4096;
    let frequency:f32 = 440.0;
    let gain:f32 = 0.5;

    // create app instance (trait object).
    // DEMO: to combine different effects, uncomment at most 1 line among different indent lines
    let app = 
        Box::new(Echo::new(nframes_delay, feedback, port_pairs,
            Box::new(Echo::new(nframes_delay*2, 0.3, port_pairs,
                Box::new(Smooth::new(0.9, //port_pairs,
                // Box::new(OneZero::new(0.2,
                    Box::new(Pwm::new(0.1, 4*sample_rate,
                        Box::new(Distortion::new(0.1, 0.2, 1.0,
                        // TODO: Flanger and KS probably have bug (or should not take more input like sinewave).
                        // Box::new(Flanger::new(sample_rate, 16384, 1000.0, gain, 0.9, 1.0, nframes_delay, feedback, port_pairs,
                        // Box::new(KS::new(sample_rate, 0.999, frequency, 0.5, port_pairs,
                            Box::new(Am::new(sample_rate, 16384, 1000.0, 2.0, 0.5, gain))
                            // Box::new(Fm::new(sample_rate, 16384, 440.0, 100.0, 0.8, gain))
                            // Box::new(SineWave::new(sample_rate, size, None, frequency, gain))
                            // Box::new(WhiteNoise::new(gain))
                        ))
                        // ))
                    ))
                ))
            ))
        )
    );
    // create `config` instance (a mydsp-jack module)
    let config = Config::new(input_ports, output_ports, app);
    // begin processing in real time.
    let active_client = client.activate_async((), config)?;
    // timer for this jack thread.
    std::thread::sleep(std::time::Duration::from_secs(200));
    // end this jack thread.
    active_client.deactivate()?;
    // send a signal.
    Ok(())
}
