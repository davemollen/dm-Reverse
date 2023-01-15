use super::clip::Clip;
use super::delay_line::DelayLine;
use super::lowpass::Lowpass;
use super::mix::Mix;
use super::phasor::Phasor;

pub struct Reverse {
  delay_line: DelayLine,
  phasor: Phasor,
  lowpass: Lowpass,
}

impl Reverse {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 5.02) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
      lowpass: Lowpass::new(sample_rate),
    }
  }

  fn read_delay_line(&mut self, phasor: f32, time: f32, gain: f32) -> f32 {
    if gain == 0. {
      0.
    } else {
      self.delay_line.read(phasor * time, "linear") * gain
    }
  }

  fn reverse_delay(&mut self, time: f32) -> f32 {
    let freq = 1000. / time;
    let phasor_a = self.phasor.run(freq) * 2.;
    let phasor_b = (phasor_a + 1.) % 2.;

    let xfade_factor = time / 20.;
    let xfade_offset = 1. / xfade_factor + 1.;
    let ramp_up = Clip::run(phasor_a * xfade_factor, 0., 1.);
    let ramp_down = Clip::run((xfade_offset - phasor_a) * xfade_factor, 0., 1.);
    let xfade_a = ramp_up * ramp_down;
    let xfade_b = 1. - xfade_a;

    let reverse_delay_a = self.read_delay_line(phasor_a, time, xfade_a);
    let reverse_delay_b = self.read_delay_line(phasor_b, time, xfade_b);
    reverse_delay_a + reverse_delay_b
  }

  pub fn run(&mut self, input: f32, t: f32, feedback: f32, mix: f32) -> f32 {
    let time = self.lowpass.run(t, 1.);
    let reverse_delay = self.reverse_delay(time);
    let delay = self.delay_line.read(time, "linear");
    self.delay_line.write(input + delay * feedback);
    Mix::run(input, reverse_delay, mix)
  }
}
