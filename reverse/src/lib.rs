mod delay_line;
mod shared {
  pub mod float_ext;
}
mod mix;
mod params;
mod phasor;
pub use params::Params;
use {
  delay_line::{DelayLine, Interpolation},
  mix::Mix,
  params::Smoother,
  phasor::Phasor,
};

pub struct Reverse {
  delay_line: DelayLine,
  phasor: Phasor,
}

impl Reverse {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 5.02) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let time = params.time.next();
    let feedback = params.feedback.next();
    let mix = params.mix.next();

    let reverse_delay = self.reverse_delay(time);
    let delay = self.delay_line.read(time, Interpolation::Linear);
    self.delay_line.write(input + delay * feedback);

    Mix::process(input, reverse_delay, mix)
  }

  fn read_delay_line(&self, phasor: f32, time: f32, gain: f32) -> f32 {
    if gain == 0. {
      0.
    } else {
      self.delay_line.read(phasor * time, Interpolation::Linear) * gain
    }
  }

  fn reverse_delay(&mut self, time: f32) -> f32 {
    let freq = time.recip() * 1000.;
    let phasor_a = self.phasor.process(freq) * 2.;
    let phasor_b = Self::wrap(phasor_a + 1.);

    let xfade_factor = time / 20.;
    let xfade_offset = xfade_factor.recip() + 1.;
    let ramp_up = (phasor_a * xfade_factor).min(1.);
    let ramp_down = ((xfade_offset - phasor_a) * xfade_factor).clamp(0., 1.);
    let xfade_a = ramp_up * ramp_down;
    let xfade_b = 1. - xfade_a;

    let reverse_delay_a = self.read_delay_line(phasor_a, time, xfade_a);
    let reverse_delay_b = self.read_delay_line(phasor_b, time, xfade_b);
    reverse_delay_a + reverse_delay_b
  }

  fn wrap(x: f32) -> f32 {
    if x >= 2. {
      x - 2.
    } else {
      x
    }
  }
}
