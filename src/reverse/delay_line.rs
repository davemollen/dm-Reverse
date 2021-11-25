pub struct DelayLine {
  buffer: Vec<f32>,
  write_pointer: usize,
  sample_rate: f64,
}

impl DelayLine {
  pub fn new(length: usize, sample_rate: f64) -> Self {
    Self {
      buffer: vec![0.0; length],
      write_pointer: 0,
      sample_rate,
    }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate as f32
  }

  fn wrap(&self, index: usize) -> usize {
    index % self.buffer.len()
  }

  fn step_interp(&self, index: usize) -> f32 {
    self.buffer[self.wrap(index)]
  }

  fn linear_interp(&self, index: usize, mix: f32) -> f32 {
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    x * (1. - mix) + y * mix
  }

  pub fn read(&mut self, time: f32, interp: &str) -> f32 {
    let read_pointer = self.write_pointer as f32 - self.mstosamps(time) + self.buffer.len() as f32;
    let floored_read_pointer = read_pointer.floor();
    let mix = read_pointer - floored_read_pointer;
    let mut index = floored_read_pointer as usize;
    if index == self.write_pointer {
      index -= 1;
    }

    match interp {
      "step" => self.step_interp(index),
      "linear" => self.linear_interp(index, mix),
      _ => self.step_interp(index),
    }
  }

  pub fn write(&mut self, value: f32) {
    self.buffer[self.write_pointer] = value;
    self.write_pointer = self.wrap(self.write_pointer + 1);
  }
}
