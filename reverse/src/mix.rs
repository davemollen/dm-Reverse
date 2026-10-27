use crate::shared::float_ext::FloatExt;
use std::f32::consts::FRAC_PI_2;

pub struct Mix {
  mix: f32,
  dry_gain: f32,
  wet_gain: f32,
}

impl Mix {
  pub fn new() -> Self {
    Self {
      mix: 0.,
      dry_gain: 1.,
      wet_gain: 0.,
    }
  }

  pub fn process(&mut self, dry: f32, wet: f32, mix: f32) -> f32 {
    if mix != self.mix {
      let factor = mix * FRAC_PI_2;
      self.mix = mix;
      self.dry_gain = factor.fast_cos();
      self.wet_gain = factor.fast_sin();
    }
    dry * self.dry_gain + wet * self.wet_gain
  }
}

#[cfg(test)]
mod tests {
  use super::Mix;
  use crate::assert_approximately_eq;

  #[test]
  fn mix() {
    let mut mix = Mix::new();
    assert_approximately_eq!(mix.process(0., 1., 0.), 0., 3);
    assert_approximately_eq!(mix.process(0., 1., 0.5), 0.707, 3);
    assert_approximately_eq!(mix.process(0., 1., 1.), 1., 3);
  }
}
