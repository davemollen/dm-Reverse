mod delay_line;
mod shared {
  pub mod float_ext;
}
mod mix;
mod param_filter;
mod phasor;
use {
  delay_line::{DelayLine, Interpolation},
  mix::Mix,
  param_filter::ParamFilter,
  phasor::Phasor,
};

pub struct Reverse {
  delay_line: DelayLine,
  phasor: Phasor,
  smooth_time: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_mix: ParamFilter,
}

impl Reverse {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 5.02) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
      smooth_time: ParamFilter::new(sample_rate, 3.),
      smooth_feedback: ParamFilter::new(sample_rate, 12.),
      smooth_mix: ParamFilter::new(sample_rate, 12.),
    }
  }

  pub fn initialize_params(&mut self, time: f32, feedback: f32, mix: f32) {
    self.smooth_time.initialize(time);
    self.smooth_feedback.initialize(feedback);
    self.smooth_mix.initialize(mix);
  }

  pub fn process(&mut self, input: f32, time: f32, feedback: f32, mix: f32) -> f32 {
    let time = self.smooth_time.process(time);
    let feedback = self.smooth_feedback.process(feedback);
    let mix = self.smooth_mix.process(mix);

    let reverse_delay = self.reverse_delay(time);
    let delay = self.delay_line.read(time, Interpolation::Linear);
    self.delay_line.write(input + delay * feedback);

    Mix::process(input, reverse_delay, mix)
  }

  fn read_delay_line(&mut self, phasor: f32, time: f32, gain: f32) -> f32 {
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
