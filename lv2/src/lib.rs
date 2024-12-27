extern crate lv2;
extern crate reverse;
use lv2::prelude::*;
use reverse::{Params, Reverse};

#[derive(PortCollection)]
struct Ports {
  time: InputPort<Control>,
  feedback: InputPort<Control>,
  mix: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Reverse")]
struct DmReverse {
  reverse: Reverse,
  params: Params,
}

impl Plugin for DmReverse {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      reverse: Reverse::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self
      .params
      .set(*ports.time, *ports.feedback * 0.01, *ports.mix * 0.01);

    for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
      *out_frame = self.reverse.process(*in_frame, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmReverse);
