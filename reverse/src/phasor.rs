pub struct Phasor {
  sample_rate: f64,
  x: f32,
}

impl Phasor {
  pub fn new(sample_rate: f64) -> Self {
    Self { sample_rate, x: 0. }
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

  pub fn run(&mut self, freq: f32) -> f32 {
    let multiplier = 1. / self.sample_rate as f32;
    self.x = self.wrap(self.x + freq * multiplier);
    self.x
  }
}

#[cfg(test)]
mod phasor {
  use super::*;

  fn approx_equal(a: f32, b: f32, decimal_places: u8) {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    assert_eq!(a, b);
  }

  #[test]
  fn wrap_test() {
    let mut phasor = Phasor::new(10.);
    approx_equal(phasor.wrap(1.2), 0.2, 3);
  }

  #[test]
  fn test() {
    let mut phasor = Phasor::new(10.);

    for i in 1..32 {
      approx_equal(phasor.run(1.), (i % 10) as f32 * 0.1, 1);
    }
  }
}
