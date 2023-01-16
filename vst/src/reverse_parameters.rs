use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct ReverseParameters {
  pub time: AtomicFloat,
  pub feedback: AtomicFloat,
  pub mix: AtomicFloat,
}

impl Default for ReverseParameters {
  fn default() -> Self {
    Self {
      time: AtomicFloat::new(250.),
      feedback: AtomicFloat::new(0.5),
      mix: AtomicFloat::new(0.5),
    }
  }
}

impl PluginParameters for ReverseParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => ((self.time.get() - 20.) / 4980.).powf(0.333333),
      1 => self.feedback.get(),
      2 => self.mix.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2} ms", self.time.get()),
      1 => format!("{:.2}%", self.feedback.get() * 100.0),
      2 => format!("{:.2}%", self.mix.get() * 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Time",
      1 => "Feedback",
      2 => "Mix",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.time.set(val.powf(3.) * 4980. + 20.),
      1 => self.feedback.set(val),
      2 => self.mix.set(val),
      _ => (),
    }
  }
}
