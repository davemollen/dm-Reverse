#[macro_use]
extern crate vst;
mod reverse_parameters;
use reverse::{Params as ProcessParams, Reverse};
use reverse_parameters::ReverseParameters;
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
};

struct DmReverse {
  params: Arc<ReverseParameters>,
  reverse: Reverse,
  process_params: ProcessParams,
}

impl Default for DmReverse {
  fn default() -> Self {
    Self {
      params: Arc::new(ReverseParameters::default()),
      reverse: Reverse::new(44100.),
      process_params: ProcessParams::new(44100.),
    }
  }
}

impl Plugin for DmReverse {
  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.reverse = Reverse::new(sample_rate);
    self.process_params = ProcessParams::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-Reverse".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 1,
      outputs: 1,
      parameters: 3,
      unique_id: 1357,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    self.process_params.set(
      self.params.time.get(),
      self.params.feedback.get(),
      self.params.mix.get(),
    );

    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = self
          .reverse
          .process(*input_sample, &mut self.process_params);
      }
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmReverse);
