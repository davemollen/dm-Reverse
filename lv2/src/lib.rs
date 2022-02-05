extern crate lv2;
extern crate reverse;
use lv2::prelude::*;
use reverse::Reverse;

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
}

impl Plugin for DmReverse {
    // Tell the framework which ports this plugin has.
    type Ports = Ports;

    // We don't need any special host features; We can leave them out.
    type InitFeatures = ();
    type AudioFeatures = ();

    // Create a new instance of the plugin; Trivial in this case.
    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            reverse: Reverse::new(_plugin_info.sample_rate()),
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let time = *ports.time;
        let feedback = *ports.feedback * 0.01;
        let mix = *ports.mix * 0.01;

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = self.reverse.run(*in_frame, time, feedback, mix);
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmReverse);
