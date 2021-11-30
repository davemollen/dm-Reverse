#[macro_use]
extern crate vst;

use reverse::Reverse;
use std::sync::Arc;
use vst::api::TimeInfo;
use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::{HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

struct DmReverse {
    params: Arc<ReverseParameters>,
    reverse: Reverse,
}

struct ReverseParameters {
    time: AtomicFloat,
    feedback: AtomicFloat,
    mix: AtomicFloat,
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

impl Default for DmReverse {
    fn default() -> Self {
        Self {
            params: Arc::new(ReverseParameters::default()),
            reverse: Reverse::new(44100.),
        }
    }
}

impl Plugin for DmReverse {
    fn new(host: HostCallback) -> Self {
        fn get_sample_rate(info: TimeInfo) -> f64 {
            info.sample_rate
        }
        let sample_rate = host.get_time_info(0).map(get_sample_rate).unwrap();
        Self {
            params: Arc::new(ReverseParameters::default()),
            reverse: Reverse::new(sample_rate),
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.reverse = Reverse::new(f64::from(sample_rate));
    }

    fn get_info(&self) -> Info {
        Info {
            name: "dm-Reverse".to_string(),
            inputs: 1,
            outputs: 1,
            parameters: 3,
            unique_id: 1357,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let time = self.params.time.get();
        let feedback = self.params.feedback.get();
        let mix = self.params.mix.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = self.reverse.run(*input_sample, time, feedback, mix);
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
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

plugin_main!(DmReverse);
