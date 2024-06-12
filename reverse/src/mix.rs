use std::f32::consts::FRAC_PI_2;

use crate::shared::float_ext::FloatExt;

pub struct Mix;

impl Mix {
  pub fn process(dry: f32, wet: f32, mix: f32) -> f32 {
    let dry_gain = (mix * FRAC_PI_2).fast_cos();
    let wet_gain = (mix * FRAC_PI_2).fast_sin();
    let dry_out = dry * dry_gain;
    dry_out * dry_gain + wet * wet_gain
  }
}

#[cfg(test)]
mod tests {
  use crate::mix::Mix;

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!((left * 100.).round() / 100., (right * 100.).round() / 100.)
  }

  #[test]
  fn mix() {
    assert_approximately_eq(Mix::process(0., 1., 0.), 0.);
    assert_approximately_eq(Mix::process(0., 1., 0.5), 0.707);
    assert_approximately_eq(Mix::process(0., 1., 1.), 1.);
  }
}
