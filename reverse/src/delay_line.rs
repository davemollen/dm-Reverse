pub struct DelayLine {
  buffer: Vec<f32>,
  write_pointer: usize,
  sample_rate: f32,
}

impl DelayLine {
  pub fn new(length: usize, sample_rate: f32) -> Self {
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
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
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
    let read_pointer = (self.write_pointer - 1 + self.buffer.len()) as f32 - self.mstosamps(time);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

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
