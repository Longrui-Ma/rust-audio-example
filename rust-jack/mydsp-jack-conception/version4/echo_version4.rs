// 1 echo in lib enough, encapsulate multiple echo.tick() for more echo
// TODO: 
// * verify `feedback` < 1
use crate::config::AppTrait;

// #[derive(Debug)]
pub struct Echo {
    buffer_echo: Vec<f32>, // circular buffer to store delayed frames (from input).
    buffer_index: usize, // buffer's read and write position.
    nframes_delay: usize, // nframes_delay / sample_rate = time of delay.
    feedback: f32, // echo volumn decay rate [0,1).
    in_app: Box<dyn AppTrait>,
}

impl Echo{
    pub fn new(nframes_delay: usize, feedback: f32, in_app: Box<dyn AppTrait>) -> Self {
        Echo {
            buffer_echo: vec![0.0; nframes_delay], // buffer size = nframes_delay.
            buffer_index: 0, // starting point
            nframes_delay, // e.g. nframes_delay = sample_rate / 2;  // delay of 0.5s.
            feedback,
            in_app,
        }
    }
}

impl AppTrait for Echo {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let in_frame = in_frame + self.in_app.tick(in_frame); // combine in_frame (e.g. mic input) with app.tick()
        let delayed_frame = self.buffer_echo[self.buffer_index]; // read delayed frame.
        // println!("{}, {}, {}, {}", self.nframes_delay, self.buffer_index, in_frame, delayed_frame);
        self.buffer_echo[self.buffer_index] = in_frame; // write, copy new frame from input.
        self.buffer_index = (self.buffer_index + 1) % self.nframes_delay; // update write position.
        in_frame + delayed_frame * self.feedback
    }
    fn save_init(&mut self) {
    }
    fn load_init(&mut self) {
    }
}
