use crate::delay_line::{DelayLine, Interpolation};
use crate::mix::Mix;
use crate::one_pole_filter::Mode;
use crate::one_pole_filter::OnePoleFilter;
use crate::phasor::Phasor;

pub struct Reverse {
  delay_line: DelayLine,
  phasor: Phasor,
  smooth_time: OnePoleFilter,
  smooth_feedback: OnePoleFilter,
  smooth_mix: OnePoleFilter,
}

impl Reverse {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 5.02) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
      smooth_time: OnePoleFilter::new(sample_rate),
      smooth_feedback: OnePoleFilter::new(sample_rate),
      smooth_mix: OnePoleFilter::new(sample_rate),
    }
  }

  fn read_delay_line(&mut self, phasor: f32, time: f32, gain: f32) -> f32 {
    if gain == 0. {
      0.
    } else {
      self.delay_line.read(phasor * time, Interpolation::Linear) * gain
    }
  }

  fn reverse_delay(&mut self, time: f32) -> f32 {
    let freq = 1000. / time;
    let phasor_a = self.phasor.run(freq) * 2.;
    let phasor_b = (phasor_a + 1.) % 2.;

    let xfade_factor = time / 20.;
    let xfade_offset = 1. / xfade_factor + 1.;
    let ramp_up = (phasor_a * xfade_factor).min(1.);
    let ramp_down = ((xfade_offset - phasor_a) * xfade_factor).clamp(0., 1.);
    let xfade_a = ramp_up * ramp_down;
    let xfade_b = 1. - xfade_a;

    let reverse_delay_a = self.read_delay_line(phasor_a, time, xfade_a);
    let reverse_delay_b = self.read_delay_line(phasor_b, time, xfade_b);
    reverse_delay_a + reverse_delay_b
  }

  pub fn run(&mut self, input: f32, time: f32, feedback: f32, mix: f32) -> f32 {
    let time = self.smooth_time.run(time, 3., Mode::Hertz);
    let feedback = self.smooth_feedback.run(feedback, 12., Mode::Hertz);
    let mix = self.smooth_mix.run(mix, 12., Mode::Hertz);

    let reverse_delay = self.reverse_delay(time);
    let delay = self.delay_line.read(time, Interpolation::Linear);
    self.delay_line.write(input + delay * feedback);
    Mix::run(input, reverse_delay, mix)
  }
}
