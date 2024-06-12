use reverse::Reverse;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut reverse = Reverse::new(44100.);

  loop {
    let input = generate_signal();
    reverse.process(input, 250., 0.5, 0.5);
  }
}
