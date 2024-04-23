use std::f32;

pub struct Mix;

impl Mix {
  pub fn process(dry: f32, wet: f32, mix: f32) -> f32 {
    let twopi = f32::consts::PI * 2.;
    let phase = mix * 0.25;
    let dry_gain = (phase * twopi).cos();
    let wet_gain = ((phase + 0.75) * twopi).cos();
    dry * dry_gain + wet * wet_gain
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn approx_equal(a: f32, b: f32, decimal_places: u8) {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    assert_eq!(a, b);
  }

  #[test]
  fn mix() {
    approx_equal(Mix::process(0., 1., 0.), 0., 3);
    approx_equal(Mix::process(0., 1., 0.5), 0.707, 3);
    approx_equal(Mix::process(0., 1., 1.), 1., 3);
  }
}
