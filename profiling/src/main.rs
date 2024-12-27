use reverse::{Params, Reverse};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut reverse = Reverse::new(44100.);
  let mut params = Params::new(44100.);
  params.set(250., 0.5, 0.5);

  loop {
    let input = generate_signal();
    reverse.process(input, &mut params);
  }
}
