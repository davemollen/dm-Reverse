mod smooth;
pub use smooth::Smoother;
use smooth::{LinearSmooth, LogarithmicSmooth};

pub struct Params {
  pub time: LogarithmicSmooth,
  pub feedback: LinearSmooth,
  pub mix: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      time: LogarithmicSmooth::new(sample_rate, 0.25),
      feedback: LinearSmooth::new(sample_rate, 12.),
      mix: LinearSmooth::new(sample_rate, 12.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, time: f32, feedback: f32, mix: f32) {
    if self.is_initialized {
      self.time.set_target(time);
      self.feedback.set_target(feedback);
      self.mix.set_target(mix);
    } else {
      self.time.reset(time);
      self.feedback.reset(feedback);
      self.mix.reset(mix);
      self.is_initialized = true;
    }
  }
}
