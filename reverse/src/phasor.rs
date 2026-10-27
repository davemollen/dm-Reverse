pub struct Phasor {
  sample_rate: f32,
  x: f32,
}

impl Phasor {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, x: 0. }
  }

  pub fn reset(&mut self) {
    self.x = 0.;
  }

  pub fn process(&mut self, freq: f32) -> f32 {
    let multiplier = 1. / self.sample_rate as f32;
    self.x = self.wrap(self.x + freq * multiplier);
    self.x
  }

  fn wrap(&mut self, input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else if input <= 0. {
      input + 1.
    } else {
      input
    }
  }
}

#[cfg(test)]
mod phasor {
  use super::Phasor;
  use crate::assert_approximately_eq;

  #[test]
  fn wrap_test() {
    let mut phasor = Phasor::new(10.);
    assert_approximately_eq!(phasor.wrap(1.2), 0.2, 3);
  }

  #[test]
  fn test() {
    let mut phasor = Phasor::new(10.);

    for i in 1..32 {
      assert_approximately_eq!(phasor.process(1.), (i % 10) as f32 * 0.1, 1);
    }
  }
}
